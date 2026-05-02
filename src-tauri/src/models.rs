use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: Option<i64>,
    pub title: String,
    pub author: String,
    pub category: String,
    pub tags: Vec<String>,
    pub word_count: i64,
    pub rating: i32,
    pub date_read: String,
    pub reflection: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub keyword: Option<String>,
    pub category: Option<String>,
    pub min_rating: Option<i32>,
    pub max_rating: Option<i32>,
    pub min_word_count: Option<i64>,
    pub max_word_count: Option<i64>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}
