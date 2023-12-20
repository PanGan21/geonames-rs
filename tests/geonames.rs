use geonames_rs::{
    ApiClient, ApiEndpoint, CountryCodeResponse, GeoNamesApi, PostalCode, PostalCodeSearchResponse,
};
use std::collections::HashMap;

const USERNAME: &str = env!("API_USER");

#[test]
fn call_api_country_code() {
    let client = ApiClient::new(GeoNamesApi::CountryCode, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "47.03");
    params.insert("lng", "10.02");

    let result: CountryCodeResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let excpected_result = CountryCodeResponse {
        languages: "de-AT,hr,hu,sl".to_string(),
        distance: "0".to_string(),
        country_code: "AT".to_string(),
        country_name: "Austria".to_string(),
    };
    assert_eq!(result, excpected_result);
}

#[test]
fn call_api_postal_code_search() {
    let client = ApiClient::new(GeoNamesApi::PostalCodeSearch, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("postalcode", "1033 SC");

    let result: PostalCodeSearchResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = PostalCodeSearchResponse {
        postal_codes: vec![PostalCode {
            admin_code_1: "07".to_string(),
            admin_code_2: "0363".to_string(),
            admin_name_1: "Noord-Holland".to_string(),
            admin_name_2: "Amsterdam".to_string(),
            lng: 4.891274330675293,
            country_code: "NL".to_string(),
            postal_code: "1033 SC".to_string(),
            iso: "NH".to_string(),
            place_name: "Amsterdam".to_string(),
            lat: 52.40451488171361,
        }],
    };

    assert_eq!(result, expected_result);

    // let excpected_result = CountryCodeResponse {
    //     languages: "de-AT,hr,hu,sl".to_string(),
    //     distance: "0".to_string(),
    //     country_code: "AT".to_string(),
    //     country_name: "Austria".to_string(),
    // };
    // assert_eq!(result, excpected_result);
}
