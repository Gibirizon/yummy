extern crate serde;
use candid::{CandidType, Deserialize};
use ic_cdk::init;
use ic_stable_structures::memory_manager::{MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use recipes::{create_recipe, RecipeBrief, RecipeInfo};
use serde_json::{from_str, Value};
use std::cell::RefCell;
use user::User;

pub mod recipes;
pub mod user;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
}

#[derive(CandidType, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecipeInside {
    name: String,
    main_image: String,
    instructions: Vec<String>,
    ingredient_lines: Vec<String>,
    cuisines: Option<Vec<String>>,
    tags: Vec<String>,
    total_time_in_seconds: u16,
}
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RecipeSingleItem {
    node: RecipeInside,
}

#[init]
fn init() {
    // ic_cdk::println!("Canister initialized. Setting up timer for HTTP call.");
    // setup_for_timer();
    const FILE_CONTENTS: [&str; 5] = [
        include_str!("../data/data1.json"),
        include_str!("../data/data2.json"),
        include_str!("../data/data3.json"),
        include_str!("../data/data4.json"),
        include_str!("../data/data5.json"),
    ];

    for i in 0..5 {
        let data = FILE_CONTENTS[i];
        let json: Value = from_str(&data).unwrap();
        let recipes_info;
        if i == 0 {
            recipes_info = json
                .get("data")
                .unwrap()
                .get("popularRecipes")
                .unwrap()
                .get("edges")
                .unwrap();
        } else {
            recipes_info = json
                .get("data")
                .unwrap()
                .get("recipesByTag")
                .unwrap()
                .get("edges")
                .unwrap();
        }
        let recipes_list: Vec<RecipeSingleItem> =
            serde_json::from_value(recipes_info.clone()).unwrap();

        for recipe in recipes_list {
            let new_recipe = RecipeInfo {
                instructions: recipe.node.instructions,
                cuisines: recipe.node.cuisines,
                tags: recipe.node.tags,
                total_time_in_seconds: recipe.node.total_time_in_seconds,
                ingredients: recipe.node.ingredient_lines,
                popular: if i == 0 { true } else { false },
                author: None,
            };
            if i == 0 {
                ic_cdk::println!(
                    "Recipe: {}, popular: {}",
                    recipe.node.name,
                    new_recipe.popular
                );
            }

            // RECIPES.with(|m| m.borrow_mut().insert(recipe.node.name, new_recipe));
            create_recipe(recipe.node.name, new_recipe);
        }
    }
}

// Error types
#[derive(CandidType, Debug)]
pub enum Error {
    UserNotFound { msg: String },
    UserAlreadyExists { msg: String },
    RecipesNotFound { msg: String },
    InvalidName { msg: String },
    CallerNotAuthorized { msg: String },
    RecipeNotFound { msg: String },
    ImageNotDownloaded { msg: String },
    RecipeAlreadyExists { msg: String },
}

// enable the export of the candid file
ic_cdk::export_candid!();
