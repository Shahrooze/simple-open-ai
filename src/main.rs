mod chat;
mod error;
mod stt;
mod tts;

use chat::send_text;
use error::ApiError;
use std::io;
use stt::get_text_from_voice;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tts::{get_voice_from_text, Request, Voice};

use crate::chat::ChatCompletionRequest;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    println!("Pelase enter api key");
    let mut api_key = String::new();
    io::stdin()
        .read_line(&mut api_key)
        .expect("Failed to read line");
    println!("api key {} ", api_key);
    let api_key = api_key.trim();
    let mut conversation: Vec<chat::Message> = Vec::new();

    loop {
        let mut input_text = String::new();
        println!("Pelase write any thing");
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");
        let user_message = chat::Message {
            role: "user".to_string(),
            content: input_text.trim().to_string(), // Trim the newline or whitespace
        };
        conversation.push(user_message);
        let chat_request = ChatCompletionRequest {
            messages: &conversation,
            max_tokens: 100,
            model: "gpt-3.5-turbo".to_string(),
        };
        let response = send_text(&api_key, chat_request).await.unwrap();
        if let Some(choice) = response.choices.first() {
            println!("Chatbot: {}", choice.message.content);
            conversation.push(chat::Message {
                role: choice.message.role.clone(),
                content: choice.message.content.clone(),
            });
        }
        // let tts_request = Request::new(String::from("tts-1"), input_text, Voice::Nova);
        // let bytes = get_voice_from_text(tts_request, api_key).await?;
        // let mut file = File::create("output.mp3").await.unwrap();
        // file.write_all(&bytes).await.unwrap();

        // let file_path = "output.mp3";
        // let mut file: File = File::open(file_path).await.unwrap();
        // let mut contents = Vec::new();
        // file.read_to_end(&mut contents).await.unwrap();

        // let mut sst_request = stt::Request::new(contents);
        //  sst_request = sst_request.with_language("fa".to_string());
        // sst_request = sst_request.with_response_format(stt::ResponseFormat::Text);
        // let text = get_text_from_voice(api_key, sst_request).await.unwrap();
        // println!("From OpenAi :{}", text)
    }
}
