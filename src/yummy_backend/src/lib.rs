extern crate serde;
use candid::Principal;
use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};
use std::cell::RefCell;
use std::vec::Vec;

#[derive(CandidType, Clone, Deserialize)]
struct User {
    id: Principal,
    name: String,
}

thread_local! {
    static USERS: RefCell<Vec<User>> = RefCell::new(Vec::new());
}

#[update]
fn create_user(name: String) -> Result<String, Error> {
    let id = ic_cdk::caller();

    USERS.with(|users| {
        if users.borrow().iter().any(|user| user.id == id) {
            Err(Error::UserAlreadyExists {
                msg: "User already exists".to_string(),
            })
        } else {
            let user = User { id, name };
            users.borrow_mut().push(user);
            Ok("User succesfully created".to_string())
        }
    })
}

#[update]
fn delete_user() -> Result<String, Error> {
    let id = ic_cdk::caller();
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        if let Some(index) = users.iter().position(|user| user.id == id) {
            users.remove(index);
            Ok("User successfully deleted".to_string())
        } else {
            Err(Error::UserNotFound {
                msg: "User not found".to_string(),
            })
        }
    })
}

#[update]
fn delete_user_by_id(id: Principal) -> Result<String, Error> {
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        if let Some(index) = users.iter().position(|user| user.id == id) {
            users.remove(index);
            Ok("User successfully deleted".to_string())
        } else {
            Err(Error::UserNotFound {
                msg: "User not found".to_string(),
            })
        }
    })
}

#[query]
fn get_user(id: Principal) -> Result<User, Error> {
    USERS.with(
        |users| match users.borrow().iter().find(|user| user.id == id) {
            Some(user) => Ok(user.clone()),
            None => Err(Error::UserNotFound {
                msg: "User not found".to_string(),
            }),
        },
    )
}

#[query]
fn get_all_users() -> Vec<User> {
    USERS.with(|users| users.borrow().clone())
}

#[query]
fn whoami() -> Principal {
    ic_cdk::caller()
}

// Error types
#[derive(CandidType)]
enum Error {
    UserNotFound { msg: String },
    UserAlreadyExists { msg: String },
}

// Required by Candid
ic_cdk::export_candid!();
