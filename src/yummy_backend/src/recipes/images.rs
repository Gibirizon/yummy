use crate::Error;
use crate::{Memory, MEMORY_MANAGER};
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{storable::Bound, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;

const MAX_RESPONSE_BYTES: u64 = 2 * 1024 * 1024; // 2 MB

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum ImageData {
    Bytes(Vec<u8>),
    Url(String),
}

thread_local! {
    pub static IMAGES: RefCell<StableBTreeMap<String, ImageData , Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        )
    );

}

impl Storable for ImageData {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 3 * 1024 * 1024,
        is_fixed_size: false,
    };
}

pub async fn fetch_image(url: &str, name: &str) -> Result<String, Error> {
    let mut offset = 0;
    let mut total_size = 0;
    let mut image = Vec::new();

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
            Err(_) => {
                return Err(Error::ImageNotDownloaded {
                    msg: "Cannot download image".to_string(),
                });
            }
        }
    }
    add_image(name.to_string(), ImageData::Bytes(image));
    Ok("Fetched and stored image".to_string())
}

fn parse_content_range(header: &str) -> Option<u64> {
    let parts: Vec<&str> = header.split('/').collect();
    if parts.len() == 2 {
        parts[1].parse().ok()
    } else {
        None
    }
}

#[update]
pub fn add_image(name: String, image: ImageData) {
    IMAGES.with(|images| images.borrow_mut().insert(name, image));
}

#[query]
pub fn get_image(name: String) -> ImageData {
    IMAGES.with(|images| images.borrow().get(&name).clone().unwrap())
}
