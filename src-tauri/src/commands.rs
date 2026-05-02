use tauri::State;
use crate::db::Database;
use crate::models::{Book, SearchParams};

#[tauri::command]
pub fn add_book(db: State<'_, Database>, book: Book) -> Result<i64, String> {
    db.add_book(&book).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_book(db: State<'_, Database>, book: Book) -> Result<(), String> {
    db.update_book(&book).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_book(db: State<'_, Database>, id: i64) -> Result<(), String> {
    db.delete_book(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_book(db: State<'_, Database>, id: i64) -> Result<Book, String> {
    db.get_book(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_books(db: State<'_, Database>, params: SearchParams) -> Result<(Vec<Book>, i64), String> {
    db.get_books(&params).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_json(db: State<'_, Database>) -> Result<String, String> {
    let books = db.get_all_books().map_err(|e| e.to_string())?;
    serde_json::to_string_pretty(&books).map_err(|e| e.to_string())
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

#[tauri::command]
pub fn export_csv(db: State<'_, Database>) -> Result<String, String> {
    let books = db.get_all_books().map_err(|e| e.to_string())?;
    let mut buf = String::with_capacity(books.len() * 200 + 100);
    // BOM for Excel compatibility
    buf.push('\u{FEFF}');
    buf.push_str("ID,\u{6807}\u{9898},\u{4f5c}\u{8005},\u{5206}\u{7c7b},\u{6807}\u{7b7e},\u{5b57}\u{6570},\u{8bc4}\u{5206},\u{9605}\u{8bfb}\u{65e5}\u{671f},\u{8bfb}\u{540e}\u{611f},\u{521b}\u{5efa}\u{65f6}\u{95f4},\u{66f4}\u{65b0}\u{65f6}\u{95f4}\n");
    for b in &books {
        buf.push_str(&b.id.unwrap_or(0).to_string());
        buf.push(',');
        buf.push_str(&csv_escape(&b.title));
        buf.push(',');
        buf.push_str(&csv_escape(&b.author));
        buf.push(',');
        buf.push_str(&csv_escape(&b.category));
        buf.push(',');
        buf.push_str(&csv_escape(&b.tags.join("; ")));
        buf.push(',');
        buf.push_str(&b.word_count.to_string());
        buf.push(',');
        buf.push_str(&b.rating.to_string());
        buf.push(',');
        buf.push_str(&csv_escape(&b.date_read));
        buf.push(',');
        buf.push_str(&csv_escape(&b.reflection));
        buf.push(',');
        buf.push_str(&csv_escape(b.created_at.as_deref().unwrap_or("")));
        buf.push(',');
        buf.push_str(&csv_escape(b.updated_at.as_deref().unwrap_or("")));
        buf.push('\n');
    }
    Ok(buf)
}
