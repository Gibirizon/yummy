extern crate serde;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use https::RecipeStorage;
use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{storable::Bound, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;

pub mod https;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type IdCell = Cell<u64, Memory>;

#[derive(CandidType, Clone, Deserialize)]
struct User {
    id: Principal,
    name: String,
}

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static INDEX_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 1)
            .expect("Cannot create a counter")
    );

    static USERS: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

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
fn create_user(name: String) -> Result<User, Error> {
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
            let user = User { id, name };
            users.borrow_mut().insert(index, user.clone());
            Ok(user)
        }
    })
}
#[update]
fn update_user_by_index(index: u64, name: String) -> Result<User, Error> {
    let user = USERS.with(|users| users.borrow().get(&index));
    match user {
        Some(user) => {
            if name.is_empty() {
                return Err(Error::InvalidName {
                    msg: "Name cannot be empty".to_string(),
                });
            }
            ic_cdk::api::print(format!("caller: {}", ic_cdk::api::caller()));
            if ic_cdk::caller() == Principal::anonymous() || user.id != ic_cdk::caller() {
                return Err(Error::CallerNotAuthorized {
                    msg: "The caller is not authorized".to_string(),
                });
            }
            let updated_user = User { id: user.id, name };
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
fn get_user(index: u64) -> Result<User, Error> {
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
        ic_cdk::api::print(format!("caller: {}", ic_cdk::api::caller()));
        let index = match users.iter().find(|(_, user)| user.id == ic_cdk::caller()) {
            Some((index, _)) => Ok(index),
            None => Err(Error::UserNotFound {
                msg: "User not found".to_string(),
            }),
        };
        index
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

#[query]
fn whoami() -> Principal {
    ic_cdk::caller()
}

// Error types
#[derive(CandidType)]
pub enum Error {
    UserNotFound { msg: String },
    UserAlreadyExists { msg: String },
    RecipesNotFound { msg: String },
    InvalidName { msg: String },
    CallerNotAuthorized { msg: String },
}

// enable the export of the candid file
ic_cdk::export_candid!();
