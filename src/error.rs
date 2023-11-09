#[derive(Debug)]
pub enum ApiError {
    Error(String),
    RequestError(reqwest::Error),
}
impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        ApiError::Error(value.to_string())
    }
}
