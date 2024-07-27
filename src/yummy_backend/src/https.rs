use crate::https::api_key::SUGGESTIC_API_KEY;
use crate::https::images::fetch_and_store_images;
use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk::update;
use serde_json::{from_str, Value};
use urlencoding::encode;

mod api_key;
pub mod images;

#[derive(CandidType, Deserialize, Clone)]
pub struct Recipe {
    node: RecipeInfo,
}

#[derive(CandidType, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecipeInfo {
    id: String,
    main_image: String,
    name: String,
}

#[update]
pub async fn popular_recipes() -> String {
    let query = r#"
    query {
        popularRecipes {
            edges {
                node {
                    id
                    name
                    mainImage
                }
            }
        }
    }
    "#;
    get_recipes(query).await
}

pub async fn get_recipes(query: &str) -> String {
    let encoded_query = encode(query);

    let url = format!(
        "https://production.suggestic.com/graphql?query={}",
        encoded_query
    );
    let request_headers = vec![HttpHeader {
        name: "Authorization".to_string(),
        value: format!("Token {}", SUGGESTIC_API_KEY),
    }];

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
            let res = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");
            let json: Value = from_str(&res).unwrap();
            let data = json
                .get("data")
                .unwrap()
                .get("popularRecipes")
                .unwrap()
                .get("edges")
                .unwrap();
            let recipes_list: Vec<Recipe> = serde_json::from_value(data.clone()).unwrap();
            get_images_of_recipes(recipes_list).await;

            ic_cdk::api::print(format!("JSON: {:#?}", data));
            res
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            message
        }
    }
}

pub async fn get_images_of_recipes(recipes_list: Vec<Recipe>) -> String {
    fetch_and_store_images(recipes_list).await
}
