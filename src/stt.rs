/// Module `stt` - Handles speech-to-text conversion using an API service.
/// 
/// # Overview
/// This module provides functionality to convert speech to text by communicating
/// with an external API that processes audio data and returns transcriptions.
/// 
/// # Constants
/// - `ENDPOINT`: The API endpoint for speech-to-text conversion service.
/// 
/// # Structures
/// - `Request`: Represents an API request for STT conversion. Includes the audio
///   data and parameters such as model, language, and format preferences.
/// - `OpenAiResponse`: The expected response from the API, containing the
///   transcribed text.
/// 
/// # Enums and Traits
/// - `ResponseFormat`: (Assuming it's an enum, explanation would go here)
/// 
/// # Usage
/// ```rust
/// use crate::stt::{Request, OpenAiResponse};
/// let audio_data = vec![/* ... audio data ... */];
/// let request = Request {
///     // ... initialization of request fields ...
/// };
/// 
/// // Example of sending the request and handling the response
/// let client = reqwest::Client::new();
/// let response = async {
///     client.post(ENDPOINT)
///         .body(audio_data)
///         .send()
///         .await?
///         .json::<OpenAiResponse>()
///         .await
/// };
/// ```
/// 
use crate::error::ApiError;
use reqwest::Response;
use reqwest::{multipart, Client};
use serde::Deserialize;
use serde::Serialize;
use strum_macros::{Display, EnumString};

const ENDPOINT: &str = "https://api.openai.com/v1/audio/transcriptions";
#[derive(Serialize)]
pub struct Request {
    model: String,
    language: String,
    prompt: String,
    response_format: ResponseFormat,
    temperature: f32,
    content: Vec<u8>,
}
#[derive(Deserialize)]
pub struct OpenAiResponse {
    pub text: String,
}
#[derive(Serialize, EnumString, Display, Debug)]
pub enum ResponseFormat {
    #[serde(rename = "json")]
    #[strum(serialize = "json")]
    Json,
    #[strum(serialize = "text")]
    #[serde(rename = "text")]
    Text,
    #[strum(serialize = "srt")]
    #[serde(rename = "srt")]
    Srt,
    #[strum(serialize = "verbose_json")]
    #[serde(rename = "verbose_json")]
    VerboseJson,
    #[strum(serialize = "vtt")]
    #[serde(rename = "vtt")]
    Vtt,
}
impl Request {
    pub fn new(content: Vec<u8>) -> Request {
        Request {
            content: content,
            model: String::from("whisper-1"),
            language: String::from("en"),
            prompt: String::from(""),
            response_format: ResponseFormat::Json,
            temperature: 0.0,
        }
    }
    pub fn with_response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = response_format;
        self
    }
    pub fn with_language(mut self, language: String) -> Self {
        self.language = language;
        self
    }
    pub fn with_prompt(mut self, prompt: String) -> Self {
        self.prompt = prompt;
        self
    }
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }
}
pub async fn get_text_from_voice(api_key: &str, req: Request) -> Result<String, ApiError> {
    let part = multipart::Part::stream(reqwest::Body::from(req.content))
        .file_name("filename.mp3")
        .mime_str("audio/mpeg");
    if let Ok(part) = part {
        let form = multipart::Form::new()
            .text("model", "whisper-1")
            .text("language", req.language)
            .text("prompt", req.prompt)
            .text("response_format", req.response_format.to_string())
            .text("temperature", req.temperature.to_string())
            .part("file", part);
        let client: Client = Client::new();
        let response = client
            .post(ENDPOINT)
            .multipart(form)
            .bearer_auth(api_key)
            .send()
            .await;
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    return Ok(get_response(req.response_format, response).await);
                } else {
                    return Err(ApiError::Error(response.status().to_string()));
                }
            }
            Err(e) => return Err(ApiError::RequestError(e)),
        }
    }
    Err(ApiError::Error(String::from("Unknown Error")))
}
async fn get_response(response_fromat: ResponseFormat, response: Response) -> String {
    match response_fromat {
        ResponseFormat::Json => {
            if let Ok(response_body) = response.json::<OpenAiResponse>().await {
                return response_body.text;
            }
            "".to_string()
        }
        ResponseFormat::Text => {
            if let Ok(response_body) = response.text().await {
                return response_body;
            }
            "".to_string()
        }
        ResponseFormat::Srt => {
            if let Ok(response_body) = response.text().await {
                return response_body;
            }
            "".to_string()
        }
        ResponseFormat::VerboseJson => todo!(),
        ResponseFormat::Vtt => todo!(),
    }
}
