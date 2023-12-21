use async_trait::async_trait;
use std::collections::HashMap;

use reqwest::{Client, Url};

use crate::{
    config::{
        GeoNamesApi, ASTERGDEM_PARAMS, BASE_URI, BASE_URI_COMMERCIAL, COUNTRY_CODE_PARAMS,
        COUNTRY_INFO_PARAMS, COUNTRY_SUBDIVISION_PARAMS, FIND_NEARBY_PARAMS,
        FIND_NEARBY_PLACE_NAME_PARAMS, FIND_NEARBY_POSTAL_CODES_PARAMS, GET_PARAMS, GTOPO30_PARAMS,
        NEIGHBOURHOOD_PARAMS, OCEAN_PARAMS, POSTAL_CODE_LOOKUP_PARAMS, POSTAL_CODE_SEARCH_PARAMS,
        SRTM1_PARAMS, SRTM3_PARAMS, TIMEZONE_PARAMS,
    },
    response::ApiResponse,
    ApiError, ADDRESS_PARAMS, CHILDREN_PARAMS, CITIES_PARAMS, CONTAINS_PARAMS, EARTHQUAKES_PARAMS,
    FIND_NEARBY_BY_WEATHER_PARAMS, FIND_NEARBY_BY_WIKIPEDIA_PARAMS, FIND_NEARBY_POIS_OSM_PARAMS,
    FIND_NEARBY_STREETS_0SM_PARAMS, GEO_CODE_ADDRESS_PARAMS, STREET_NAME_LOOKUP_PARAMS,
};

#[async_trait]
pub trait ApiEndpoint {
    fn allowed_params(&self) -> Option<&'static HashMap<&'static str, Vec<&'static str>>>;
    async fn call_api<T: ApiResponse>(
        &self,
        params: Option<HashMap<&'static str, &'static str>>,
    ) -> Result<T, ApiError>;
}

pub struct ApiClient {
    api: GeoNamesApi,
    username: &'static str,
    token: Option<&'static str>,
}

impl ApiClient {
    pub fn new(api: GeoNamesApi, username: &'static str, token: Option<&'static str>) -> Self {
        ApiClient {
            api,
            username,
            token,
        }
    }
}

#[async_trait]
impl ApiEndpoint for ApiClient {
    async fn call_api<T: ApiResponse>(
        &self,
        params: Option<HashMap<&'static str, &'static str>>,
    ) -> Result<T, ApiError> {
        let maybe_allowed_params = self.allowed_params();

        match maybe_allowed_params {
            Some(allowed_params) => {
                if let Some(p) = params.clone() {
                    for (param, value) in p.clone() {
                        if !allowed_params.contains_key(param) {
                            return Err(ApiError::InvalidParams(format!(
                                "Param '{}' not allowed for this API",
                                param
                            )));
                        }

                        let allowed_values = allowed_params[param].clone();
                        if !allowed_values.is_empty() && !allowed_values.contains(&value) {
                            return Err(ApiError::InvalidParams(format!(
                                "Invalid value '{}' for param '{}'",
                                value, param
                            )));
                        }
                    }
                }
            }
            None => {
                if params.is_some() {
                    return Err(ApiError::InvalidParams(format!(
                        "Params for api {:?} should be None",
                        self.api
                    )));
                }
            }
        };

        let api_name = serde_variant::to_variant_name(&self.api).unwrap(); // TODO: Return err

        let base_url = match self.token {
            Some(_) => format!("{}{}{}", BASE_URI_COMMERCIAL, api_name, "JSON"),
            None => format!("{}{}{}", BASE_URI, api_name, "JSON"),
        };

        let mut url = Url::parse(&base_url)
            .map_err(|e| ApiError::UrlParse(format!("Failed to parse URL: {}", e)))?;

        if let Some(p) = params {
            for (param, value) in p {
                url.query_pairs_mut().append_pair(param, value);
            }
        }

        if let Some(token) = self.token {
            url.query_pairs_mut().append_pair("token", token);
        }

        url.query_pairs_mut().append_pair("username", self.username);
        eprintln!("URL {}", url.clone());
        let client = Client::new();
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| ApiError::GeonamesApi(format!("Geonames api error: {}", e)))?;

        let res = response
            .bytes()
            .await
            .map_err(|e| ApiError::Deserialization(format!("Deserialization error: {}", e)))?;

        eprintln!("HEREEEEEEEEE {:#?}", res);

        let api_res = T::deserialize_response(res)?;

        Ok(api_res)
    }

    fn allowed_params(&self) -> Option<&'static HashMap<&'static str, Vec<&'static str>>> {
        match self.api {
            GeoNamesApi::Astergdem => Some(&ASTERGDEM_PARAMS),
            GeoNamesApi::Children => Some(&CHILDREN_PARAMS),
            GeoNamesApi::Cities => Some(&CITIES_PARAMS),
            GeoNamesApi::Contains => Some(&CONTAINS_PARAMS),
            GeoNamesApi::CountryCode => Some(&COUNTRY_CODE_PARAMS),
            GeoNamesApi::CountryInfo => Some(&COUNTRY_INFO_PARAMS),
            GeoNamesApi::CountrySubdivision => Some(&COUNTRY_SUBDIVISION_PARAMS),
            GeoNamesApi::Earthquakes => Some(&EARTHQUAKES_PARAMS),
            GeoNamesApi::FindNearby => Some(&FIND_NEARBY_PARAMS),
            GeoNamesApi::FindNearbyPlaceName => Some(&FIND_NEARBY_PLACE_NAME_PARAMS),
            GeoNamesApi::FindNearbyPostalCodes => Some(&FIND_NEARBY_POSTAL_CODES_PARAMS),
            GeoNamesApi::FindNearbyStreetsOsm => Some(&FIND_NEARBY_STREETS_0SM_PARAMS),
            GeoNamesApi::FindNearByWeather => Some(&FIND_NEARBY_BY_WEATHER_PARAMS),
            GeoNamesApi::FindNearbyWikipedia => Some(&FIND_NEARBY_BY_WIKIPEDIA_PARAMS),
            GeoNamesApi::FindNearestIntersectionOsm => None,
            GeoNamesApi::FindNearbyPoisOsm => Some(&FIND_NEARBY_POIS_OSM_PARAMS),
            GeoNamesApi::Address => Some(&ADDRESS_PARAMS),
            GeoNamesApi::GeoCodeAddress => Some(&GEO_CODE_ADDRESS_PARAMS),
            GeoNamesApi::StreetNameLookup => Some(&STREET_NAME_LOOKUP_PARAMS),
            GeoNamesApi::Get => Some(&GET_PARAMS),
            GeoNamesApi::Gtopo30 => Some(&GTOPO30_PARAMS),
            GeoNamesApi::Hierarchy => None,
            GeoNamesApi::Neighbourhood => Some(&NEIGHBOURHOOD_PARAMS),
            GeoNamesApi::Neighbours => None,
            GeoNamesApi::Ocean => Some(&OCEAN_PARAMS),
            GeoNamesApi::PostalCodeCountryInfo => None,
            GeoNamesApi::PostalCodeLookup => Some(&POSTAL_CODE_LOOKUP_PARAMS),
            GeoNamesApi::PostalCodeSearch => Some(&POSTAL_CODE_SEARCH_PARAMS),
            GeoNamesApi::RssToGeo => None,
            GeoNamesApi::Search => None,
            GeoNamesApi::Siblings => None,
            GeoNamesApi::Srtm1 => Some(&SRTM1_PARAMS),
            GeoNamesApi::Srtm3 => Some(&SRTM3_PARAMS),
            GeoNamesApi::Timezone => Some(&TIMEZONE_PARAMS),
            GeoNamesApi::Weather => None,
            GeoNamesApi::WeatherIcao => None,
            GeoNamesApi::WikipediaBoundingBox => None,
            GeoNamesApi::WikipediaSearch => None,
        }
    }
}
