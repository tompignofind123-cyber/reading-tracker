pub mod openai;
pub mod anthropic;
pub mod gemini;

use async_trait::async_trait;
use tauri::ipc::Channel;
use crate::chat::models::{ChatSettings, ChatMessage, ChatEvent, ChatSource};

#[async_trait]
pub trait ChatProvider: Send + Sync {
    async fn stream(
        &self,
        settings: &ChatSettings,
        system_prompt: &str,
        history: &[ChatMessage],
        user_message: &str,
        on_event: &Channel<ChatEvent>,
    ) -> Result<(String, Vec<ChatSource>), String>;
}

pub fn get_provider(name: &str) -> Box<dyn ChatProvider> {
    match name {
        "anthropic" => Box::new(anthropic::AnthropicProvider),
        "gemini" => Box::new(gemini::GeminiProvider),
        _ => Box::new(openai::OpenAIProvider),
    }
}

pub fn http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(180))
        .connect_timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("failed to build reqwest client")
}

pub fn emit(on_event: &Channel<ChatEvent>, ev: ChatEvent) {
    let _ = on_event.send(ev);
}
