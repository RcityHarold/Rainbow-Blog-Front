use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Article {
    #[serde(deserialize_with = "deserialize_thing_id")]
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub content: String,
    pub content_html: String,
    pub excerpt: Option<String>,
    pub cover_image_url: Option<String>,
    pub author: Author,
    pub publication: Option<Publication>,
    pub series: Option<Series>,
    pub tags: Vec<Tag>,
    pub status: String,
    pub is_paid_content: bool,
    pub is_featured: bool,
    pub reading_time: i32,
    pub word_count: i32,
    pub view_count: i32,
    pub clap_count: i32,
    pub comment_count: i32,
    pub bookmark_count: i32,
    pub share_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_description: Option<String>,
    #[serde(default)]
    pub seo_keywords: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub is_bookmarked: Option<bool>,
    #[serde(default)]
    pub is_clapped: Option<bool>,
    #[serde(default)]
    pub user_clap_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    #[serde(deserialize_with = "deserialize_thing_id")]
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Publication {
    #[serde(deserialize_with = "deserialize_thing_id")]
    pub id: String,
    pub name: String,
    pub slug: String,
    pub logo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Series {
    #[serde(deserialize_with = "deserialize_thing_id")]
    pub id: String,
    pub title: String,
    pub slug: String,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    #[serde(deserialize_with = "deserialize_thing_id")]
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleListResponse {
    pub articles: Vec<Article>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pagination {
    pub current_page: i32,
    pub total_pages: i32,
    pub total_items: i32,
    pub items_per_page: i32,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub subtitle: Option<String>,
    pub content: String,
    pub excerpt: Option<String>,
    pub cover_image_url: Option<String>,
    pub publication_id: Option<String>,
    pub series_id: Option<String>,
    pub series_order: Option<i32>,
    pub is_paid_content: bool,
    pub tags: Vec<String>,
    pub save_as_draft: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo_keywords: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub cover_image_url: Option<String>,
    pub publication_id: Option<String>,
    pub series_id: Option<String>,
    pub series_order: Option<i32>,
    pub is_paid_content: Option<bool>,
    pub tags: Option<Vec<String>>,
}

/// 处理 SurrealDB Thing ID 的反序列化
fn deserialize_thing_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct ThingIdVisitor;

    impl<'de> Visitor<'de> for ThingIdVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or a SurrealDB Thing object")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // 处理包含 JSON 的字符串格式，如: "article:{\"String\":\"uuid\"}"
            if value.contains(":{\"") && value.contains("\"}") {
                // 提取表名和 ID
                if let Some(colon_pos) = value.find(':') {
                    let table = &value[..colon_pos];
                    let json_part = &value[colon_pos + 1..];
                    
                    // 尝试解析 JSON 部分
                    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(json_part) {
                        if let Some(id_str) = json_value.get("String").and_then(|v| v.as_str()) {
                            return Ok(format!("{}:{}", table, id_str));
                        }
                    }
                }
            }
            
            // 如果不是特殊格式，直接返回
            Ok(value.to_string())
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            let mut tb = None;
            let mut id = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "tb" => {
                        tb = Some(map.next_value::<String>()?);
                    }
                    "id" => {
                        // id 可能是字符串或对象
                        let value = map.next_value::<serde_json::Value>()?;
                        id = Some(match value {
                            serde_json::Value::String(s) => s,
                            serde_json::Value::Object(obj) => {
                                obj.get("String")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("unknown")
                                    .to_string()
                            }
                            _ => value.to_string(),
                        });
                    }
                    _ => {
                        let _: serde_json::Value = map.next_value()?;
                    }
                }
            }

            match (tb, id) {
                (Some(table), Some(id_val)) => Ok(format!("{}:{}", table, id_val)),
                _ => Err(de::Error::custom("Missing table or id in Thing object")),
            }
        }
    }

    deserializer.deserialize_any(ThingIdVisitor)
}