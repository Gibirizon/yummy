use crate::Error;
use crate::{Memory, MEMORY_MANAGER};
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{storable::Bound, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;

pub mod images;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RecipeInfo {
    pub instructions: Vec<String>,
    pub ingredients: Vec<String>,
    pub cuisines: Option<Vec<String>>,
    pub tags: Vec<String>,
    pub total_time_in_seconds: u16,
    pub popular: bool,
    pub author: Option<String>,
}

impl RecipeInfo {
    pub fn new(
        instructions: Vec<String>,
        ingredients: Vec<String>,
        tags: Vec<String>,
        total_time_in_seconds: u16,
        author: Option<String>,
    ) -> Self {
        Self {
            instructions,
            ingredients,
            cuisines: None,
            tags,
            total_time_in_seconds,
            popular: false,
            author,
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RecipeBrief {
    name: String,
    tags: Vec<String>,
    total_time: u16,
    author: Option<String>,
}

impl RecipeBrief {
    pub fn new(name: String, tags: Vec<String>, total_time: u16, author: Option<String>) -> Self {
        Self {
            name,
            tags,
            total_time,
            author,
        }
    }
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
pub fn create_recipe(name: String, recipe: RecipeInfo) {
    RECIPES.with(|recipes| recipes.borrow_mut().insert(name, recipe));
}

#[query]
pub fn recipe_exists(name: String) -> bool {
    RECIPES.with(|recipes| recipes.borrow().contains_key(&name))
}

#[query]
pub fn take_recipe(name: String) -> Result<RecipeInfo, Error> {
    RECIPES.with(|recipes| match recipes.borrow().get(&name) {
        Some(recipe) => Ok(recipe),
        None => Err(Error::RecipeNotFound {
            msg: "Recipe not found".to_string(),
        }),
    })
}

// for testing
#[query]
fn get_recipes_len() -> u64 {
    RECIPES.with(|recipes| recipes.borrow().len() as u64)
}

#[query]
pub fn take_recipes_names() -> Vec<String> {
    RECIPES.with(|recipes| {
        recipes
            .borrow()
            .iter()
            .map(|(name, _)| name.clone())
            .collect()
    })
}

#[query]
pub fn take_recipes_of_specific_type(recipes_type: String) -> Vec<RecipeBrief> {
    RECIPES.with(|recipes| {
        const TAGS_MAX_LENGTH: usize = 5;
        let filtered_recipes = recipes
            .borrow()
            .iter()
            .filter(|(_, recipe)| {
                if recipes_type == "Popular" {
                    recipe.popular == true
                } else if recipes_type == "Users" {
                    recipe.author.is_some()
                } else {
                    recipe.tags.iter().any(|tag| tag == &recipes_type)
                }
            })
            .map(|(name, recipe)| {
                let first_tags = &recipe.tags[..TAGS_MAX_LENGTH.min(recipe.tags.len())];
                RecipeBrief::new(
                    name.clone(),
                    first_tags.to_vec(),
                    recipe.total_time_in_seconds / 60,
                    recipe.author,
                )
            })
            .collect();
        filtered_recipes
    })
}
