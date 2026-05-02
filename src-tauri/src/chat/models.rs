use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSettings {
    pub provider: String,        // "openai" | "anthropic" | "gemini"
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub enable_search: bool,
    #[serde(default)]
    pub extra_json: String,
}

impl Default for ChatSettings {
    fn default() -> Self {
        Self {
            provider: "openai".into(),
            base_url: "https://api.openai.com/v1".into(),
            api_key: String::new(),
            model: "gpt-4o-mini".into(),
            enable_search: true,
            extra_json: "{}".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: Option<i64>,
    pub book_id: Option<i64>,
    pub mode: String,            // "book" | "cross" | "global"
    pub title: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSource {
    pub url: String,
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Option<i64>,
    pub session_id: i64,
    pub role: String,            // "user" | "assistant" | "system"
    pub content: String,
    #[serde(default)]
    pub sources: Vec<ChatSource>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
pub enum ChatEvent {
    Delta { text: String },
    Source { url: String, title: Option<String> },
    Error { message: String },
    Done { message_id: i64 },
}
