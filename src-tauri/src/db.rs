use rusqlite::{Connection, params, Result as SqlResult};
use std::sync::Mutex;
use crate::models::{Book, SearchParams};

pub struct Database {
    pub(crate) conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &str) -> SqlResult<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA cache_size=-2000;
             PRAGMA temp_store=MEMORY;
             PRAGMA mmap_size=268435456;"
        )?;
        let db = Database { conn: Mutex::new(conn) };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS books (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                title       TEXT NOT NULL,
                author      TEXT DEFAULT '',
                category    TEXT NOT NULL,
                tags        TEXT DEFAULT '[]',
                word_count  INTEGER DEFAULT 0,
                rating      INTEGER DEFAULT 0,
                date_read   TEXT NOT NULL,
                reflection  TEXT DEFAULT '',
                created_at  TEXT NOT NULL,
                updated_at  TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_books_category ON books(category);
            CREATE INDEX IF NOT EXISTS idx_books_date_read ON books(date_read);
            CREATE INDEX IF NOT EXISTS idx_books_rating ON books(rating);
            CREATE INDEX IF NOT EXISTS idx_books_word_count ON books(word_count);
            CREATE INDEX IF NOT EXISTS idx_books_title ON books(title);

            CREATE TABLE IF NOT EXISTS chat_settings (
                id            INTEGER PRIMARY KEY CHECK (id = 1),
                provider      TEXT NOT NULL DEFAULT 'openai',
                base_url      TEXT NOT NULL DEFAULT 'https://api.openai.com/v1',
                api_key       TEXT NOT NULL DEFAULT '',
                model         TEXT NOT NULL DEFAULT 'gpt-4o-mini',
                enable_search INTEGER NOT NULL DEFAULT 1,
                extra_json    TEXT NOT NULL DEFAULT '{}'
            );
            INSERT OR IGNORE INTO chat_settings (id) VALUES (1);

            CREATE TABLE IF NOT EXISTS chat_sessions (
                id         INTEGER PRIMARY KEY AUTOINCREMENT,
                book_id    INTEGER REFERENCES books(id) ON DELETE SET NULL,
                mode       TEXT NOT NULL,
                title      TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_sessions_book ON chat_sessions(book_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_updated ON chat_sessions(updated_at);

            CREATE TABLE IF NOT EXISTS chat_messages (
                id           INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id   INTEGER NOT NULL REFERENCES chat_sessions(id) ON DELETE CASCADE,
                role         TEXT NOT NULL,
                content      TEXT NOT NULL,
                sources_json TEXT NOT NULL DEFAULT '[]',
                created_at   TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_messages_session ON chat_messages(session_id, id);

            PRAGMA foreign_keys = ON;"
        )?;
        Ok(())
    }

    pub fn add_book(&self, book: &Book) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let tags_json = serde_json::to_string(&book.tags).unwrap_or_default();
        conn.execute(
            "INSERT INTO books (title, author, category, tags, word_count, rating, date_read, reflection, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![book.title, book.author, book.category, tags_json, book.word_count, book.rating, book.date_read, book.reflection, now, now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn update_book(&self, book: &Book) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let tags_json = serde_json::to_string(&book.tags).unwrap_or_default();
        conn.execute(
            "UPDATE books SET title=?1, author=?2, category=?3, tags=?4, word_count=?5, rating=?6, date_read=?7, reflection=?8, updated_at=?9 WHERE id=?10",
            params![book.title, book.author, book.category, tags_json, book.word_count, book.rating, book.date_read, book.reflection, now, book.id],
        )?;
        Ok(())
    }

    pub fn delete_book(&self, id: i64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM books WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn get_book(&self, id: i64) -> SqlResult<Book> {
        let conn = self.conn.lock().unwrap();
        conn.query_row("SELECT id, title, author, category, tags, word_count, rating, date_read, reflection, created_at, updated_at FROM books WHERE id=?1", params![id], |row| {
            Self::row_to_book(row)
        })
    }

    pub fn get_books(&self, search: &SearchParams) -> SqlResult<(Vec<Book>, i64)> {
        let conn = self.conn.lock().unwrap();
        let mut sql = String::with_capacity(256);
        let mut count_sql = String::with_capacity(128);
        let mut bind_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::with_capacity(8);

        let mut where_parts: Vec<String> = Vec::with_capacity(8);
        let mut param_idx = 1usize;

        macro_rules! add_filter {
            ($cond:expr, $val:expr) => {{
                where_parts.push($cond.replace("?N", &format!("?{}", param_idx)));
                bind_values.push(Box::new($val));
                param_idx += 1;
            }};
        }

        if let Some(ref kw) = search.keyword {
            if !kw.is_empty() {
                let like = format!("%{}%", kw);
                let cond = format!("(title LIKE ?{} OR author LIKE ?{})", param_idx, param_idx + 1);
                where_parts.push(cond);
                bind_values.push(Box::new(like.clone()));
                bind_values.push(Box::new(like));
                param_idx += 2;
            }
        }
        if let Some(ref cat) = search.category {
            if !cat.is_empty() { add_filter!("category = ?N", cat.clone()); }
        }
        if let Some(v) = search.min_rating { add_filter!("rating >= ?N", v); }
        if let Some(v) = search.max_rating { add_filter!("rating <= ?N", v); }
        if let Some(v) = search.min_word_count { add_filter!("word_count >= ?N", v); }
        if let Some(v) = search.max_word_count { add_filter!("word_count <= ?N", v); }
        if let Some(ref v) = search.date_from {
            if !v.is_empty() { add_filter!("date_read >= ?N", v.clone()); }
        }
        if let Some(ref v) = search.date_to {
            if !v.is_empty() { add_filter!("date_read <= ?N", v.clone()); }
        }

        let where_clause = if where_parts.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", where_parts.join(" AND "))
        };

        count_sql.push_str("SELECT COUNT(*) FROM books");
        count_sql.push_str(&where_clause);

        let bind_refs: Vec<&dyn rusqlite::types::ToSql> = bind_values.iter().map(|b| b.as_ref()).collect();
        let total: i64 = conn.query_row(&count_sql, bind_refs.as_slice(), |row| row.get(0))?;

        let page = search.page.unwrap_or(1).max(1);
        let page_size = search.page_size.unwrap_or(50).clamp(1, 200);
        let offset = (page - 1) * page_size;

        sql.push_str("SELECT id, title, author, category, tags, word_count, rating, date_read, reflection, created_at, updated_at FROM books");
        sql.push_str(&where_clause);
        sql.push_str(&format!(" ORDER BY date_read DESC, id DESC LIMIT {} OFFSET {}", page_size, offset));

        let bind_refs2: Vec<&dyn rusqlite::types::ToSql> = bind_values.iter().map(|b| b.as_ref()).collect();
        let mut stmt = conn.prepare_cached(&sql)?;
        let books: Vec<Book> = stmt.query_map(bind_refs2.as_slice(), |row| {
            Self::row_to_book(row)
        })?.filter_map(|r| r.ok()).collect();

        Ok((books, total))
    }

    pub fn get_all_books(&self) -> SqlResult<Vec<Book>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare_cached("SELECT id, title, author, category, tags, word_count, rating, date_read, reflection, created_at, updated_at FROM books ORDER BY date_read DESC, id DESC")?;
        let books: Vec<Book> = stmt.query_map([], |row| {
            Self::row_to_book(row)
        })?.filter_map(|r| r.ok()).collect();
        Ok(books)
    }

    fn row_to_book(row: &rusqlite::Row<'_>) -> rusqlite::Result<Book> {
        let tags_str: String = row.get(4)?;
        let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
        Ok(Book {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            author: row.get(2)?,
            category: row.get(3)?,
            tags,
            word_count: row.get(5)?,
            rating: row.get(6)?,
            date_read: row.get(7)?,
            reflection: row.get(8)?,
            created_at: Some(row.get(9)?),
            updated_at: Some(row.get(10)?),
        })
    }
}
