use rusqlite::{params, Result as SqlResult};
use crate::db::Database;
use crate::chat::models::{ChatSettings, ChatSession, ChatMessage, ChatSource};

impl Database {
    pub fn get_chat_settings(&self) -> SqlResult<ChatSettings> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT provider, base_url, api_key, model, enable_search, extra_json FROM chat_settings WHERE id = 1",
            [],
            |row| Ok(ChatSettings {
                provider: row.get(0)?,
                base_url: row.get(1)?,
                api_key: row.get(2)?,
                model: row.get(3)?,
                enable_search: row.get::<_, i64>(4)? != 0,
                extra_json: row.get(5)?,
            }),
        )
    }

    pub fn save_chat_settings(&self, s: &ChatSettings) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE chat_settings SET provider=?1, base_url=?2, api_key=?3, model=?4, enable_search=?5, extra_json=?6 WHERE id = 1",
            params![s.provider, s.base_url, s.api_key, s.model, s.enable_search as i64, s.extra_json],
        )?;
        Ok(())
    }

    pub fn list_chat_sessions(&self, book_id: Option<i64>, mode: Option<String>) -> SqlResult<Vec<ChatSession>> {
        let conn = self.conn.lock().unwrap();
        let mut where_parts: Vec<String> = Vec::new();
        let mut binds: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
        let mut idx = 1;
        if let Some(b) = book_id {
            where_parts.push(format!("book_id = ?{}", idx));
            binds.push(Box::new(b));
            idx += 1;
        }
        if let Some(m) = mode {
            where_parts.push(format!("mode = ?{}", idx));
            binds.push(Box::new(m));
        }
        let where_clause = if where_parts.is_empty() { String::new() } else { format!(" WHERE {}", where_parts.join(" AND ")) };
        let sql = format!(
            "SELECT id, book_id, mode, title, created_at, updated_at FROM chat_sessions{} ORDER BY updated_at DESC, id DESC",
            where_clause
        );
        let mut stmt = conn.prepare(&sql)?;
        let bind_refs: Vec<&dyn rusqlite::types::ToSql> = binds.iter().map(|b| b.as_ref()).collect();
        let rows = stmt.query_map(bind_refs.as_slice(), |row| Ok(ChatSession {
            id: Some(row.get(0)?),
            book_id: row.get(1)?,
            mode: row.get(2)?,
            title: row.get(3)?,
            created_at: Some(row.get(4)?),
            updated_at: Some(row.get(5)?),
        }))?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    pub fn create_chat_session(&self, book_id: Option<i64>, mode: &str, title: &str) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        conn.execute(
            "INSERT INTO chat_sessions (book_id, mode, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4)",
            params![book_id, mode, title, now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn delete_chat_session(&self, id: i64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM chat_messages WHERE session_id = ?1", params![id])?;
        conn.execute("DELETE FROM chat_sessions WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn rename_chat_session(&self, id: i64, title: &str) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE chat_sessions SET title = ?1 WHERE id = ?2", params![title, id])?;
        Ok(())
    }

    pub fn touch_chat_session(&self, id: i64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        conn.execute("UPDATE chat_sessions SET updated_at = ?1 WHERE id = ?2", params![now, id])?;
        Ok(())
    }

    pub fn get_chat_session(&self, id: i64) -> SqlResult<ChatSession> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, book_id, mode, title, created_at, updated_at FROM chat_sessions WHERE id = ?1",
            params![id],
            |row| Ok(ChatSession {
                id: Some(row.get(0)?),
                book_id: row.get(1)?,
                mode: row.get(2)?,
                title: row.get(3)?,
                created_at: Some(row.get(4)?),
                updated_at: Some(row.get(5)?),
            }),
        )
    }

    pub fn get_session_messages(&self, session_id: i64) -> SqlResult<Vec<ChatMessage>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, session_id, role, content, sources_json, created_at FROM chat_messages WHERE session_id = ?1 ORDER BY id ASC"
        )?;
        let rows = stmt.query_map(params![session_id], |row| {
            let sources_str: String = row.get(4)?;
            let sources: Vec<ChatSource> = serde_json::from_str(&sources_str).unwrap_or_default();
            Ok(ChatMessage {
                id: Some(row.get(0)?),
                session_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                sources,
                created_at: Some(row.get(5)?),
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    pub fn add_message(&self, session_id: i64, role: &str, content: &str, sources: &[ChatSource]) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let sources_json = serde_json::to_string(sources).unwrap_or_else(|_| "[]".to_string());
        conn.execute(
            "INSERT INTO chat_messages (session_id, role, content, sources_json, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![session_id, role, content, sources_json, now],
        )?;
        Ok(conn.last_insert_rowid())
    }
}
