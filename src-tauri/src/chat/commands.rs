use tauri::{State, ipc::Channel};
use crate::db::Database;
use crate::chat::models::{ChatSettings, ChatSession, ChatMessage, ChatEvent};
use crate::chat::prompt::{build_system_prompt, library_summary_text};
use crate::chat::providers::get_provider;

#[tauri::command]
pub fn get_chat_settings(db: State<'_, Database>) -> Result<ChatSettings, String> {
    db.get_chat_settings().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_chat_settings(db: State<'_, Database>, settings: ChatSettings) -> Result<(), String> {
    db.save_chat_settings(&settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_chat_sessions(
    db: State<'_, Database>,
    book_id: Option<i64>,
    mode: Option<String>,
) -> Result<Vec<ChatSession>, String> {
    db.list_chat_sessions(book_id, mode).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_chat_session(
    db: State<'_, Database>,
    book_id: Option<i64>,
    mode: String,
    title: Option<String>,
) -> Result<i64, String> {
    let t = title.unwrap_or_else(|| {
        let prefix = match mode.as_str() {
            "book" => "本书对话",
            "cross" => "跨书对话",
            _ => "新对话",
        };
        format!("{} {}", prefix, chrono::Local::now().format("%m-%d %H:%M"))
    });
    db.create_chat_session(book_id, &mode, &t).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_chat_session(db: State<'_, Database>, id: i64) -> Result<(), String> {
    db.delete_chat_session(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn rename_chat_session(db: State<'_, Database>, id: i64, title: String) -> Result<(), String> {
    db.rename_chat_session(id, &title).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_session_messages(db: State<'_, Database>, session_id: i64) -> Result<Vec<ChatMessage>, String> {
    db.get_session_messages(session_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_chat_message(
    db: State<'_, Database>,
    session_id: i64,
    content: String,
    on_event: Channel<ChatEvent>,
) -> Result<(), String> {
    let session = db.get_chat_session(session_id).map_err(|e| e.to_string())?;
    let settings = db.get_chat_settings().map_err(|e| e.to_string())?;

    if settings.api_key.trim().is_empty() {
        let msg = "未配置 API Key，请先在「AI 设置」中填写。".to_string();
        let _ = on_event.send(ChatEvent::Error { message: msg.clone() });
        return Err(msg);
    }

    db.add_message(session_id, "user", &content, &[]).map_err(|e| e.to_string())?;
    let history = db.get_session_messages(session_id).map_err(|e| e.to_string())?;
    let history_for_provider: &[ChatMessage] = if history.is_empty() {
        &[]
    } else {
        &history[..history.len() - 1]
    };

    let book = if session.mode == "book" {
        session.book_id.and_then(|bid| db.get_book(bid).ok())
    } else { None };

    let lib_summary = if session.mode == "cross" {
        let books = db.get_all_books().map_err(|e| e.to_string())?;
        Some(library_summary_text(&books, 50))
    } else { None };

    let system_prompt = build_system_prompt(&session.mode, book.as_ref(), lib_summary.as_deref());

    let provider = get_provider(&settings.provider);
    let result = provider.stream(
        &settings,
        &system_prompt,
        history_for_provider,
        &content,
        &on_event,
    ).await;

    match result {
        Ok((full_text, sources)) => {
            let saved_text = if full_text.is_empty() { "（空响应）".to_string() } else { full_text };
            let msg_id = db.add_message(session_id, "assistant", &saved_text, &sources)
                .map_err(|e| e.to_string())?;
            db.touch_chat_session(session_id).ok();
            let _ = on_event.send(ChatEvent::Done { message_id: msg_id });
            Ok(())
        }
        Err(err) => {
            let _ = on_event.send(ChatEvent::Error { message: err.clone() });
            Err(err)
        }
    }
}
