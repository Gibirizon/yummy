use crate::https::Recipe;
use crate::Error;
use crate::{Memory, MEMORY_MANAGER};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk::query;
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

thread_local! {

    pub static IMAGES: RefCell<StableBTreeMap<String, Vec<u8> , Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );

}
const MAX_RESPONSE_BYTES: u64 = 2 * 1024 * 1024; // 2 MB

pub async fn fetch_and_store_images(recipes_list: Vec<Recipe>) -> String {
    let mut url;
    let mut offset;
    let mut total_size;
    let mut image;
    for recipe in recipes_list {
        offset = 0;
        total_size = 0;
        url = recipe.node.main_image.as_str();
        image = Vec::new();
        ic_cdk::api::print(format!("URL: {}", url));

        while offset < total_size || total_size == 0 {
            let request_headers = vec![
                HttpHeader {
                    name: "Host".to_string(),
                    value: "storage.googleapis.com".to_string(),
                },
                HttpHeader {
                    name: "Range".to_string(),
                    value: format!("bytes={}-{}", offset, offset + MAX_RESPONSE_BYTES - 1),
                },
                HttpHeader {
                    name: "Transfer-Encoding".to_string(),
                    value: "chunked".to_string(),
                },
                HttpHeader {
                    name: "Content-Type".to_string(),
                    value: "text/plain".to_string(),
                },
            ];

            let request = CanisterHttpRequestArgument {
                url: url.to_string(),
                method: HttpMethod::GET,
                body: None,
                max_response_bytes: None,
                transform: None,
                headers: request_headers,
            };

            let cycles = 230_949_972_000;
            match http_request(request, cycles).await {
                Ok((response,)) => {
                    ic_cdk::api::print(format!("Response status {:?}", response.status));
                    if response.status == 206u64 || response.status == 200u64 {
                        // Parse Content-Range header to get total size
                        if total_size == 0 {
                            for header in &response.headers {
                                if header.name.to_lowercase() == "content-range" {
                                    ic_cdk::api::print(format!(
                                        "Headers content-range: {:?}",
                                        header.value
                                    ));
                                    if let Some(size) = parse_content_range(&header.value) {
                                        total_size = size;
                                    }
                                }
                            }
                        }
                        image.extend(response.body);
                        offset += MAX_RESPONSE_BYTES;
                    }
                }
                Err(e) => {
                    ic_cdk::api::print(format!("Failed to fetch chunk: {:?}", e));
                    break;
                }
            }
        }
        if image == Vec::<u8>::new() {
            ic_cdk::api::print("Image is empty");
            continue;
        }
        IMAGES.with(|images| {
            images.borrow_mut().insert(recipe.node.name, image);
        });
    }
    String::from("Done")
}

fn parse_content_range(header: &str) -> Option<u64> {
    let parts: Vec<&str> = header.split('/').collect();
    if parts.len() == 2 {
        parts[1].parse().ok()
    } else {
        None
    }
}

#[query]
pub async fn get_image(name: String) -> Vec<u8> {
    IMAGES.with(|images| images.borrow().get(&name).clone().unwrap())
}

#[query]
pub async fn get_images_names() -> Result<Vec<String>, Error> {
    IMAGES.with(|images| {
        let stable_btree_map = &*images.borrow();

        let records: Vec<String> = stable_btree_map
            .iter()
            .map(|(name, _)| name.clone())
            .collect();

        if records.is_empty() {
            return Err(Error::ImagesNotFound {
                msg: "No images found".to_string(),
            });
        } else {
            Ok(records)
        }
    })
}
