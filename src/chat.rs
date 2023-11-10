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
