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
pub struct RecipeStorage {
    name: String,
    main_image: Vec<u8>,
    cuisines: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    total_time_in_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RecipeInfo {
    node: RecipeInsideInfo,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecipeInsideInfo {
    name: String,
    main_image: String,
    cuisines: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    total_time_in_seconds: Option<u64>,
}

thread_local! {
    pub static POPULAR_RECIPES: RefCell<StableBTreeMap<u64, RecipeStorage , Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );
    pub static TAG_RECIPES: RefCell<StableBTreeMap<u64, RecipeStorage , Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        )
    );


}

impl Storable for RecipeStorage {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 100 * 1024 * 1024,
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
    let recipes_list: Vec<RecipeInfo> = serde_json::from_value(data.clone()).unwrap();

    ic_cdk::api::print(format!("JSON: {:#?}", recipes_list));

    for recipe in recipes_list {
        let image_url = recipe.node.main_image;
        let image_base = fetch_image(&image_url).await;
        let new_recipe = RecipeStorage {
            name: recipe.node.name,
            main_image: image_base,
            cuisines: recipe.node.cuisines,
            tags: recipe.node.tags,
            total_time_in_seconds: recipe.node.total_time_in_seconds,
        };

        // store recipe in correct storage
        if recipes_type == "popularRecipes" {
            //get the len of the map
            let storage_length = POPULAR_RECIPES.with(|m| m.borrow().len());
            POPULAR_RECIPES.with(|m| m.borrow_mut().insert(storage_length, new_recipe));
        } else if recipes_type == "recipesByTag" {
            let storage_length = TAG_RECIPES.with(|m| m.borrow().len());
            TAG_RECIPES.with(|m| m.borrow_mut().insert(storage_length, new_recipe));
        }
    }
}

#[update]
pub async fn breakfast_recipes() {
    let query = r#"
    query {
        recipesByTag (tag: "Breakfast") {
            edges {
                node {
                    cuisines
                    mainImage
                    tags
                    name
                    totalTimeInSeconds
                }
            }
        }
  }"#;
    let res = get_recipes(query).await;
    transform_and_store_response(res, "recipesByTag").await;
}

#[update]
pub async fn popular_recipes() {
    let query = r#"
    query {
        popularRecipes {
            edges {
                node {
                    name
                    mainImage
                    cuisines
                    tags
                    totalTimeInSeconds
                }
            }
        }
    }
    "#;
    let res = get_recipes(query).await;
    transform_and_store_response(res, "popularRecipes").await;
}

pub async fn get_recipes(query: &str) -> String {
    let encoded_query = encode(query);

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

pub fn get_names(
    btree_map: &StableBTreeMap<u64, RecipeStorage, Memory>,
) -> Result<Vec<String>, Error> {
    let records: Vec<String> = btree_map
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
}
#[query]
pub async fn get_popular_recipe(id: u64) -> RecipeStorage {
    POPULAR_RECIPES.with(|images| images.borrow().get(&id).clone().unwrap())
}
#[query]
pub async fn get_popular_recipes_len() -> u64 {
    POPULAR_RECIPES.with(|recipes| recipes.borrow().len() as u64)
}

#[query]
pub async fn get_popular_recipes_names() -> Result<Vec<String>, Error> {
    POPULAR_RECIPES.with(|recipes| {
        let stable_btree_map = &*recipes.borrow();
        get_names(stable_btree_map)
    })
}

#[query]
pub async fn get_tag_recipe(id: u64) -> RecipeStorage {
    TAG_RECIPES.with(|images| images.borrow().get(&id).clone().unwrap())
}
#[query]
pub async fn get_tag_recipes_len() -> u64 {
    TAG_RECIPES.with(|recipes| recipes.borrow().len() as u64)
}

#[query]
pub async fn get_tag_recipes_names() -> Result<Vec<String>, Error> {
    TAG_RECIPES.with(|recipes| {
        let stable_btree_map = &*recipes.borrow();
        get_names(stable_btree_map)
    })
}
