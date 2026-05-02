use async_trait::async_trait;
use futures_util::StreamExt;
use serde_json::{json, Value};
use tauri::ipc::Channel;
use crate::chat::models::{ChatSettings, ChatMessage, ChatEvent, ChatSource};
use super::{ChatProvider, http_client, emit};

pub struct AnthropicProvider;

#[async_trait]
impl ChatProvider for AnthropicProvider {
    async fn stream(
        &self,
        settings: &ChatSettings,
        system_prompt: &str,
        history: &[ChatMessage],
        user_message: &str,
        on_event: &Channel<ChatEvent>,
    ) -> Result<(String, Vec<ChatSource>), String> {
        let mut messages: Vec<Value> = Vec::with_capacity(history.len() + 1);
        for m in history {
            if m.role == "system" { continue; }
            messages.push(json!({ "role": m.role, "content": m.content }));
        }
        messages.push(json!({ "role": "user", "content": user_message }));

        let mut body = json!({
            "model": settings.model,
            "max_tokens": 4096,
            "system": system_prompt,
            "messages": messages,
            "stream": true,
        });

        if settings.enable_search {
            body["tools"] = json!([{
                "type": "web_search_20250305",
                "name": "web_search",
                "max_uses": 5
            }]);
        }

        let base = settings.base_url.trim_end_matches('/');
        // 如果用户填的是默认 OpenAI 风格 URL 则修正；否则尊重用户填写
        let base = if base.is_empty() || base == "https://api.openai.com/v1" {
            "https://api.anthropic.com"
        } else {
            base
        };
        let url = format!("{}/v1/messages", base);

        let resp = http_client()
            .post(&url)
            .header("x-api-key", &settings.api_key)
            .header("anthropic-version", "2023-06-01")
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

                let event_type = v.get("type").and_then(|t| t.as_str()).unwrap_or("");
                match event_type {
                    "content_block_delta" => {
                        if let Some(delta) = v.get("delta") {
                            let dt = delta.get("type").and_then(|t| t.as_str()).unwrap_or("");
                            if dt == "text_delta" {
                                if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                                    if !text.is_empty() {
                                        full_text.push_str(text);
                                        emit(on_event, ChatEvent::Delta { text: text.to_string() });
                                    }
                                }
                            }
                        }
                    }
                    "content_block_start" => {
                        if let Some(block) = v.get("content_block") {
                            let bt = block.get("type").and_then(|t| t.as_str()).unwrap_or("");
                            if bt == "web_search_tool_result" {
                                if let Some(content) = block.get("content").and_then(|c| c.as_array()) {
                                    for item in content {
                                        if let Some(url_val) = item.get("url").and_then(|u| u.as_str()) {
                                            let title = item.get("title").and_then(|t| t.as_str()).map(String::from);
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
                    "message_stop" => break,
                    _ => {}
                }
            }
        }

        Ok((full_text, sources))
    }
}
