use async_trait::async_trait;
use futures_util::StreamExt;
use serde_json::{json, Value};
use tauri::ipc::Channel;
use crate::chat::models::{ChatSettings, ChatMessage, ChatEvent, ChatSource};
use super::{ChatProvider, http_client, emit};

pub struct GeminiProvider;

fn map_role(role: &str) -> &'static str {
    if role == "assistant" { "model" } else { "user" }
}

#[async_trait]
impl ChatProvider for GeminiProvider {
    async fn stream(
        &self,
        settings: &ChatSettings,
        system_prompt: &str,
        history: &[ChatMessage],
        user_message: &str,
        on_event: &Channel<ChatEvent>,
    ) -> Result<(String, Vec<ChatSource>), String> {
        let mut contents: Vec<Value> = Vec::with_capacity(history.len() + 1);
        for m in history {
            if m.role == "system" { continue; }
            contents.push(json!({
                "role": map_role(&m.role),
                "parts": [{ "text": m.content }]
            }));
        }
        contents.push(json!({
            "role": "user",
            "parts": [{ "text": user_message }]
        }));

        let mut body = json!({
            "systemInstruction": { "parts": [{ "text": system_prompt }] },
            "contents": contents,
        });

        if settings.enable_search {
            body["tools"] = json!([{ "google_search": {} }]);
        }

        let base = settings.base_url.trim_end_matches('/');
        let base = if base.is_empty() || base == "https://api.openai.com/v1" {
            "https://generativelanguage.googleapis.com/v1beta"
        } else {
            base
        };
        let url = format!(
            "{}/models/{}:streamGenerateContent?alt=sse&key={}",
            base, settings.model, settings.api_key
        );

        let resp = http_client()
            .post(&url)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("HTTP 请求失败: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("API 返回 {}: {}", status, text));
        }

        let mut stream = resp.bytes_stream();
        let mut buffer = String::new();
        let mut full_text = String::new();
        let mut sources: Vec<ChatSource> = Vec::new();

        while let Some(chunk) = stream.next().await {
            let bytes = chunk.map_err(|e| format!("读取流失败: {}", e))?;
            buffer.push_str(&String::from_utf8_lossy(&bytes));

            while let Some(pos) = buffer.find('\n') {
                let line = buffer[..pos].trim_end_matches('\r').to_string();
                buffer.drain(..=pos);
                if !line.starts_with("data:") { continue; }
                let payload = line[5..].trim();
                if payload.is_empty() { continue; }

                let v: Value = match serde_json::from_str(payload) {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                if let Some(candidates) = v.get("candidates").and_then(|c| c.as_array()) {
                    for cand in candidates {
                        if let Some(parts) = cand.pointer("/content/parts").and_then(|p| p.as_array()) {
                            for part in parts {
                                if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                                    if !text.is_empty() {
                                        full_text.push_str(text);
                                        emit(on_event, ChatEvent::Delta { text: text.to_string() });
                                    }
                                }
                            }
                        }
                        if let Some(chunks) = cand.pointer("/groundingMetadata/groundingChunks").and_then(|c| c.as_array()) {
                            for ch in chunks {
                                if let Some(uri) = ch.pointer("/web/uri").and_then(|u| u.as_str()) {
                                    let title = ch.pointer("/web/title").and_then(|t| t.as_str()).map(String::from);
                                    let src = ChatSource { url: uri.to_string(), title: title.clone() };
                                    if !sources.iter().any(|s| s.url == src.url) {
                                        emit(on_event, ChatEvent::Source { url: src.url.clone(), title });
                                        sources.push(src);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok((full_text, sources))
    }
}
