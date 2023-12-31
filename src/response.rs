use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CountryCodeResponse {
    pub languages: String,
    pub distance: String,
    pub country_code: String,
    pub country_name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PostalCodeSearchResponse {
    pub postal_codes: Vec<PostalCode>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PostalCode {
    pub admin_code_1: String,
    pub admin_code_2: String,
    pub admin_name_1: String,
    pub admin_name_2: String,
    pub lng: f64,
    pub country_code: String,
    pub postal_code: String,
    #[serde(rename = "ISO3166-2")]
    pub iso: String,
    pub place_name: String,
    pub lat: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AstergdemResponse {
    pub lng: f64,
    pub lat: f64,
    pub astergdem: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChildrenResponse {
    pub total_results_count: i32,
    pub geonames: Vec<Geoname>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Geoname {
    pub admin_code_1: String,
    pub lng: String,
    pub geoname_id: i32,
    pub toponym_name: String,
    pub country_id: String,
    pub admin_codes1: AdminCodes1,
    pub country_name: String,
    pub fcode_name: String,
    pub admin_name1: String,
    pub lat: String,
    pub fcode: String,
    pub fcl: String,
    pub population: i32,
    pub country_code: String,
    pub name: String,
    pub fcl_name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct AdminCodes1 {
    pub iso3166_2: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CitiesResponse {
    pub geonames: Vec<CitiesGeoname>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CitiesGeoname {
    pub lng: f64,
    pub geoname_id: i32,
    pub toponym_name: String,
    pub fcode_name: String,
    pub lat: f64,
    pub fcode: String,
    pub fcl: String,
    pub population: i32,
    pub countrycode: String,
    pub name: String,
    pub fcl_name: String,
    pub wikipedia: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContainsResponse {
    pub geonames: Vec<Geoname>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CountryInfoResponse {
    pub geonames: Vec<CountryInfoGeoname>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CountryInfoGeoname {
    pub continent: String,
    pub capital: String,
    pub languages: String,
    pub geoname_id: i32,
    pub south: f64,
    pub iso_alpha3: String,
    pub north: f64,
    pub fips_code: String,
    pub population: String,
    pub east: f64,
    pub iso_numeric: String,
    pub area_in_sq_km: String,
    pub country_code: String,
    pub west: f64,
    pub country_name: String,
    pub postal_code_format: String,
    pub continent_name: String,
    pub currency_code: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CountrySubvisionResponse {
    pub codes: Vec<CountrySubvisionCode>,
    pub admin_code1: String,
    pub distance: f64,
    pub country_code: String,
    pub country_name: String,
    pub admin_name1: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CountrySubvisionCode {
    pub code: String,
    pub level: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EarthquakesResponse {
    pub earthquakes: Vec<Earthquake>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Earthquake {
    pub datetime: String,
    pub depth: f64,
    pub lng: f64,
    pub src: String,
    pub eqid: String,
    pub magnitude: f64,
    pub lat: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FindNearbyResponse {
    pub geonames: Vec<Geoname>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FindNearbyPlaceResponse {
    pub geonames: Vec<GeonameNearbyPlace>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GeonameNearbyPlace {
    pub admin_code_1: String,
    pub lng: String,
    pub geoname_id: i32,
    pub toponym_name: String,
    pub country_id: String,
    pub admin_codes1: AdminCodes1,
    pub country_name: String,
    pub fcode_name: String,
    pub admin_name1: String,
    pub lat: String,
    pub fcode: String,
    pub fcl: String,
    pub population: i32,
    pub country_code: String,
    pub name: String,
    pub fcl_name: String,
    pub distance: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FindNearbyPostalCodesResponse {
    pub postal_codes: Vec<PostalCodeFindNearby>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PostalCodeFindNearby {
    pub admin_code1: String,
    pub admin_code2: String,
    pub admin_code3: String,
    pub admin_name1: String,
    pub admin_name2: String,
    pub admin_name3: String,
    pub lng: f64,
    pub distance: String,
    pub country_code: String,
    pub postal_code: String,
    pub place_name: String,
    pub lat: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FindNearbyStreetsOSMResponse {
    pub street_segment: StreetSegment,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StreetSegment {
    pub way_id: String,
    pub distance: String,
    pub line: String,
    pub country_code: String,
    pub name: String,
    pub highway: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FindNearbyByWeatherResponse {
    pub weather_observation: WeatherObservation,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeatherObservation {
    pub elevation: i32,
    pub lng: f64,
    pub observation: String,
    #[serde(rename = "ICAO")]
    pub icao: String,
    pub clouds: String,
    pub dew_point: String,
    pub clouds_code: String,
    pub datetime: String,
    pub country_code: String,
    pub temperature: String,
    pub humidity: f64,
    pub station_name: String,
    pub weather_condition: String,
    pub wind_direction: i32,
    pub hecto_pasc_altimeter: i32,
    pub wind_speed: String,
    pub lat: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FindNearbyByWikipediaResponse {
    pub geonames: Vec<WikipediaGeoname>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WikipediaGeoname {
    pub summary: String,
    pub elevation: f64,
    pub geo_name_id: i32,
    pub feature: String,
    pub lng: f64,
    pub distance: String,
    pub country_code: String,
    pub rank: i32,
    pub lang: String,
    pub title: String,
    pub lat: f64,
    pub wikipedia_url: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FindNearbyByPoisOsmResponse {
    pub poi: Poi,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Poi {
    pub lng: String,
    pub distance: String,
    pub name: String,
    pub type_class: String,
    pub type_name: String,
    pub lat: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AddressResponse {
    pub address: Address,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub admin_code2: String,
    pub source_id: String,
    pub admin_code3: String,
    pub admin_code1: String,
    pub lng: String,
    pub distance: String,
    pub house_number: String,
    pub locality: String,
    pub admin_code4: String,
    pub admin_name2: String,
    pub street: String,
    pub postalcode: String,
    pub country_code: String,
    pub admin_name1: String,
    pub lat: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GeoCodeAddressResponse {
    pub address: GeoCodeAddress,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GeoCodeAddress {
    pub admin_code2: String,
    pub source_id: String,
    pub admin_code3: String,
    pub admin_code1: String,
    pub lng: String,
    pub house_number: String,
    pub locality: String,
    pub admin_code4: String,
    pub admin_name2: String,
    pub street: String,
    pub postalcode: String,
    pub country_code: String,
    pub admin_name1: String,
    pub lat: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StreetNameLookupResponse {
    pub address: Vec<StreetNameLookupAddress>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StreetNameLookupAddress {
    pub admin_code2: String,
    pub admin_code3: String,
    pub admin_code1: String,
    pub lng: String,
    pub house_number: String,
    pub locality: String,
    pub admin_code4: String,
    pub admin_name2: String,
    pub street: String,
    pub postalcode: String,
    pub country_code: String,
    pub admin_name1: String,
    pub lat: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetResponse {
    pub timezone: Timezone,
    pub bbox: Bbox,
    pub ascii_name: String,
    pub astergdem: f64,
    pub country_id: String,
    pub fcl: String,
    pub srtm3: f64,
    pub admin_id2: String,
    pub country_code: String,
    pub admin_codes1: AdminCodes1,
    pub admin_id1: String,
    pub lat: String,
    pub fcode: String,
    pub continent_code: String,
    pub admin_code2: String,
    pub admin_code1: String,
    pub lng: String,
    pub geoname_id: i32,
    pub toponym_name: String,
    pub population: i32,
    #[serde(rename = "wikipediaURL")]
    pub wikipedia_url: String,
    pub admin_name5: String,
    pub admin_name4: String,
    pub admin_name3: String,
    pub alternate_names: Vec<AlternateName>,
    pub admin_name2: String,
    pub name: String,
    pub fcl_name: String,
    pub country_name: String,
    pub fcode_name: String,
    pub admin_name1: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Timezone {
    pub gmt_offset: f64,
    pub time_zone_id: String,
    pub dst_offset: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bbox {
    pub east: f64,
    pub south: f64,
    pub north: f64,
    pub west: f64,
    pub accuracy_level: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AlternateName {
    pub name: String,
    pub lang: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Gtopo30Response {
    pub lng: f64,
    pub gtopo30: f64,
    pub lat: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HierarchyResponse {
    pub geonames: Vec<GeonameHierarchy>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GeonameHierarchy {
    pub lng: String,
    pub geoname_id: i32,
    pub name: String,
    pub fcl_name: String,
    pub toponym_name: String,
    pub fcode_name: String,
    pub admin_name1: String,
    pub lat: String,
    pub fcl: String,
    pub fcode: String,
    pub population: i64,
    pub admin_code1: Option<String>,
    pub admin_codes1: Option<AdminCodes1>,
    pub country_id: Option<String>,
    pub country_name: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NeighboursResponse {
    pub total_results_count: i32,
    pub geonames: Vec<NeighboursGeoname>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NeighboursGeoname {
    pub admin_code1: String,
    pub lng: String,
    pub geoname_id: i32,
    pub toponym_name: String,
    pub country_id: String,
    pub fcl: String,
    pub population: i64,
    pub country_code: String,
    pub name: String,
    pub fcl_name: String,
    pub country_name: String,
    pub fcode_name: String,
    pub admin_name1: String,
    pub lat: String,
    pub fcode: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OceanResponse {
    pub ocean: Ocean,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ocean {
    pub distance: String,
    pub geoname_id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PostalCodeCountryInfoResponse {
    pub geonames: Vec<PostalCodeCountryInfoGeoname>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PostalCodeCountryInfoGeoname {
    pub num_postal_codes: i32,
    pub max_postal_code: String,
    pub country_code: String,
    pub min_postal_code: String,
    pub country_name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PostalCodeLookupResponse {
    pub postalcodes: Vec<PostalCodeLookup>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PostalCodeLookup {
    pub admin_code2: String,
    pub admin_code3: String,
    pub admin_name3: String,
    pub admin_code1: String,
    pub admin_name2: String,
    pub lng: f64,
    pub country_code: String,
    pub postalcode: String,
    pub admin_name1: String,
    pub place_name: String,
    pub lat: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub total_results_count: i32,
    pub geonames: Vec<Geoname>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SiblingsResponse {
    pub total_results_count: i32,
    pub geonames: Vec<SiblingGeoname>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SiblingGeoname {
    pub admin_code1: String,
    pub lng: String,
    pub geoname_id: i32,
    pub toponym_name: String,
    pub country_id: String,
    pub fcl: String,
    pub population: i32,
    pub country_code: String,
    pub name: String,
    pub fcl_name: String,
    pub country_name: String,
    pub fcode_name: String,
    pub admin_name1: String,
    pub lat: String,
    pub fcode: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Srtm1Response {
    pub srtm1: i32,
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Srtm3Response {
    pub srtm3: i32,
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TimezoneResponse {
    pub sunrise: String,
    pub lng: f64,
    pub country_code: String,
    pub gmt_offset: i32,
    pub raw_offset: i32,
    pub sunset: String,
    pub timezone_id: String,
    pub dst_offset: i32,
    pub country_name: String,
    pub time: String,
    pub lat: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
    pub weather_observations: Vec<Weather>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub lng: f64,
    pub observation: String,
    #[serde(rename = "ICAO")]
    pub icao: String,
    pub clouds: String,
    pub dew_point: String,
    pub datetime: String,
    pub temperature: String,
    pub humidity: i32,
    pub station_name: String,
    pub weather_condition: String,
    pub wind_direction: i32,
    pub wind_speed: String,
    pub lat: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeatherIcaoResponse {
    pub weather_observation: WeatherIcao,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeatherIcao {
    pub elevation: i32,
    pub lng: f64,
    pub observation: String,
    #[serde(rename = "ICAO")]
    pub icao: String,
    pub clouds: String,
    pub dew_point: String,
    pub clouds_code: String,
    pub datetime: String,
    pub country_code: String,
    pub temperature: String,
    pub humidity: f64,
    pub station_name: String,
    pub weather_condition: String,
    pub wind_direction: i32,
    pub hecto_pasc_altimeter: i32,
    pub wind_speed: String,
    pub lat: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WikipediaBoundingBoxResponse {
    pub geonames: Vec<WikipediaBoundingBoxGeoname>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WikipediaBoundingBoxGeoname {
    pub summary: String,
    pub elevation: i32,
    pub feature: String,
    pub lng: f64,
    pub country_code: String,
    pub rank: i32,
    pub thumbnail_img: String,
    pub lang: String,
    pub title: String,
    pub lat: f64,
    pub wikipedia_url: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WikipediaSearchResponse {
    pub geonames: Vec<WikipediaSearchGeoname>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WikipediaSearchGeoname {
    pub summary: String,
    pub elevation: i32,
    pub geo_name_id: i32,
    pub feature: String,
    pub lng: f64,
    pub country_code: String,
    pub rank: i32,
    pub thumbnail_img: String,
    pub lang: String,
    pub title: String,
    pub lat: f64,
    pub wikipedia_url: String,
}
