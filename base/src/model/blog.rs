use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx::FromRow;

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPostPreview {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub thumbnail_url: String,
    pub created_at: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPostPreviewResponseBody {
    pub posts: Vec<BlogPostPreview>
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPostResponseBody {
    pub slug: String,
    pub title: String,
    pub tags: Vec<String>,
    pub thumbnail_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPostCreateRequestBody {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub thumbnail_url: String,
    pub content: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPostUpdateRequestBody {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub thumbnail_url: String,
    pub content: String,
    pub hash: String,
}

#[cfg_attr(feature = "sqlx", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPostHash {
    pub slug: String,
    pub hash: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPostHashesResponseBody {
    pub posts: Vec<BlogPostHash>
}
