extern crate serde;
use candid::CandidType;
use ic_cdk::init;
use ic_stable_structures::memory_manager::{MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use recipes::images::ImageData;
use recipes::{recipes_initialization, RecipeInfo};
use std::cell::RefCell;
use std::time::Duration;
use user::User;

pub mod recipes;
pub mod user;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
}

#[init]
fn init() {
    ic_cdk::println!("Canister initialized. Setting up timer for HTTP call.");
    setup_for_timer();
}

fn set_timer(query: String, recipes_type: String, secs: u64) {
    ic_cdk_timers::set_timer(Duration::from_secs(secs), || {
        ic_cdk::spawn(async {
            recipes_initialization(query, recipes_type).await;
        });
    });
}

fn setup_for_timer() {
    let mut secs: u64 = 5;
    let query = r#"
    query {
        popularRecipes {
            edges {
                node {
                    cuisines
                    tags
                    instructions
                    totalTimeInSeconds
                    ingredients {
                        name
                    }
                    mainImage
                    name
                }
            }
    }
  }
    "#
    .to_string();

    set_timer(query, "popularRecipes".to_string(), secs);
    secs += 1;
    let tags = ["Breakfast".to_string(), "Dinner".to_string()];
    for tag in tags {
        let query = format!(
            r#"
    query {{
        recipesByTag (tag: "{}") {{
            edges {{
                node {{
                    cuisines
                    tags
                    instructions
                    totalTimeInSeconds
                    ingredients {{
                        name
                    }}
                    mainImage
                    name
                }}
            }}
        }}
    }}
"#,
            tag
        );
        set_timer(query, "recipesByTag".to_string(), secs);
        secs += 1;
    }
}

// Error types
#[derive(CandidType)]
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
