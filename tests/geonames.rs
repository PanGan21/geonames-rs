use geonames_rs::{
    Address, AddressResponse, AdminCodes1, AlternateName, ApiClient, ApiEndpoint,
    AstergdemResponse, Bbox, ChildrenResponse, CitiesGeoname, CitiesResponse, ContainsResponse,
    CountryCodeResponse, CountryInfoGeoname, CountryInfoResponse, CountrySubvisionCode,
    CountrySubvisionResponse, Earthquake, EarthquakesResponse, FindNearbyByPoisOsmResponse,
    FindNearbyByWeatherResponse, FindNearbyByWikipediaResponse, FindNearbyPlaceResponse,
    FindNearbyPostalCodesResponse, FindNearbyResponse, FindNearbyStreetsOSMResponse,
    GeoCodeAddress, GeoCodeAddressResponse, GeoNamesApi, Geoname, GeonameHierarchy,
    GeonameNearbyPlace, GetResponse, Gtopo30Response, HierarchyResponse, NeighboursGeoname,
    NeighboursResponse, Ocean, OceanResponse, Poi, PostalCode, PostalCodeCountryInfoGeoname,
    PostalCodeCountryInfoResponse, PostalCodeFindNearby, PostalCodeLookup,
    PostalCodeLookupResponse, PostalCodeSearchResponse, StreetNameLookupAddress,
    StreetNameLookupResponse, StreetSegment, Timezone, WeatherObservation, WikipediaGeoname,
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
fn call_api_find_nearby_by_weather() {
    let client = ApiClient::new(GeoNamesApi::FindNearByWeather, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "43");
    params.insert("lng", "-2");

    let result: FindNearbyByWeatherResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = FindNearbyByWeatherResponse {
        weather_observation: WeatherObservation {
            elevation: 8,
            lng: -1.8,
            observation: "LESO 212000Z 29009KT 210V360 9000 BKN025 OVC045 14/10 Q1031".to_string(),
            icao: "LESO".to_string(),
            clouds: "broken clouds".to_string(),
            dew_point: "10".to_string(),
            clouds_code: "BKN".to_string(),
            datetime: "2023-12-21 13:00:00".to_string(),
            country_code: "ES".to_string(),
            temperature: "14".to_string(),
            humidity: 76.0,
            station_name: "San Sebastian / Fuenterrabia".to_string(),
            weather_condition: "n/a".to_string(),
            wind_direction: 240,
            hecto_pasc_altimeter: 1030,
            wind_speed: "08".to_string(),
            lat: 43.35,
        },
    };
    // Weather is dynamic
    assert_eq!(
        result.weather_observation.station_name,
        expected_result.weather_observation.station_name
    );
}

#[test]
fn call_api_find_nearby_by_wikipedia() {
    let client = ApiClient::new(GeoNamesApi::FindNearbyWikipedia, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "47");
    params.insert("lng", "9");
    params.insert("maxRows", "1");

    let result: FindNearbyByWikipediaResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = FindNearbyByWikipediaResponse {
            geonames: vec![WikipediaGeoname {
                summary: "The Glärnisch is a mountain massif of the Schwyz Alps, overlooking the valley of the Linth in the Swiss canton of Glarus. It consists of several summits, of which the highest, Bächistock, is 2,915 metres above sea level (...)".to_string(),
                elevation: 2880f64,
                geo_name_id: 2660595,
                feature: "mountain".to_string(),
                lng: 8.99849,
                distance: "0.1853".to_string(),
                country_code: "CH".to_string(),
                rank: 93,
                lang: "en".to_string(),
                title: "Glärnisch".to_string(),
                lat: 46.99869,
                wikipedia_url: "en.wikipedia.org/wiki/Gl%C3%A4rnisch".to_string(),
            }],
        };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_find_nearby_pois_osm() {
    let client = ApiClient::new(GeoNamesApi::FindNearbyPoisOsm, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "37.451");
    params.insert("lng", "-122.18");
    params.insert("maxRows", "1");

    let result: FindNearbyByPoisOsmResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = FindNearbyByPoisOsmResponse {
        poi: Poi {
            lng: "-122.18023".to_string(),
            distance: "0.04".to_string(),
            name: "".to_string(),
            type_class: "amenity".to_string(),
            type_name: "fire_hydrant".to_string(),
            lat: "37.45131".to_string(),
        },
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_address() {
    let client = ApiClient::new(GeoNamesApi::Address, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "52.358");
    params.insert("lng", "4.881");

    let result: AddressResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = AddressResponse {
        address: Address {
            admin_code2: "0363".to_string(),
            source_id: "0363200012086034".to_string(),
            admin_code3: "".to_string(),
            admin_code1: "07".to_string(),
            lng: "4.88132".to_string(),
            distance: "0.02".to_string(),
            house_number: "7".to_string(),
            locality: "Amsterdam".to_string(),
            admin_code4: "".to_string(),
            admin_name2: "Gemeente Amsterdam".to_string(),
            street: "Paulus Potterstraat".to_string(),
            postalcode: "1071 CX".to_string(),
            country_code: "NL".to_string(),
            admin_name1: "North Holland".to_string(),
            lat: "52.35792".to_string(),
        },
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_geo_code_address() {
    let client = ApiClient::new(GeoNamesApi::GeoCodeAddress, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("q", "Museumplein 6 amsterdam");

    let result: GeoCodeAddressResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = GeoCodeAddressResponse {
        address: GeoCodeAddress {
            admin_code2: "0047".to_string(),
            source_id: "0047200000307407".to_string(),
            admin_code3: "".to_string(),
            admin_code1: "04".to_string(),
            lng: "6.87625".to_string(),
            house_number: "10".to_string(),
            locality: "Veendam".to_string(),
            admin_code4: "".to_string(),
            admin_name2: "Veendam Municipality".to_string(),
            street: "Museumplein".to_string(),
            postalcode: "9641 AD".to_string(),
            country_code: "NL".to_string(),
            admin_name1: "Groningen".to_string(),
            lat: "53.10643".to_string(),
        },
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_street_name_lookup() {
    let client = ApiClient::new(GeoNamesApi::StreetNameLookup, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("q", "Museum");
    params.insert("adminCode2", "59350");
    params.insert("postalcode", "6635");

    let result: StreetNameLookupResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = StreetNameLookupResponse {
        address: vec![StreetNameLookupAddress {
            admin_code2: "59350".to_string(),
            admin_code3: "".to_string(),
            admin_code1: "08".to_string(),
            lng: "116.68179".to_string(),
            house_number: "".to_string(),
            locality: "Yalgoo".to_string(),
            admin_code4: "".to_string(),
            admin_name2: "Yalgoo".to_string(),
            street: "Museum Court".to_string(),
            postalcode: "6635".to_string(),
            country_code: "AU".to_string(),
            admin_name1: "Western Australia".to_string(),
            lat: "-28.3414".to_string(),
        }],
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_get() {
    let client = ApiClient::new(GeoNamesApi::Get, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("geonameId", "2746385");

    let result: GetResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = GetResponse {
        timezone: Timezone {
            gmt_offset: 1f64,
            time_zone_id: "Europe/Amsterdam".to_string(),
            dst_offset: 2f64,
        },
        bbox: Bbox {
            east: 4.90836736921802,
            south: 52.9842910190593,
            north: 53.1847360990614,
            west: 4.70705799730483,
            accuracy_level: 0,
        },
        ascii_name: "Texel".to_string(),
        astergdem: 7f64,
        country_id: "2750405".to_string(),
        fcl: "T".to_string(),
        srtm3: -1f64,
        admin_id2: "2746383".to_string(),
        country_code: "NL".to_string(),
        admin_codes1: AdminCodes1 {
            iso3166_2: "NH".to_string(),
        },
        admin_id1: "2749879".to_string(),
        lat: "53.08333".to_string(),
        fcode: "ISL".to_string(),
        continent_code: "EU".to_string(),
        admin_code2: "0448".to_string(),
        admin_code1: "07".to_string(),
        lng: "4.83333".to_string(),
        geoname_id: 2746385,
        toponym_name: "Texel".to_string(),
        population: 0,
        wikipedia_url: "en.wikipedia.org/wiki/Texel".to_string(),
        admin_name5: "".to_string(),
        admin_name4: "".to_string(),
        admin_name3: "".to_string(),
        alternate_names: vec![
                AlternateName { name: "텍셀".to_string(), lang: Some("ko".to_string()) },
        AlternateName { name: "https://en.wikipedia.org/wiki/Texel".to_string(), lang: Some("link".to_string()) },
        AlternateName{
          name: "https://ru.wikipedia.org/wiki/%D0%A2%D0%B5%D0%BA%D1%81%D0%B5%D0%BB_%28%D0%BE%D1%81%D1%82%D1%80%D0%BE%D0%B2%29".to_string(),
          lang: Some("link".to_string())
        },
        AlternateName { name: "Q47009672".to_string(), lang: Some("wkdt".to_string()) },
        AlternateName { name: "Tekselis".to_string(), lang: Some("lt".to_string()) },
        AlternateName { name: "Tessel".to_string(), lang: Some("fy".to_string()) },
        AlternateName { name: "Tessel".to_string(), lang: Some("li".to_string()) },
        AlternateName { name: "Tessel".to_string(), lang: None },
        AlternateName { name: "Texel".to_string(), lang: Some("ca".to_string()) },
        AlternateName { name: "Texel".to_string(), lang: Some("de".to_string()) },
        AlternateName { name: "Texel".to_string(), lang: Some("en".to_string()) },
        AlternateName { name: "Texel".to_string(), lang: Some("eo".to_string()) },
        AlternateName { name: "Texel".to_string(), lang: Some("es".to_string()) },
        AlternateName { name: "Texel".to_string(), lang: Some("fr".to_string()) },
        AlternateName { name: "Texel".to_string(), lang: Some("id".to_string()) },
        AlternateName { name: "Texel".to_string(), lang: Some("it".to_string()) },
        AlternateName { name: "Texel".to_string(), lang: Some("nl".to_string()) },
        AlternateName { name: "Texel".to_string(), lang: Some("sv".to_string()) },
        AlternateName { name: "Texel".to_string(), lang: None },
        AlternateName { name: "Тексел".to_string(), lang: Some("ru".to_string()) },
        AlternateName { name: "Тесел".to_string(), lang: Some("uk".to_string()) },
        AlternateName{
          name: "ტექსელი".to_string(),
          lang: Some("ka".to_string())
        },
        AlternateName { name: "Թեսել".to_string(), lang: Some("hy".to_string()) },
        AlternateName { name: "טסל".to_string(), lang: Some("he".to_string()) },
        AlternateName { name: "تيكسل".to_string(), lang: Some("ar".to_string()) },
        AlternateName{
          name: "เทกเซล".to_string(),
          lang: Some("th".to_string())
        },
        AlternateName { name: "テセル".to_string(), lang: Some("ja".to_string()) },
        AlternateName { name: "特塞尔".to_string(), lang: Some("zh".to_string()) }
            ],
        admin_name2: "Texel Municipality".to_string(),
        name: "Texel".to_string(),
        fcl_name: "mountain,hill,rock,... ".to_string(),
        country_name: "The Netherlands".to_string(),
        fcode_name: "island".to_string(),
        admin_name1: "North Holland".to_string(),
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_gtopo30() {
    let client = ApiClient::new(GeoNamesApi::Gtopo30, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "47.01");
    params.insert("lng", "10.02");

    let result: Gtopo30Response = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = Gtopo30Response {
        lng: 10.02,
        gtopo30: 2053f64,
        lat: 47.01,
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_hierarchy() {
    let client = ApiClient::new(GeoNamesApi::Hierarchy, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("geonameId", "2657896");

    let result: HierarchyResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = HierarchyResponse {
        geonames: vec![
            GeonameHierarchy {
                lng: "0".to_string(),
                geoname_id: 6295630,
                name: "Earth".to_string(),
                fcl_name: "parks,area, ...".to_string(),
                toponym_name: "Earth".to_string(),
                fcode_name: "area".to_string(),
                admin_name1: "".to_string(),
                lat: "0".to_string(),
                fcl: "L".to_string(),
                fcode: "AREA".to_string(),
                population: 6814400000,
                admin_code1: None,
                admin_codes1: None,
                country_id: None,
                country_name: None,
                country_code: None,
            },
            GeonameHierarchy {
                lng: "9.14062".to_string(),
                geoname_id: 6255148,
                name: "Europe".to_string(),
                fcl_name: "parks,area, ...".to_string(),
                toponym_name: "Europe".to_string(),
                fcode_name: "continent".to_string(),
                admin_name1: "".to_string(),
                lat: "48.69096".to_string(),
                fcl: "L".to_string(),
                fcode: "CONT".to_string(),
                population: 741000000,
                admin_code1: None,
                admin_codes1: None,
                country_id: None,
                country_name: None,
                country_code: None,
            },
            GeonameHierarchy {
                admin_code1: Some("00".to_string()),
                lng: "8.01427".to_string(),
                geoname_id: 2658434,
                toponym_name: "Switzerland".to_string(),
                country_id: Some("2658434".to_string()),
                fcl: "A".to_string(),
                population: 8516543,
                country_code: Some("CH".to_string()),
                name: "Switzerland".to_string(),
                fcl_name: "country, state, region,...".to_string(),
                country_name: Some("Switzerland".to_string()),
                fcode_name: "independent political entity".to_string(),
                admin_name1: "".to_string(),
                lat: "47.00016".to_string(),
                fcode: "PCLI".to_string(),
                admin_codes1: None,
            },
            GeonameHierarchy {
                admin_code1: Some("ZH".to_string()),
                lng: "8.66667".to_string(),
                geoname_id: 2657895,
                toponym_name: "Kanton Zürich".to_string(),
                country_id: Some("2658434".to_string()),
                fcl: "A".to_string(),
                population: 1553423,
                country_code: Some("CH".to_string()),
                name: "Zurich".to_string(),
                fcl_name: "country, state, region,...".to_string(),
                admin_codes1: Some(AdminCodes1 {
                    iso3166_2: "ZH".to_string(),
                }),
                country_name: Some("Switzerland".to_string()),
                fcode_name: "first-order administrative division".to_string(),
                admin_name1: "Zurich".to_string(),
                lat: "47.41667".to_string(),
                fcode: "ADM1".to_string(),
            },
            GeonameHierarchy {
                admin_code1: Some("ZH".to_string()),
                lng: "8.54323".to_string(),
                geoname_id: 6458798,
                toponym_name: "Bezirk Zürich".to_string(),
                country_id: Some("2658434".to_string()),
                fcl: "A".to_string(),
                population: 421878,
                country_code: Some("CH".to_string()),
                name: "Zürich District".to_string(),
                fcl_name: "country, state, region,...".to_string(),
                admin_codes1: Some(AdminCodes1 {
                    iso3166_2: "ZH".to_string(),
                }),
                country_name: Some("Switzerland".to_string()),
                fcode_name: "second-order administrative division".to_string(),
                admin_name1: "Zurich".to_string(),
                lat: "47.3711".to_string(),
                fcode: "ADM2".to_string(),
            },
            GeonameHierarchy {
                admin_code1: Some("ZH".to_string()),
                lng: "8.53071".to_string(),
                geoname_id: 7287650,
                toponym_name: "Zürich".to_string(),
                country_id: Some("2658434".to_string()),
                fcl: "A".to_string(),
                population: 420217,
                country_code: Some("CH".to_string()),
                name: "Zurich".to_string(),
                fcl_name: "country, state, region,...".to_string(),
                admin_codes1: Some(AdminCodes1 {
                    iso3166_2: "ZH".to_string(),
                }),
                country_name: Some("Switzerland".to_string()),
                fcode_name: "third-order administrative division".to_string(),
                admin_name1: "Zurich".to_string(),
                lat: "47.38283".to_string(),
                fcode: "ADM3".to_string(),
            },
            GeonameHierarchy {
                admin_code1: Some("ZH".to_string()),
                lng: "8.55".to_string(),
                geoname_id: 2657896,
                toponym_name: "Zürich".to_string(),
                country_id: Some("2658434".to_string()),
                fcl: "P".to_string(),
                population: 341730,
                country_code: Some("CH".to_string()),
                name: "Zurich".to_string(),
                fcl_name: "city, village,...".to_string(),
                admin_codes1: Some(AdminCodes1 {
                    iso3166_2: "ZH".to_string(),
                }),
                country_name: Some("Switzerland".to_string()),
                fcode_name: "seat of a first-order administrative division".to_string(),
                admin_name1: "Zurich".to_string(),
                lat: "47.36667".to_string(),
                fcode: "PPLA".to_string(),
            },
        ],
    };
    assert_eq!(result, expected_result);
}

#[test]
fn call_api_neighbours() {
    let client = ApiClient::new(GeoNamesApi::Neighbours, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("geonameId", "2658434");

    let result: NeighboursResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_response = NeighboursResponse {
        total_results_count: 5,
        geonames: vec![
            NeighboursGeoname {
                admin_code1: "00".to_string(),
                lng: "13.33333".to_string(),
                geoname_id: 2782113,
                toponym_name: "Republic of Austria".to_string(),
                country_id: "2782113".to_string(),
                fcl: "A".to_string(),
                population: 8847037,
                country_code: "AT".to_string(),
                name: "Austria".to_string(),
                fcl_name: "country, state, region,...".to_string(),
                country_name: "Austria".to_string(),
                fcode_name: "independent political entity".to_string(),
                admin_name1: "".to_string(),
                lat: "47.33333".to_string(),
                fcode: "PCLI".to_string(),
            },
            NeighboursGeoname {
                admin_code1: "00".to_string(),
                lng: "2".to_string(),
                geoname_id: 3017382,
                toponym_name: "Republic of France".to_string(),
                country_id: "3017382".to_string(),
                fcl: "A".to_string(),
                population: 66987244,
                country_code: "FR".to_string(),
                name: "France".to_string(),
                fcl_name: "country, state, region,...".to_string(),
                country_name: "France".to_string(),
                fcode_name: "independent political entity".to_string(),
                admin_name1: "".to_string(),
                lat: "46".to_string(),
                fcode: "PCLI".to_string(),
            },
            NeighboursGeoname {
                admin_code1: "00".to_string(),
                lng: "10.5".to_string(),
                geoname_id: 2921044,
                toponym_name: "Federal Republic of Germany".to_string(),
                country_id: "2921044".to_string(),
                fcl: "A".to_string(),
                population: 82927922,
                country_code: "DE".to_string(),
                name: "Germany".to_string(),
                fcl_name: "country, state, region,...".to_string(),
                country_name: "Germany".to_string(),
                fcode_name: "independent political entity".to_string(),
                admin_name1: "".to_string(),
                lat: "51.5".to_string(),
                fcode: "PCLI".to_string(),
            },
            NeighboursGeoname {
                admin_code1: "00".to_string(),
                lng: "12.83333".to_string(),
                geoname_id: 3175395,
                toponym_name: "Italian Republic".to_string(),
                country_id: "3175395".to_string(),
                fcl: "A".to_string(),
                population: 60431283,
                country_code: "IT".to_string(),
                name: "Italy".to_string(),
                fcl_name: "country, state, region,...".to_string(),
                country_name: "Italy".to_string(),
                fcode_name: "independent political entity".to_string(),
                admin_name1: "".to_string(),
                lat: "42.83333".to_string(),
                fcode: "PCLI".to_string(),
            },
            NeighboursGeoname {
                admin_code1: "00".to_string(),
                lng: "9.53333".to_string(),
                geoname_id: 3042058,
                toponym_name: "Principality of Liechtenstein".to_string(),
                country_id: "3042058".to_string(),
                fcl: "A".to_string(),
                population: 37910,
                country_code: "LI".to_string(),
                name: "Liechtenstein".to_string(),
                fcl_name: "country, state, region,...".to_string(),
                country_name: "Liechtenstein".to_string(),
                fcode_name: "independent political entity".to_string(),
                admin_name1: "".to_string(),
                lat: "47.16667".to_string(),
                fcode: "PCLI".to_string(),
            },
        ],
    };
    assert_eq!(result, expected_response);
}

#[test]
fn call_api_ocean() {
    let client = ApiClient::new(GeoNamesApi::Ocean, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("lat", "40.7834");
    params.insert("lng", "-43.96635");

    let result: OceanResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_response = OceanResponse {
        ocean: Ocean {
            distance: "0".to_string(),
            geoname_id: 3411923,
            name: "North Atlantic Ocean".to_string(),
        },
    };
    assert_eq!(result, expected_response);
}

#[test]
fn call_api_postal_code_country_info() {
    let client = ApiClient::new(GeoNamesApi::PostalCodeCountryInfo, USERNAME, None);

    let result: PostalCodeCountryInfoResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(None))
        .unwrap();

    let expected_result = PostalCodeCountryInfoResponse {
        geonames: vec![PostalCodeCountryInfoGeoname {
            num_postal_codes: 7,
            max_postal_code: "AD700".to_string(),
            country_code: "AD".to_string(),
            min_postal_code: "AD100".to_string(),
            country_name: "Andorra".to_string(),
        }],
    };

    assert_eq!(result.geonames[0], expected_result.geonames[0]);
}

#[test]
fn call_api_postal_code_lookup() {
    let client = ApiClient::new(GeoNamesApi::PostalCodeLookup, USERNAME, None);
    let mut params = HashMap::new();
    params.insert("postalcode", "6600");
    params.insert("country", "AT");
    params.insert("maxRows", "1");

    let result: PostalCodeLookupResponse = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client.call_api(Some(params)))
        .unwrap();

    let expected_result = PostalCodeLookupResponse {
        postalcodes: vec![PostalCodeLookup {
            admin_code2: "708".to_string(),
            admin_code3: "70820".to_string(),
            admin_name3: "Lechaschau".to_string(),
            admin_code1: "07".to_string(),
            admin_name2: "Politischer Bezirk Reutte".to_string(),
            lng: 10.706520080566406,
            country_code: "AT".to_string(),
            postalcode: "6600".to_string(),
            admin_name1: "Tirol".to_string(),
            place_name: "Lechaschau".to_string(),
            lat: 47.488035007826824,
        }],
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
