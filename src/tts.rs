use crate::error::ApiError;
use bytes::Bytes;
use reqwest::Client;
use serde::Serialize;
const ENDPOINT: &str = "https://api.openai.com/v1/audio/speech";
#[derive(Serialize)]
pub struct Request {
    model: String,
    input: String,
    voice: Voice,
    response_format: ResponseFormat,
}
#[derive(Serialize)]
pub enum ResponseFormat {
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "opus")]
    Opus,
    #[serde(rename = "aac")]
    Aac,
    #[serde(rename = "flac")]
    Flac,
}
#[derive(Serialize)]
pub enum Voice {
    #[serde(rename = "alloy")]
    Alloy,
    #[serde(rename = "echo")]
    Echo,
    #[serde(rename = "fable")]
    Fable,
    #[serde(rename = "onyx")]
    Onyx,
    #[serde(rename = "nova")]
    Nova,
    #[serde(rename = "shimmer")]
    Shimmer,
}
impl Request {
    pub fn new(model: String, input: String, voice: Voice) -> Self {
        Request {
            model: model,
            input: input,
            voice: voice,
            response_format: ResponseFormat::Mp3,
        }
    }
    pub fn with_response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = response_format;
        self
    }
}
pub async fn get_voice_from_text(req: Request, api_key: &str) -> Result<Bytes, ApiError> {
    let client = Client::new();
    let response = client
        .post(ENDPOINT)
        .bearer_auth(api_key)
        .json(&req)
        .send()
        .await;
    match response {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(response) = response.bytes().await {
                    Ok(response)
                } else {
                    Err(ApiError::Error(String::from("Error in posting data")))
                }
            } else {
                Err(ApiError::Error(response.status().to_string()))
            }
        }
        Err(e) => Err(ApiError::RequestError(e)),
    }
}
