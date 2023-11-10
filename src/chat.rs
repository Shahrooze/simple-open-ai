/// Module `chat` - Handles API communication for chat completions.
/// 
/// # Overview
/// This module provides functionality to send requests to a chat completion API
/// and interpret the responses. It is a part of a chat application that uses
/// AI models to generate responses based on a conversational context.
/// 
/// # Dependencies
/// - `reqwest`: A high-level HTTP client.
/// - `serde`: A serialization and deserialization framework.
/// 
/// # Constants
/// `ENDPOINT`: The API endpoint for obtaining chat completions.
/// 
/// # Structures
/// - `ChatCompletionRequest`: Represents a request payload for chat completions.
/// - `ChatCompletionResponse`: Encapsulates the response from the chat completion API.
/// 
/// # Error Handling
/// Uses `ApiError` from `crate::error` module for error responses.
/// 
/// # Examples
/// ```
/// use crate::chat::{ChatCompletionRequest, ChatCompletionResponse};
/// let messages = vec![/* ... message history ... */];
/// let request = ChatCompletionRequest {
///     messages: &messages,
///     max_tokens: 150,
///     model: "gpt-3.5-turbo".to_string(),
/// };
/// 
/// // Example of sending the request
/// let client = reqwest::Client::new();
/// let response = async {
///     client.post(ENDPOINT)
///         .json(&request)
///         .send()
///         .await?
///         .json::<ChatCompletionResponse>()
///         .await
/// };
/// ```
/// 
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

use crate::error::ApiError;

const ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest<'a> {
    pub messages: &'a Vec<Message>,
    pub max_tokens: u32,
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    // pub system_fingerprint: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub index: u8,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
pub async fn send_text<'a>(
    api_key: &str,
    req: ChatCompletionRequest<'a>,
) -> Result<ChatCompletionResponse, ApiError> {
    let client: Client = Client::new();
    let response = client
        .post(ENDPOINT)
        .bearer_auth(api_key)
        .json(&req)
        .send()
        .await;
    match response {
        Ok(response) => {
            if response.status().is_success() {
                let response_body = response.json::<ChatCompletionResponse>().await?;
                return Ok(response_body);
            } else {
                return Err(ApiError::Error(response.status().to_string()));
            }
        }
        Err(e) => return Err(ApiError::RequestError(e)),
    }
}
