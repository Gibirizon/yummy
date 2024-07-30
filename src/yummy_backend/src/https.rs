use crate::https::api_key::SUGGESTIC_API_KEY;
use crate::https::images::fetch_image;
use crate::Error;
use crate::{Memory, MEMORY_MANAGER};
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{storable::Bound, StableBTreeMap, Storable};
use serde_json::{from_str, Value};
use std::borrow::Cow;
use std::cell::RefCell;
use urlencoding::encode;

mod api_key;
pub mod images;
#[derive(CandidType, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecipeInside {
    name: String,
    main_image: String,
    cuisines: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    meal_tags: Option<Vec<String>>,
    total_time_in_seconds: Option<u64>,
    ingredients: Option<Vec<Ingredient>>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RecipeInfo {
    name: String,
    main_image: Option<String>,
    cuisines: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    meal_tags: Option<Vec<String>>,
    total_time_in_seconds: Option<u64>,
    ingredients: Option<Vec<Ingredient>>,
    image_bytes: Option<Vec<u8>>,
    popular: bool,
    owner: Option<u64>,
}
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Ingredient {
    name: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RecipeSingleItem {
    node: RecipeInside,
}

thread_local! {
    pub static RECIPES: RefCell<StableBTreeMap<u64, RecipeInfo , Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );

}

impl Storable for RecipeInfo {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 2 * 1024 * 1024 * 1024,
        is_fixed_size: false,
    };
}

pub async fn transform_and_store_response(http_response: String, recipes_type: &str) -> () {
    let json: Value = from_str(&http_response).unwrap();
    let data = json
        .get("data")
        .unwrap()
        .get(recipes_type)
        .unwrap()
        .get("edges")
        .unwrap();
    let recipes_list: Vec<RecipeSingleItem> = serde_json::from_value(data.clone()).unwrap();

    for recipe in recipes_list {
        let image_url = recipe.node.main_image;
        let image_base = fetch_image(&image_url).await;
        match image_base {
            Ok(image_bytes) => {
                let new_recipe = RecipeInfo {
                    name: recipe.node.name,
                    main_image: Some(image_url),
                    cuisines: recipe.node.cuisines,
                    tags: recipe.node.tags,
                    meal_tags: recipe.node.meal_tags,
                    total_time_in_seconds: recipe.node.total_time_in_seconds,
                    ingredients: recipe.node.ingredients,
                    image_bytes: Some(image_bytes),
                    popular: if recipes_type == "popularRecipes" {
                        true
                    } else {
                        false
                    },
                    owner: None,
                };

                //get the len of the map
                let storage_length = RECIPES.with(|m| m.borrow().len());
                RECIPES.with(|m| m.borrow_mut().insert(storage_length, new_recipe));
            }
            Err(_) => {
                continue;
            }
        }
    }
}

#[update]
pub async fn breakfast_recipes() {
    let tags = vec!["Breakfast".to_string(), "Dinner".to_string()];
    for tag in tags {
        let query = format!(
            r#"
    query {{
        recipesByTag (tag: "{}") {{
            edges {{
                node {{
                    cuisines
                    tags
                    totalTimeInSeconds
                    ingredients {{
                        name
                    }}
                    mainImage
                    mealTags
                    name
                }}
            }}
        }}
    }}
"#,
            tag
        );

        let res = get_recipes(query).await;
        transform_and_store_response(res, "recipesByTag").await;
    }
}

#[update]
pub async fn popular_recipes() {
    let query = r#"
    query {
        popularRecipes {
            edges {
                node {
                    cuisines
                    tags
                    totalTimeInSeconds
                    ingredients {
                        name
                    }
                    mainImage
                    mealTags
                    name
                }
            }
    }
  }
    "#
    .to_string();
    let res = get_recipes(query).await;
    transform_and_store_response(res, "popularRecipes").await;
}

pub async fn get_recipes(query: String) -> String {
    let encoded_query = encode(&query);

    let url = format!(
        "https://production.suggestic.com/graphql?query={}",
        encoded_query
    );
    let request_headers = vec![HttpHeader {
        name: "Authorization".to_string(),
        value: format!("Token {}", SUGGESTIC_API_KEY),
    }];

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    let cycles = 230_949_972_000;

    match http_request(request, cycles).await {
        Ok((response,)) => {
            let res = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");
            res
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            message
        }
    }
}

#[query]
pub async fn get_recipe(id: u64) -> Result<RecipeInfo, Error> {
    let recipe = RECIPES.with(|images| images.borrow().get(&id));
    match recipe {
        Some(recipe) => Ok(recipe),
        None => Err(Error::RecipeNotFound {
            msg: "Recipe not found".to_string(),
        }),
    }
}
#[query]
pub async fn get_recipes_len() -> u64 {
    RECIPES.with(|recipes| recipes.borrow().len() as u64)
}

#[query]
pub async fn get_recipes_names() -> Result<Vec<String>, Error> {
    RECIPES.with(|recipes| {
        let stable_btree_map = &*recipes.borrow();
        let records: Vec<String> = stable_btree_map
            .iter()
            .map(|(_, recipe)| recipe.name.clone())
            .collect();

        if records.is_empty() {
            return Err(Error::RecipesNotFound {
                msg: "No recipes".to_string(),
            });
        } else {
            Ok(records)
        }
    })
}
