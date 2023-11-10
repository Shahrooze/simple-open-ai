/// Root module of the library crate.
/// 
/// # Library Overview
/// This crate provides a set of modules for speech-to-text (STT), text-to-speech (TTS),
/// error handling, and chat functionality as part of a conversational AI application.
/// It's designed to be modular and reusable across various components of the application.
/// 
/// # Modules
/// - `stt`: Handles the conversion of speech to text.
/// - `tts`: Converts text into spoken voice outputs.
/// - `error`: Provides error types and handling mechanisms for the API.
/// - `chat`: Manages communication with a chat completion API service.
/// 
/// # Usage
/// This library can be included in Rust projects by adding it to the dependencies
/// in the project's `Cargo.toml` file and using the modules as needed.
/// 
/// For more details on each module, refer to the respective module documentation.
/// 
mod stt;
mod tts;
mod error;
mod chat;
