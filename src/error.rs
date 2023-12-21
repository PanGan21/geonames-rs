#[derive(Debug)]
pub enum ApiError {
    Deserialization(String),
    UrlParse(String),
    InvalidParams(String),
    GeonamesApi(String),
}
