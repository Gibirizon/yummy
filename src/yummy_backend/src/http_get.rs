use crate::http_get::api_key::SUGGESTIC_API_KEY;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use urlencoding::encode;
mod api_key;

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
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            message
        }
    }
}
