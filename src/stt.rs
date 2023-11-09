use crate::error::ApiError;
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
}
#[derive(Deserialize)]
pub struct Response {
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
    pub fn new() -> Request {
        Request {
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
pub async fn get_text(
    content: Vec<u8>,
    api_key: &str,
    req: Request,
) -> Result<String, ApiError> {
    let part = multipart::Part::stream(reqwest::Body::from(content))
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
                    if let Ok(response_body) = response.json::<Response>().await {
                        return Ok(response_body.text);
                    }
                } else {
                    return Err(ApiError::Error(response.status().to_string()));
                }
            }
            Err(e) => return Err(ApiError::RequestError(e)),
        }
    }
    Err(ApiError::Error(String::from("Unknown Error")))
}
