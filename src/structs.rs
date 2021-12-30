use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub message: Option<String>,
    pub response: Option<SearchResponse>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub status: String,
    pub user_tier: Option<String>,
    pub total: Option<u32>,
    pub start_index: Option<u32>,
    pub page_size: Option<u32>,
    pub current_page: Option<u32>,
    pub pages: Option<u32>,
    pub order_by: Option<String>,
    pub results: Option<Vec<SearchResult>>,
    pub message: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub byline: Option<String>,
    pub short_url: Option<String>,
    pub trail_text: Option<String>,
    pub headline: Option<String>,
    pub body: Option<String>,
    pub last_modified: Option<chrono::DateTime<Utc>>,
    pub has_story_package: Option<String>,
    pub score: Option<String>,
    pub standfirst: Option<String>,
    pub show_in_related_content: Option<String>,
    pub thumbnail: Option<String>,
    pub wordcount: Option<String>,
    pub commentable: Option<String>,
    pub is_premoderated: Option<String>,
    pub allow_ugc: Option<String>,
    pub publication: Option<String>,
    pub internal_page_code: Option<String>,
    pub production_office: Option<String>,
    pub should_hide_adverts: Option<String>,
    pub live_blogging_now: Option<String>,
    pub comment_close_date: Option<chrono::DateTime<Utc>>,
    pub star_rating: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub id: String,
    pub r#type: String,
    pub section_id: String,
    pub section_name: String,
    pub web_publication_date: chrono::DateTime<Utc>,
    pub web_title: String,
    pub web_url: String,
    pub api_url: String,
    pub is_hosted: bool,
    pub pillar_id: Option<String>,
    pub pillar_name: Option<String>,
    pub fields: Option<Fields>,
}