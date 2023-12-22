use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GeoNamesApi {
    Astergdem,
    Children,
    Cities,
    Contains,
    CountryCode,
    CountryInfo,
    CountrySubdivision,
    Earthquakes,
    FindNearby,
    FindNearbyPlaceName,
    FindNearbyPostalCodes,
    #[serde(rename = "findNearbyStreetsOSM")]
    FindNearbyStreetsOsm,
    FindNearByWeather,
    FindNearbyWikipedia,
    FindNearestIntersectionOsm,
    #[serde(rename = "findNearbyPOIsOSM")]
    FindNearbyPoisOsm,
    Address,
    GeoCodeAddress,
    StreetNameLookup,
    Get,
    Gtopo30,
    Hierarchy,
    Neighbours,
    Ocean,
    PostalCodeCountryInfo,
    PostalCodeLookup,
    PostalCodeSearch,
    Search,
    Siblings,
    Srtm1,
    Srtm3,
    Timezone,
    Weather,
    WeatherIcao,
    WikipediaBoundingBox,
    WikipediaSearch,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Format {
    Json,
}

lazy_static::lazy_static! {
    pub static ref POSTAL_CODE_SEARCH_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("postalcode", vec![]);
        map.insert("postalcode_startsWith", vec![]);
        map.insert("placename", vec![]);
        map.insert("placename_startsWith", vec![]);
        map.insert("country", vec![]);
        map.insert("countryBias", vec![]);
        map.insert("maxRows", vec![]);
        map.insert("style", vec!["SHORT", "MEDIUM", "LONG", "FULL"]);
        map.insert("operator", vec!["AND", "OR"]);
        map.insert("charset", vec![]);
        map.insert("isReduced", vec!["true", "false"]);
        map
    };
    pub static ref POSTAL_CODE_LOOKUP_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("postalcode", vec![]);
        map.insert("country", vec![]);
        map.insert("maxRows", vec![]);
        map.insert("charset", vec![]);
        map
    };
    pub static ref FIND_NEARBY_POSTAL_CODES_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map.insert("radius", vec![]);
        map.insert("maxRows", vec![]);
        map.insert("style", vec!["SHORT", "MEDIUM", "LONG", "FULL"]);
        map.insert("country", vec![]);
        map.insert("localCountry", vec![]);
        map.insert("isReduced", vec!["true", "false"]);
        map.insert("postalCode", vec![]);
        map
    };
    pub static ref FIND_NEARBY_PLACE_NAME_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map.insert("lang", vec![]);
        map.insert("maxRows", vec![]);
        map.insert("radius", vec![]);
        map.insert("style", vec!["SHORT", "MEDIUM", "LONG", "FULL"]);
        map.insert("localCountry", vec![]);
        map.insert("cities", vec!["cities1000", "cities5000", "cities15000"]);
        map
    };
    pub static ref FIND_NEARBY_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map.insert("featureClass", vec![]);
        map.insert("featureCode", vec![]);
        map.insert("radius", vec![]);
        map.insert("maxRows", vec![]);
        map.insert("style", vec!["SHORT", "MEDIUM", "LONG", "FULL"]);
        map
    };
    pub static ref COUNTRY_INFO_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("country", vec![]);
        map.insert("lang", vec![]);
        map
    };
    pub static ref COUNTRY_CODE_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map.insert("type", vec![]);
        map.insert("lang", vec![]);
        map.insert("radius", vec![]);
        map
    };
    pub static ref COUNTRY_SUBDIVISION_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map.insert("lang", vec![]);
        map.insert("radius", vec![]);
        map.insert("level", vec![]);
        map
    };
    pub static ref GET_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("geonameId", vec![]);
        map.insert("lang", vec![]);
        map.insert("style", vec![]);
        map
    };
    pub static ref OCEAN_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map.insert("radius", vec![]);
        map
    };
    pub static ref SRTM1_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map
    };
    pub static ref SRTM3_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map
    };
    pub static ref ASTERGDEM_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("postalcodes", vec![]);
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map
    };
    pub static ref GTOPO30_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map
    };
    pub static ref TIMEZONE_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map.insert("radius", vec![]);
        map.insert("lang", vec![]);
        map.insert("date", vec![]);
        map
    };
    pub static ref CHILDREN_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("geonameId", vec![]);
        map.insert("maxRows", vec![]);
        map.insert("hierarchy", vec!["tourism", "geography", "dependency"]);
        map
    };
    pub static ref CITIES_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("north", vec![]);
        map.insert("south", vec![]);
        map.insert("east", vec![]);
        map.insert("west", vec![]);
        map.insert("maxRows", vec![]);
        map
    };
    pub static ref CONTAINS_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("geonameId", vec![]);
        map.insert("featureClass", vec![]);
        map.insert("featureCode", vec![]);
        map.insert("maxRows", vec![]);
        map
    };
    pub static ref EARTHQUAKES_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("north", vec![]);
        map.insert("south", vec![]);
        map.insert("east", vec![]);
        map.insert("west", vec![]);
        map.insert("date", vec![]);
        map.insert("minMagnitude", vec![]);
        map.insert("maxRows", vec![]);
        map
    };
    pub static ref FIND_NEARBY_STREETS_0SM_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map.insert("radius", vec![]);
        map.insert("maxRows", vec![]);
        map
    };
    pub static ref FIND_NEARBY_BY_WEATHER_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map.insert("radius", vec![]);
        map
    };
    pub static ref FIND_NEARBY_BY_WIKIPEDIA_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lang", vec![]);
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map.insert("radius", vec![]);
        map.insert("maxRows", vec![]);
        map.insert("country", vec![]);
        map.insert("postalCode", vec![]);
        map.insert("length", vec![]);
        map
    };
    pub static ref FIND_NEARBY_POIS_OSM_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map.insert("radius", vec![]);
        map.insert("maxRows", vec![]);
        map
    };
    pub static ref ADDRESS_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
        map.insert("radius", vec![]);
        map
    };
    pub static ref GEO_CODE_ADDRESS_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("q", vec![]);
        map.insert("country", vec![]);
        map.insert("postalcode", vec![]);
        map
    };
    pub static ref STREET_NAME_LOOKUP_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("q", vec![]);
        map.insert("country", vec![]);
        map.insert("postalcode", vec![]);
        map.insert("adminCode1", vec![]);
        map.insert("adminCode2", vec![]);
        map.insert("adminCode3", vec![]);
        map.insert("isUniqueStreetName", vec![]);
        map
    };
    pub static ref HIERARCHY_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("geonameId", vec![]);
        map
    };
    pub static ref NEIGHBOURS_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("geonameId", vec![]);
        map.insert("country", vec![]);
        map
    };
    pub static ref SEARCH_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("q", vec![]);
        map.insert("name", vec![]);
        map.insert("name_equals", vec![]);
        map.insert("name_startsWith", vec![]);
        map.insert("maxRows", vec![]);
        map.insert("startRow", vec![]);
        map.insert("country", vec![]);
        map.insert("countryBias", vec![]);
        map.insert("continentCode", vec![]);
        map.insert("adminCode1", vec![]);
        map.insert("adminCode2", vec![]);
        map.insert("adminCode3", vec![]);
        map.insert("adminCode4", vec![]);
        map.insert("adminCode5", vec![]);
        map.insert("featureClass", vec![]);
        map.insert("featureCode", vec![]);
        map.insert("cities", vec![]);
        map.insert("lang", vec![]);
        map.insert("type", vec![]);
        map.insert("style", vec![]);
        map.insert("isNameRequired", vec![]);
        map.insert("tag", vec![]);
        map.insert("operator", vec![]);
        map.insert("charset", vec![]);
        map.insert("fuzzy", vec![]);
        map.insert("north", vec![]);
        map.insert("south", vec![]);
        map.insert("east", vec![]);
        map.insert("west", vec![]);
        map.insert("searchlang", vec![]);
        map.insert("orderby", vec![]);
        map.insert("inclBbox", vec![]);
        map
    };
    pub static ref SIBLINGS_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("geonameId", vec![]);
        map
    };
    pub static ref WEATHER_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("north", vec![]);
        map.insert("south", vec![]);
        map.insert("east", vec![]);
        map.insert("west", vec![]);
        map.insert("maxRows", vec![]);
        map
    };
    pub static ref WEATHER_ICAO_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("ICAO", vec![]);
        map
    };
}

pub const BASE_URI: &str = "https://secure.geonames.org/";
pub const BASE_URI_COMMERCIAL: &str = "https://secure.geonames.net/";
