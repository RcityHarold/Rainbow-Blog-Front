use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::article::Article;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Series {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub author_id: String,
    pub article_count: i32,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeriesArticle {
    pub article: Article,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeriesWithArticles {
    pub series: Series,
    pub articles: Vec<SeriesArticle>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateSeriesRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateSeriesRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
}