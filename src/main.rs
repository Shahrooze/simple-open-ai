mod error;
mod stt;
mod tts;

use error::ApiError;
use std::io;
use stt::get_text_from_voice;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tts::{get_voice_from_text, Request, Voice};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    println!("Pelase enter api key");
    let mut api_key = String::new();
    io::stdin()
        .read_line(&mut api_key)
        .expect("Failed to read line");
    println!("api key {} ", api_key);
    let api_key = api_key.trim();
    loop {
        let mut input_text = String::new();
        println!("Pelase write any thing");
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");
        let tts_request = Request::new(String::from("tts-1"), input_text, Voice::Nova);
        let bytes = get_voice_from_text(tts_request, api_key).await?;
        let mut file = File::create("output.mp3").await.unwrap();
        file.write_all(&bytes).await.unwrap();

        let file_path = "output.mp3";
        let mut file: File = File::open(file_path).await.unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await.unwrap();

        let mut sst_request = stt::Request::new(contents);
        // sst_request = sst_request.with_language("".to_string());
        sst_request = sst_request.with_response_format(stt::ResponseFormat::Text);
        let text = get_text_from_voice(api_key, sst_request).await.unwrap();
        println!("From OpenAi :{}", text)
    }
}
