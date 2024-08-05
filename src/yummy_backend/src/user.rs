use crate::recipes::images::{add_image, IMAGES};
use crate::recipes::{recipe_exists, take_recipe, RecipeBrief, RecipeInfo, RECIPES};
use crate::Error;
use crate::{Memory, MEMORY_MANAGER};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::{api::caller, query, update};
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{storable::Bound, Cell, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;

pub type IdCell = Cell<u64, Memory>;
thread_local! {
    static INDEX_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 1)
            .expect("Cannot create a counter")
    );

    pub static USERS: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct User {
    id: Principal,
    name: String,
    pub recipes: Vec<String>,
}

impl User {
    pub fn new(id: Principal, name: String) -> Self {
        Self {
            id,
            name,
            recipes: Vec::new(),
        }
    }
}
impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1024,
        is_fixed_size: false,
    };
}

#[update]
fn create_user(name: String) -> Result<u64, Error> {
    let id = caller();
    let index = INDEX_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    USERS.with(|users| {
        if users.borrow().iter().any(|(_, user)| user.id == id) {
            Err(Error::UserAlreadyExists {
                msg: "User already exists".to_string(),
            })
        } else {
            let user = User::new(id, name);
            users.borrow_mut().insert(index, user.clone());
            Ok(index)
        }
    })
}

#[update]
fn update_username(index: u64, name: String) -> Result<User, Error> {
    let user = USERS.with(|users| users.borrow().get(&index));
    match user {
        Some(user) => {
            if name.is_empty() {
                return Err(Error::InvalidName {
                    msg: "Name cannot be empty".to_string(),
                });
            }
            ic_cdk::api::print(format!("Updating username: {}", caller()));
            if caller() == Principal::anonymous() || user.id != caller() {
                return Err(Error::CallerNotAuthorized {
                    msg: "The caller is not authorized".to_string(),
                });
            }
            let updated_user = User {
                id: user.id,
                name,
                recipes: user.recipes,
            };
            USERS.with(|users| {
                users.borrow_mut().insert(index, updated_user.clone());
            });
            Ok(updated_user)
        }
        None => Err(Error::UserNotFound {
            msg: "User not found".to_string(),
        }),
    }
}

#[update]
fn delete_user() -> Result<String, Error> {
    let user = match get_user_info() {
        Ok(user) => user,
        Err(err) => return Err(err),
    };
    // delete all informations about user recipes
    RECIPES.with(|recipes| {
        let user_recipes = recipes
            .borrow()
            .iter()
            .filter(|(name, _)| user.recipes.contains(name))
            .collect::<Vec<(String, RecipeInfo)>>();
        for (name, _) in user_recipes {
            recipes.borrow_mut().remove(&name);
        }
    });

    // delete images
    IMAGES.with(|images| {
        let user_images = images
            .borrow()
            .iter()
            .filter(|(name, _)| user.recipes.contains(name))
            .collect::<Vec<(String, String)>>();
        for (name, _) in user_images {
            images.borrow_mut().remove(&name);
        }
    });

    // delete from user list
    USERS.with(|users| {
        let index = match get_user_index() {
            Ok(index) => index,
            Err(err) => return Err(err),
        };
        users.borrow_mut().remove(&index);
        Ok("User deleted successfully".to_string())
    })
}

#[query]
fn get_user_by_index(index: u64) -> Result<User, Error> {
    let user = USERS.with(|users| users.borrow().get(&index));
    match user {
        Some(user) => Ok(user.clone()),
        None => Err(Error::UserNotFound {
            msg: "User not found".to_string(),
        }),
    }
}
#[query]
fn get_user_index() -> Result<u64, Error> {
    USERS.with(|users| {
        ic_cdk::println!("Get user index by principal caller: {}", caller());

        match users.borrow().iter().find(|(_, user)| user.id == caller()) {
            Some((index, _)) => Ok(index),
            None => Err(Error::UserNotFound {
                msg: "You are not authenticated - login to perform this action".to_string(),
            }),
        }
    })
}

#[query]
fn get_user_info() -> Result<User, Error> {
    USERS.with(|users| {
        ic_cdk::api::print(format!("Get user by principal caller: {}", caller()));
        match users.borrow().iter().find(|(_, user)| user.id == caller()) {
            Some((_, user)) => Ok(user),
            None => Err(Error::UserNotFound {
                msg: "You are not authenticated - login to perform this action".to_string(),
            }),
        }
    })
}

#[query]
fn get_all_users() -> Result<Vec<User>, Error> {
    USERS.with(|users| {
        let records: Vec<User> = users
            .borrow()
            .iter()
            .map(|(_, user)| user.clone())
            .collect();

        if records.is_empty() {
            return Err(Error::UserNotFound {
                msg: "No users found".to_string(),
            });
        } else {
            Ok(records)
        }
    })
}

#[update]
pub fn add_new_recipe(
    name: String,
    instructions: Vec<String>,
    ingredients: Vec<String>,
    tags: Vec<String>,
    total_time_minutes: u16,
    image_data: String,
    user_index: u64,
) -> Result<String, Error> {
    if recipe_exists(name.clone()) {
        ic_cdk::api::print("Recipe already exists");
        return Err(Error::RecipeAlreadyExists {
            msg: "Recipe with this title already exists - change the title".to_string(),
        });
    };

    let username = USERS.with(|users| {
        let mut user = users.borrow().get(&user_index).unwrap().clone();
        user.recipes.push(name.clone());
        users.borrow_mut().insert(user_index, user.clone());
        ic_cdk::println!("Recipe added to user");
        user.name.clone()
    });
    let new_recipe = RecipeInfo::new(
        instructions,
        ingredients,
        tags,
        total_time_minutes * 60 as u16,
        Some(username),
    );

    RECIPES.with(|recipes| {
        recipes.borrow_mut().insert(name.clone(), new_recipe);
    });

    if !image_data.is_empty() {
        add_image(name, image_data);
    }
    Ok("Recipe added successfully".to_string())
}

#[update]
pub fn delete_recipe(name: String, user_index: u64) -> Result<String, Error> {
    if !recipe_exists(name.clone()) {
        return Err(Error::RecipeNotFound {
            msg: "Recipe not found".to_string(),
        });
    }

    let user = USERS.with(|users| users.borrow().get(&user_index));
    match user {
        Some(mut user) => {
            if !user.recipes.contains(&name) {
                return Err(Error::RecipeNotFound {
                    msg: "Recipe not found for this user".to_string(),
                });
            }
            user.recipes.retain(|recipe_name| recipe_name != &name);
            ic_cdk::api::print(format!("user {:?} ", user.clone()));

            // Delete from user recipes
            USERS.with(|users| {
                users.borrow_mut().insert(user_index, user);
            });

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
        None => Err(Error::UserNotFound {
            msg: "User not found".to_string(),
        }),
    }
}

#[query]
pub fn take_user_recipes() -> Vec<RecipeBrief> {
    const TAGS_MAX_LENGTH: usize = 5;
    USERS.with(|users| {
        let mut user_recipes = Vec::new();
        match users.borrow().iter().find(|(_, user)| user.id == caller()) {
            Some((_, user)) => {
                for name in &user.recipes {
                    let recipe = match take_recipe(name.to_string()) {
                        Ok(recipe) => recipe,
                        Err(_) => return user_recipes,
                    };
                    let first_tags = &recipe.tags[..TAGS_MAX_LENGTH.min(recipe.tags.len())];
                    user_recipes.push(RecipeBrief::new(
                        name.to_string(),
                        first_tags.to_vec(),
                        recipe.total_time_in_seconds / 60,
                        Some(user.name.clone()),
                    ));
                }
            }
            None => return user_recipes,
        };
        user_recipes
    })
}
