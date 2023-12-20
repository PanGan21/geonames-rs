use bytes::Bytes;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::ApiError;

pub trait ApiResponse: DeserializeOwned + Serialize {
    fn deserialize_response(bytes: Bytes) -> Result<Self, ApiError>
    where
        Self: Sized;
}

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

impl ApiResponse for CountryCodeResponse {
    fn deserialize_response(bytes: Bytes) -> Result<Self, ApiError> {
        serde_json::from_slice(&bytes)
            .map_err(|e| ApiError::Deserialization(format!("Deserialization error: {}", e)))
    }
}

impl ApiResponse for PostalCodeSearchResponse {
    fn deserialize_response(bytes: Bytes) -> Result<Self, ApiError> {
        serde_json::from_slice(&bytes)
            .map_err(|e| ApiError::Deserialization(format!("Deserialization error: {}", e)))
    }
}

impl ApiResponse for AstergdemResponse {
    fn deserialize_response(bytes: Bytes) -> Result<Self, ApiError> {
        serde_json::from_slice(&bytes)
            .map_err(|e| ApiError::Deserialization(format!("Deserialization error: {}", e)))
    }
}
