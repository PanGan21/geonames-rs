use geonames_rs::{
    AdminCodes1, ApiClient, ApiEndpoint, AstergdemResponse, ChildrenResponse, CountryCodeResponse,
    GeoNamesApi, Geoname, PostalCode, PostalCodeSearchResponse,
};
use std::collections::HashMap;

const USERNAME: &str = env!("API_USER");

#[test]
fn call_api_astergdem() {
    let client = ApiClient::new(GeoNamesApi::Astergdem, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "47.03");
    params.insert("lng", "10.02");

    let result: AstergdemResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = AstergdemResponse {
        lng: 10.02,
        lat: 47.03,
        astergdem: 1968,
    };

    assert_eq!(result, expected_result);
}

#[test]
fn call_api_children() {
    let client = ApiClient::new(GeoNamesApi::Children, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("geonameId", "3175395");
    params.insert("maxRows", "1");

    let result: ChildrenResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = ChildrenResponse {
        total_results_count: 20,
        geonames: vec![Geoname {
            admin_code_1: "16".to_string(),
            lng: "11".to_string(),
            geoname_id: 3165361,
            toponym_name: "Toscana".to_string(),
            country_id: "3175395".to_string(),
            admin_codes1: AdminCodes1 {
                iso3166_2: "52".to_string(),
            },
            country_name: "Italy".to_string(),
            fcode_name: "first-order administrative division".to_string(),
            admin_name1: "Tuscany".to_string(),
            lat: "43.41667".to_string(),
            fcode: "ADM1".to_string(),
            fcl: "A".to_string(),
            population: 3729641,
            country_code: "IT".to_string(),
            name: "Tuscany".to_string(),
            fcl_name: "country, state, region,...".to_string(),
        }],
    };

    assert_eq!(result, expected_result);
}

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
}
