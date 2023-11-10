/// Module `error` - Defines error types for API error handling.
/// 
/// # Enum `ApiError`
/// An enumeration representing possible errors encountered during
/// API interaction. It is designed to encapsulate all error types into a single
/// manageable enum.
/// 
/// ## Variants
/// - `Error(String)`: A general error with a message.
/// - `RequestError(reqwest::Error)`: An error from the `reqwest` HTTP client.
/// 
/// # Trait Implementations
/// Implementation of `From<reqwest::Error>` trait for `ApiError` allows automatic conversion
/// which is useful for `?` operator use in error handling.
/// 
/// # Usage
/// This enum can be used to return and propagate errors in API related operations,
/// simplifying the match operations and conversion from other error types.
/// 
/// # Examples
/// ```rust
/// fn example() -> Result<(), ApiError> {
///     let result = reqwest::get("https://example.com")
///         .await
///         .map_err(ApiError::from)?;
///     // Handle result
///     Ok(())
/// }
/// ```
/// 
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
