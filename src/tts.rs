
/// Module `tts` - Handles Text-to-Speech (TTS) conversion using an API service.
///
/// # Overview
/// This module provides functionality to convert text input to speech audio, 
/// interfacing with an external TTS service API.
///
/// # Dependencies
/// - `reqwest`: A high-level HTTP client for making requests.
/// - `bytes`: Utilities for working with bytes.
/// - `serde`: Serialization and deserialization framework, used here to serialize request data.
///
/// # Constants
/// `ENDPOINT`: The API endpoint for the TTS service.
///
/// # Structures
/// `Request`: Represents a TTS API request with parameters for the speech model, input text, voice settings, 
/// and desired response format.
///
/// # Enums
/// `ResponseFormat`: Enumerates the possible audio formats for the TTS response, including MP3, Opus, AAC, and FLAC.
///
/// # Error Handling
/// Utilizes `ApiError` from `crate::error` for consistent error management across the application.
///
/// # Examples
/// Example usage not provided due to lack of complete context in the snippet.
///
/// Note: Examples and detailed error handling require a complete understanding of the module's usage patterns.
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
