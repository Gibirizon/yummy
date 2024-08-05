use crate::{Memory, MEMORY_MANAGER};
use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

thread_local! {
    pub static IMAGES: RefCell<StableBTreeMap<String, String , Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        )
    );

}

#[update]
pub fn add_image(name: String, image: String) {
    IMAGES.with(|images| images.borrow_mut().insert(name, image));
}

#[query]
pub fn get_image(name: String) -> String {
    IMAGES.with(|images| {
        let image = images.borrow().get(&name).clone();
        match image {
            Some(image) => image,
            None => "".to_string(),
        }
    })
}
