use geonames_rs::{
    AdminCodes1, ApiClient, ApiEndpoint, AstergdemResponse, ChildrenResponse, CitiesGeoname,
    CitiesResponse, ContainsResponse, CountryCodeResponse, CountryInfoGeoname, CountryInfoResponse,
    CountrySubvisionCode, CountrySubvisionResponse, Earthquake, EarthquakesResponse,
    FindNearbyPlaceResponse, FindNearbyPostalCodesResponse, FindNearbyResponse,
    FindNearbyStreetsOSMResponse, GeoNamesApi, Geoname, GeonameNearbyPlace, PostalCode,
    PostalCodeFindNearby, PostalCodeSearchResponse, StreetSegment,
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
fn call_api_cities() {
    let client = ApiClient::new(GeoNamesApi::Cities, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("north", "44.1");
    params.insert("south", "-9.9");
    params.insert("east", "-22.4");
    params.insert("west", "55.2");
    params.insert("maxRows", "1");

    let result: CitiesResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = CitiesResponse {
        geonames: vec![CitiesGeoname {
            lng: 116.397228240967,
            geoname_id: 1816670,
            name: "Beijing".to_string(),
            fcl_name: "city, village,...".to_string(),
            toponym_name: "Beijing".to_string(),
            fcode_name: "capital of a political entity".to_string(),
            lat: 39.9074977414405,
            fcl: "P".to_string(),
            population: 18960744,
            fcode: "PPLC".to_string(),
            countrycode: "CN".to_string(),
            wikipedia: "en.wikipedia.org/wiki/Beijing".to_string(),
        }],
    };

    assert_eq!(result, expected_result);
}

#[test]
fn call_api_contains() {
    let client = ApiClient::new(GeoNamesApi::Contains, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("geonameId", "2746385");
    params.insert("maxRows", "1");

    let result: ContainsResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = ContainsResponse {
        geonames: vec![Geoname {
            admin_code_1: "07".to_string(),
            lng: "4.81667".to_string(),
            geoname_id: 2749011,
            toponym_name: "Oude Schans".to_string(),
            country_id: "2750405".to_string(),
            admin_codes1: AdminCodes1 {
                iso3166_2: "NH".to_string(),
            },
            country_name: "The Netherlands".to_string(),
            fcode_name: "populated place".to_string(),
            admin_name1: "North Holland".to_string(),
            lat: "53.03333".to_string(),
            fcode: "PPL".to_string(),
            fcl: "P".to_string(),
            population: 0,
            country_code: "NL".to_string(),
            name: "Oude Schans".to_string(),
            fcl_name: "city, village,...".to_string(),
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

    let expected_result = CountryCodeResponse {
        languages: "de-AT,hr,hu,sl".to_string(),
        distance: "0".to_string(),
        country_code: "AT".to_string(),
        country_name: "Austria".to_string(),
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_country_info() {
    let client = ApiClient::new(GeoNamesApi::CountryInfo, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("country", "NL");

    let result: CountryInfoResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = CountryInfoResponse {
        geonames: vec![CountryInfoGeoname {
            continent: "EU".to_string(),
            capital: "Amsterdam".to_string(),
            languages: "nl-NL,fy-NL".to_string(),
            geoname_id: 2750405,
            south: 50.7503674993741,
            iso_alpha3: "NLD".to_string(),
            north: 53.5157125645109,
            fips_code: "NL".to_string(),
            population: "17231017".to_string(),
            east: 7.22749859212922,
            iso_numeric: "528".to_string(),
            area_in_sq_km: "41526.0".to_string(),
            country_code: "NL".to_string(),
            west: 3.35837827202,
            country_name: "The Netherlands".to_string(),
            postal_code_format: "#### @@".to_string(),
            continent_name: "Europe".to_string(),
            currency_code: "EUR".to_string(),
        }],
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_country_subvision() {
    let client = ApiClient::new(GeoNamesApi::CountrySubdivision, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "47.03");
    params.insert("lng", "10.2");

    let result: CountrySubvisionResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = CountrySubvisionResponse {
        codes: vec![CountrySubvisionCode {
            code: "7".to_string(),
            level: "1".to_string(),
            ty: "ISO3166-2".to_string(),
        }],
        admin_code1: "07".to_string(),
        distance: 0.0,
        country_code: "AT".to_string(),
        country_name: "Austria".to_string(),
        admin_name1: "Tyrol".to_string(),
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_earthquakes() {
    let client = ApiClient::new(GeoNamesApi::Earthquakes, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("north", "44.1");
    params.insert("south", "-9.9");
    params.insert("east", "-22.4");
    params.insert("west", "55.2");
    params.insert("maxRows", "1");

    let result: EarthquakesResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = EarthquakesResponse {
        earthquakes: vec![Earthquake {
            datetime: "2011-03-11 04:46:23".to_string(),
            depth: 24.4,
            lng: 142.369,
            src: "us".to_string(),
            eqid: "c0001xgp".to_string(),
            magnitude: 8.8,
            lat: 38.322,
        }],
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_find_nearby() {
    let client = ApiClient::new(GeoNamesApi::FindNearby, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "47.3");
    params.insert("lng", "9");
    params.insert("maxRows", "1");

    let result: FindNearbyResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = FindNearbyResponse {
        geonames: vec![Geoname {
            admin_code_1: "SG".to_string(),
            lng: "8.99667".to_string(),
            geoname_id: 11783836,
            toponym_name: "Habrüti".to_string(),
            country_id: "2658434".to_string(),
            admin_codes1: AdminCodes1 {
                iso3166_2: "SG".to_string(),
            },
            country_name: "Switzerland".to_string(),
            fcode_name: "house(s)".to_string(),
            admin_name1: "Saint Gallen".to_string(),
            lat: "47.30437".to_string(),
            fcode: "HSE".to_string(),
            fcl: "S".to_string(),
            population: 0,
            country_code: "CH".to_string(),
            name: "Habrüti".to_string(),
            fcl_name: "spot, building, farm".to_string(),
        }],
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_find_nearby_place_name() {
    let client = ApiClient::new(GeoNamesApi::FindNearbyPlaceName, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "47.3");
    params.insert("lng", "9");
    params.insert("maxRows", "1");

    let result: FindNearbyPlaceResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = FindNearbyPlaceResponse {
        geonames: vec![GeonameNearbyPlace {
            admin_code_1: "SG".to_string(),
            lng: "9.01488".to_string(),
            geoname_id: 7910950,
            toponym_name: "Chrüzegg".to_string(),
            country_id: "2658434".to_string(),
            admin_codes1: AdminCodes1 {
                iso3166_2: "SG".to_string(),
            },
            country_name: "Switzerland".to_string(),
            fcode_name: "section of populated place".to_string(),
            admin_name1: "Saint Gallen".to_string(),
            lat: "47.2985".to_string(),
            fcode: "PPLX".to_string(),
            fcl: "P".to_string(),
            population: 0,
            country_code: "CH".to_string(),
            name: "Chrüzegg".to_string(),
            fcl_name: "city, village,...".to_string(),
            distance: "1.1379".to_string(),
        }],
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_find_nearby_postal_codes() {
    let client = ApiClient::new(GeoNamesApi::FindNearbyPostalCodes, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "47");
    params.insert("lng", "9");
    params.insert("maxRows", "1");

    let result: FindNearbyPostalCodesResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = FindNearbyPostalCodesResponse {
        postal_codes: vec![PostalCodeFindNearby {
            admin_code1: "GL".to_string(),
            admin_code2: "800".to_string(),
            admin_code3: "1631".to_string(),
            admin_name1: "Kanton Glarus".to_string(),
            admin_name2: "Glarus".to_string(),
            admin_name3: "Glarus Süd".to_string(),
            lng: 9.00123733838,
            distance: "2.6241".to_string(),
            country_code: "CH".to_string(),
            postal_code: "8775".to_string(),
            place_name: "Luchsingen".to_string(),
            lat: 46.9764148249,
        }],
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_find_nearby_streets_osm() {
    let client = ApiClient::new(GeoNamesApi::FindNearbyStreetsOsm, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "37.451");
    params.insert("lng", "-122.18");
    params.insert("maxRows", "1");

    let result: FindNearbyStreetsOSMResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = FindNearbyStreetsOSMResponse {
        street_segment: StreetSegment {
            way_id: "8928471".to_string(),
            distance: "0.06".to_string(),
            line: "-122.1796917 37.4520107,-122.1798016 37.4518965,-122.1799937 37.4516636,-122.1801139 37.4515178,-122.1808293 37.4506505,-122.180988 37.4504593,-122.1817112 37.4495966,-122.1822516 37.4489518,-122.1831946 37.4478272,-122.1832534 37.4477571".to_string(),
            country_code: "US".to_string(),
            name: "Roble Avenue".to_string(),
            highway: "residential".to_string(),
        },
    };
    assert_eq!(result, expected_result);
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
