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
    ExtendedFindNearby,
    FindNearby,
    FindNearbyPlaceName,
    FindNearbyPostalCodes,
    FindNearbyStreets, // only USA
    FindNearbyStreetsOsm,
    FindNearByWeather,
    FindNearbyWikipedia,
    FindNearestAddress,      // only USA
    FindNearestIntersection, // only USA
    FindNearestIntersectionOsm,
    FindNearbyPoisOsm,
    Geocode, // only USA
    Get,
    Gtopo30,
    Hierarchy,
    Neighbourhood, // only USA
    Neighbours,
    Ocean,
    PostalCodeCountryInfo,
    PostalCodeLookup,
    PostalCodeSearch,
    RssToGeo,
    Search,
    Siblings,
    Srtm1,
    Srtm3,
    Timezone,
    Weather,
    WeatherIcao,
    WikipediaBoundingBox,
    WikipediaSearch,
    Address,
    GeoCodeAddress,
    StreetNameLookup,
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
    pub static ref EXTENDED_FIND_NEARBY_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
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
    pub static ref NEIGHBOURHOOD_PARAMS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("lat", vec![]);
        map.insert("lng", vec![]);
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
}

pub const BASE_URI: &str = "https://secure.geonames.org/";
pub const BASE_URI_COMMERCIAL: &str = "https://secure.geonames.net/";
