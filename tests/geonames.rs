use geonames_rs::{ApiClient, ApiEndpoint, GeoNamesApi};
use std::collections::HashMap;

const USERNAME: &str = "";

#[test]
fn call_api_no_allowed_params() {
    let client = ApiClient::new(GeoNamesApi::CountryCode, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "47.03");
    params.insert("lng", "10.02");

    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();
}
