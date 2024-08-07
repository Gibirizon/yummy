use crate::user::get_username_by_index;
use crate::Error;
use crate::{Memory, MEMORY_MANAGER};
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{storable::Bound, StableBTreeMap, Storable};
use images::{add_image, IMAGES};
use std::borrow::Cow;
use std::cell::RefCell;

pub mod images;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RecipeInfo {
    pub instructions: Vec<String>,
    pub ingredients: Vec<String>,
    pub tags: Vec<String>,
    pub cuisines: Vec<String>,
    pub total_time_in_seconds: u16,
    pub popular: bool,
    pub author_id: Option<u64>,
    pub likes: Vec<u64>,
}

impl RecipeInfo {
    pub fn new(
        instructions: Vec<String>,
        ingredients: Vec<String>,
        tags: Vec<String>,
        cuisines: Vec<String>,
        total_time_in_seconds: u16,
        popular: bool,
        author_id: Option<u64>,
    ) -> Self {
        Self {
            instructions,
            ingredients,
            tags,
            cuisines,
            total_time_in_seconds,
            popular,
            author_id,
            likes: Vec::new(),
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RecipeBriefResponse {
    name: String,
    tags: Vec<String>,
    total_time: u16,
    author: Option<String>,
}

impl RecipeBriefResponse {
    pub fn new(name: String, tags: Vec<String>, total_time: u16, author: Option<String>) -> Self {
        Self {
            name,
            tags,
            total_time,
            author,
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RecipePayload {
    name: String,
    instructions: Vec<String>,
    ingredients: Vec<String>,
    tags: Vec<String>,
    cuisines: Vec<String>,
    total_time_in_seconds: u16,
    image: String,
    author_id: u64,
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
const TAGS_MAX_LENGTH: usize = 5;

#[query]
pub fn recipe_exists(name: String) -> bool {
    RECIPES.with(|recipes| recipes.borrow().contains_key(&name))
}

#[query]
pub fn take_recipe(name: String) -> Result<(RecipeInfo, Option<String>), Error> {
    RECIPES.with(|recipes| {
        match recipes.borrow().get(&name).map(|recipe| {
            let author_name = recipe
                .author_id
                .and_then(|author_id| get_username_by_index(author_id));
            (recipe.clone(), author_name)
        }) {
            Some((recipe, author_name)) => Ok((recipe, author_name)),
            None => Err(Error::RecipeNotFound {
                msg: "Recipe not found".to_string(),
            }),
        }
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
pub fn take_recipes_of_specific_type(recipes_type: String) -> Vec<RecipeBriefResponse> {
    RECIPES.with(|recipes| {
        let filtered_recipes = recipes
            .borrow()
            .iter()
            .filter(|(_, recipe)| {
                if recipes_type == "Popular" {
                    recipe.popular == true
                } else {
                    recipe.tags.iter().any(|tag| tag == &recipes_type)
                }
            })
            .map(|(name, recipe)| {
                let author_name = recipe
                    .author_id
                    .and_then(|author_id| get_username_by_index(author_id));
                transform_recipe_to_brief_response(name, recipe, author_name)
            })
            .collect();
        filtered_recipes
    })
}

fn transform_recipe_to_brief_response(
    name: String,
    recipe: RecipeInfo,
    author_name: Option<String>,
) -> RecipeBriefResponse {
    let first_tags = &recipe.tags[..TAGS_MAX_LENGTH.min(recipe.tags.len())];
    RecipeBriefResponse::new(
        name.clone(),
        first_tags.to_vec(),
        recipe.total_time_in_seconds / 60,
        author_name,
    )
}

#[query]
pub fn take_user_recipes_with_author_names() -> Vec<RecipeBriefResponse> {
    RECIPES.with(|recipes| {
        recipes
            .borrow()
            .iter()
            .filter_map(|(name, recipe)| {
                recipe.author_id.map(|author_id| {
                    let author_name = get_username_by_index(author_id);
                    transform_recipe_to_brief_response(name, recipe, author_name)
                })
            })
            .collect()
    })
}

#[query]
pub fn take_recipes_by_author(author_id: u64) -> Vec<RecipeBriefResponse> {
    RECIPES.with(|recipes| {
        recipes
            .borrow()
            .iter()
            .filter(|(_, recipe)| recipe.author_id == Some(author_id))
            .map(|(name, recipe)| {
                let author_name = get_username_by_index(author_id);
                transform_recipe_to_brief_response(name, recipe, author_name)
            })
            .collect()
    })
}

#[update]
pub fn add_recipe(payload: RecipePayload) -> Result<String, Error> {
    if recipe_exists(payload.name.clone()) {
        ic_cdk::api::print("Recipe already exists");
        return Err(Error::RecipeAlreadyExists {
            msg: "Recipe with this title already exists - change the title".to_string(),
        });
    };

    let new_recipe = RecipeInfo::new(
        payload.instructions,
        payload.ingredients,
        payload.tags,
        payload.cuisines,
        payload.total_time_in_seconds,
        false,
        Some(payload.author_id),
    );

    RECIPES.with(|recipes| {
        recipes
            .borrow_mut()
            .insert(payload.name.clone(), new_recipe);
    });

    if !payload.image.is_empty() {
        add_image(payload.name, payload.image);
    }
    Ok("Recipe added successfully".to_string())
}

#[update]
pub fn delete_recipe(name: String) -> Result<String, Error> {
    if !recipe_exists(name.clone()) {
        return Err(Error::RecipeNotFound {
            msg: "Recipe not found".to_string(),
        });
    }

    // delete all informations from recipes
    RECIPES.with(|recipes| {
        recipes.borrow_mut().remove(&name);
    });

    // delete image
    IMAGES.with(|images| {
        images.borrow_mut().remove(&name);
    });
    Ok("Recipe deleted successfully".to_string())
}

#[update]
pub fn delete_recipes_and_images_by_author(author_id: u64) {
    RECIPES.with(|recipes| {
        let mut recipes = recipes.borrow_mut();
        let keys_to_remove: Vec<String> = recipes
            .iter()
            .filter(|(_, recipe)| recipe.author_id == Some(author_id))
            .map(|(key, _)| key.clone())
            .collect();

        for key in &keys_to_remove {
            recipes.remove(key);

            // delete also image
            IMAGES.with(|images| {
                images.borrow_mut().remove(key);
            });
        }
    })
}
