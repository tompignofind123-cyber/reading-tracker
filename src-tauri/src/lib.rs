mod commands;
mod db;
mod models;
mod chat;

use db::Database;
use std::path::PathBuf;
use tauri::Manager;

fn get_db_path(app: &tauri::App) -> PathBuf {
    let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
    app_dir.join("reading_tracker.db")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let db_path = get_db_path(app);
            let db = Database::new(db_path.to_str().unwrap())
                .expect("Failed to initialize database");
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_book,
            commands::update_book,
            commands::delete_book,
            commands::get_book,
            commands::get_books,
            commands::export_json,
            commands::export_csv,
            chat::commands::get_chat_settings,
            chat::commands::save_chat_settings,
            chat::commands::list_chat_sessions,
            chat::commands::create_chat_session,
            chat::commands::delete_chat_session,
            chat::commands::rename_chat_session,
            chat::commands::get_session_messages,
            chat::commands::send_chat_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
