use async_trait::async_trait;
use futures_util::StreamExt;
use serde_json::{json, Value};
use tauri::ipc::Channel;
use crate::chat::models::{ChatSettings, ChatMessage, ChatEvent, ChatSource};
use super::{ChatProvider, http_client, emit};

pub struct OpenAIProvider;

#[async_trait]
impl ChatProvider for OpenAIProvider {
    async fn stream(
        &self,
        settings: &ChatSettings,
        system_prompt: &str,
        history: &[ChatMessage],
        user_message: &str,
        on_event: &Channel<ChatEvent>,
    ) -> Result<(String, Vec<ChatSource>), String> {
        let mut messages: Vec<Value> = Vec::with_capacity(history.len() + 2);
        messages.push(json!({ "role": "system", "content": system_prompt }));
        for m in history {
            if m.role == "system" { continue; }
            messages.push(json!({ "role": m.role, "content": m.content }));
        }
        messages.push(json!({ "role": "user", "content": user_message }));

        let mut body = json!({
            "model": settings.model,
            "messages": messages,
            "stream": true,
        });

        if settings.enable_search {
            body["tools"] = json!([{ "type": "web_search_preview" }]);
        }

        let url = format!("{}/chat/completions", settings.base_url.trim_end_matches('/'));
        let resp = http_client()
            .post(&url)
            .bearer_auth(&settings.api_key)
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
                if payload == "[DONE]" { break; }

                let v: Value = match serde_json::from_str(payload) {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                if let Some(choices) = v.get("choices").and_then(|c| c.as_array()) {
                    if let Some(first) = choices.first() {
                        if let Some(delta) = first.get("delta") {
                            if let Some(content) = delta.get("content").and_then(|c| c.as_str()) {
                                if !content.is_empty() {
                                    full_text.push_str(content);
                                    emit(on_event, ChatEvent::Delta { text: content.to_string() });
                                }
                            }
                            // 部分模型在 delta.annotations 给出引用
                            if let Some(anns) = delta.get("annotations").and_then(|a| a.as_array()) {
                                for a in anns {
                                    if let Some(url_val) = a.get("url").and_then(|u| u.as_str()) {
                                        let title = a.get("title").and_then(|t| t.as_str()).map(String::from);
                                        let src = ChatSource { url: url_val.to_string(), title: title.clone() };
                                        if !sources.iter().any(|s| s.url == src.url) {
                                            emit(on_event, ChatEvent::Source { url: src.url.clone(), title });
                                            sources.push(src);
                                        }
                                    }
                                }
                            }
                        }
                        // gpt-4o-search-preview 在完成时把 citations 放在 message.annotations
                        if let Some(msg) = first.get("message") {
                            if let Some(anns) = msg.get("annotations").and_then(|a| a.as_array()) {
                                for a in anns {
                                    if let Some(url_val) = a.get("url").and_then(|u| u.as_str()) {
                                        let title = a.get("title").and_then(|t| t.as_str()).map(String::from);
                                        let src = ChatSource { url: url_val.to_string(), title: title.clone() };
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
        }

        Ok((full_text, sources))
    }
}
