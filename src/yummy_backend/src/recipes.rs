use crate::recipes::images::fetch_image;
use crate::recipes::key::SUGGESTIC_API_KEY;
use crate::user::USERS;
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

pub mod images;
mod key;

#[derive(CandidType, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecipeInside {
    name: String,
    main_image: String,
    instructions: Vec<String>,
    ingredients: Vec<Ingredient>,
    cuisines: Option<Vec<String>>,
    tags: Vec<String>,
    total_time_in_seconds: u16,
}
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RecipeSingleItem {
    node: RecipeInside,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Ingredient {
    name: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RecipeInfo {
    instructions: Vec<String>,
    ingredients: Vec<String>,
    cuisines: Option<Vec<String>>,
    tags: Vec<String>,
    total_time_in_seconds: u16,
    popular: bool,
}
impl RecipeInfo {
    pub fn new(
        instructions: Vec<String>,
        ingredients: Vec<String>,
        tags: Vec<String>,
        total_time_in_seconds: u16,
    ) -> Self {
        Self {
            instructions,
            ingredients,
            cuisines: None,
            tags,
            total_time_in_seconds,
            popular: false,
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RecipeBrief {
    name: String,
    tags: Vec<String>,
    total_time: u16,
}

thread_local! {
    pub static RECIPES: RefCell<StableBTreeMap<String, RecipeInfo , Memory>> = RefCell::new(
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
        max_size: 5 * 1024,
        is_fixed_size: false,
    };
}

#[update]
pub async fn recipes_initialization(query: String, recipes_type: String) {
    // ic_cdk::api::print(format!("New Query: {}", query));
    let res = fetch_recipes(query).await;
    transform_and_store_response(res, recipes_type.as_str()).await;
}

pub async fn fetch_recipes(query: String) -> String {
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
        max_response_bytes: Some(30 * 1024),
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

            ic_cdk::api::print(message.clone());
            message
        }
    }
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
        let image_fetching = fetch_image(&image_url, &recipe.node.name).await;
        match image_fetching {
            Ok(_) => {
                let indgredients = recipe
                    .node
                    .ingredients
                    .iter()
                    .map(|ingredient| ingredient.name.clone())
                    .collect();
                let new_recipe = RecipeInfo {
                    instructions: recipe.node.instructions,
                    cuisines: recipe.node.cuisines,
                    tags: recipe.node.tags,
                    total_time_in_seconds: recipe.node.total_time_in_seconds,
                    ingredients: indgredients,
                    popular: if recipes_type == "popularRecipes" {
                        true
                    } else {
                        false
                    },
                };

                RECIPES.with(|m| m.borrow_mut().insert(recipe.node.name, new_recipe));
            }
            Err(_) => {
                ic_cdk::api::print(format!("Failed to fetch image"));
                continue;
            }
        }
    }
}

#[query]
pub fn recipe_exists(name: String) -> bool {
    let main_recipes = RECIPES.with(|recipes| recipes.borrow().contains_key(&name));
    let user_recipes = USERS.with(|users| {
        users
            .borrow()
            .iter()
            .any(|(_, user)| user.recipes.contains_key(&name))
    });

    main_recipes || user_recipes
}

#[query]
pub fn get_recipe(name: String) -> Result<RecipeInfo, Error> {
    RECIPES.with(|recipes| {
        let recipe = match recipes.borrow().get(&name) {
            Some(recipe) => Ok(recipe),
            None => Err(Error::RecipeNotFound {
                msg: "Recipe not found".to_string(),
            }),
        };
        recipe
    })
}
#[query]
pub fn get_recipes_len() -> u64 {
    RECIPES.with(|recipes| recipes.borrow().len() as u64)
}

#[query]
pub fn get_recipes_names() -> Result<Vec<String>, Error> {
    RECIPES.with(|recipes| {
        let stable_btree_map = &*recipes.borrow();
        let names: Vec<String> = stable_btree_map
            .iter()
            .map(|(name, _)| name.clone())
            .collect();

        if names.is_empty() {
            return Err(Error::RecipesNotFound {
                msg: "No recipes".to_string(),
            });
        } else {
            Ok(names)
        }
    })
}

#[query]
pub fn take_recipes_of_specific_type(recipes_type: String) -> Result<Vec<RecipeBrief>, Error> {
    RECIPES.with(|recipes| {
        let stable_btree_map = &*recipes.borrow();
        let filtered_recipes: Vec<RecipeBrief>;
        const TAGS_MAX_LENGTH: usize = 5;
        filtered_recipes = stable_btree_map
            .iter()
            .filter(|(_, recipe)| {
                if recipes_type == "Popular" {
                    recipe.popular
                } else {
                    recipe.tags.iter().any(|tag| tag == &recipes_type)
                }
            })
            .map(|(name, recipe)| {
                let first_tags = &recipe.tags[..TAGS_MAX_LENGTH.min(recipe.tags.len())];
                let recipe_brief = RecipeBrief {
                    name: name.to_string(),
                    tags: first_tags.to_vec(),
                    total_time: recipe.total_time_in_seconds / 60,
                };
                recipe_brief
            })
            .collect();
        if filtered_recipes.is_empty() {
            return Err(Error::RecipesNotFound {
                msg: "No recipes".to_string(),
            });
        } else {
            Ok(filtered_recipes)
        }
    })
}
