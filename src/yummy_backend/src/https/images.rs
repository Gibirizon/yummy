use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};

const MAX_RESPONSE_BYTES: u64 = 2 * 1024 * 1024; // 2 MB

pub async fn fetch_image(url: &str) -> Vec<u8> {
    let mut offset = 0;
    let mut total_size = 0;
    let mut image = Vec::new();
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
    image
}

fn parse_content_range(header: &str) -> Option<u64> {
    let parts: Vec<&str> = header.split('/').collect();
    if parts.len() == 2 {
        parts[1].parse().ok()
    } else {
        None
    }
}

// #[query]
// pub async fn get_image(name: String) -> Vec<u8> {
//     IMAGES.with(|images| images.borrow().get(&name).clone().unwrap())
// }

// #[query]
// pub async fn get_images_names() -> Result<Vec<String>, Error> {
//     IMAGES.with(|images| {
//         let stable_btree_map = &*images.borrow();

//         let records: Vec<String> = stable_btree_map
//             .iter()
//             .map(|(name, _)| name.clone())
//             .collect();

//         if records.is_empty() {
//             return Err(Error::ImagesNotFound {
//                 msg: "No images found".to_string(),
//             });
//         } else {
//             Ok(records)
//         }
//     })
// }
