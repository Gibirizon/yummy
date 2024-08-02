use crate::recipes::images::{add_image, ImageData};
use crate::recipes::{recipe_exists, RecipeInfo};
use crate::Error;
use crate::{Memory, MEMORY_MANAGER};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{storable::Bound, Cell, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

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
    pub recipes: HashMap<String, RecipeInfo>,
}

impl User {
    pub fn new(id: Principal, name: String) -> Self {
        Self {
            id,
            name,
            recipes: HashMap::new(),
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
    let id = ic_cdk::caller();
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
            ic_cdk::api::print(format!("Updating username: {}", ic_cdk::api::caller()));
            if ic_cdk::caller() == Principal::anonymous() || user.id != ic_cdk::caller() {
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
fn delete_user_by_index(index: u64) -> Result<String, Error> {
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        if let Some(_) = users
            .iter()
            .position(|(storage_index, _)| storage_index == index)
        {
            users.remove(&index);
            Ok("User successfully deleted".to_string())
        } else {
            Err(Error::UserNotFound {
                msg: "User not found".to_string(),
            })
        }
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
fn get_user_index_by_principal() -> Result<u64, Error> {
    USERS.with(|users| {
        let users = &*users.borrow();
        ic_cdk::println!(
            "Get user index by principal caller: {}",
            ic_cdk::api::caller()
        );

        let index = match users.iter().find(|(_, user)| user.id == ic_cdk::caller()) {
            Some((index, _)) => Ok(index),
            None => Err(Error::UserNotFound {
                msg: "You are not authenticated - login to perform this action".to_string(),
            }),
        };
        index
    })
}

#[query]
fn get_user_info_by_principal() -> Result<User, Error> {
    USERS.with(|users| {
        let users = &*users.borrow();
        ic_cdk::api::print(format!(
            "Get user by principal caller: {}",
            ic_cdk::api::caller()
        ));
        let user = match users.iter().find(|(_, user)| user.id == ic_cdk::caller()) {
            Some((_, user)) => Ok(user),
            None => Err(Error::UserNotFound {
                msg: "You are not authenticated - login to perform this action".to_string(),
            }),
        };
        user
    })
}

#[query]
fn get_all_users() -> Result<Vec<User>, Error> {
    USERS.with(|storage| {
        let stable_btree_map = &*storage.borrow();

        let records: Vec<User> = stable_btree_map
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
    let new_recipe = RecipeInfo::new(
        instructions,
        ingredients,
        tags,
        total_time_minutes * 60 as u16,
    );

    USERS.with(|users| {
        let mut user = users.borrow().get(&user_index).unwrap().clone();
        user.recipes.insert(name.clone(), new_recipe);
        users.borrow_mut().insert(user_index, user);
        ic_cdk::println!("Recipe added to user")
    });

    add_image(name, ImageData::Url(image_data));
    Ok("Recipe added successfully".to_string())
}