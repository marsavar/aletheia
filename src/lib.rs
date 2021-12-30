use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::error::Error;
use std::string::ToString;
use strum_macros::Display;

#[derive(Display, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "camelCase")]
pub enum Field {
    TrailText,
    Headline,
    ShowInRelatedContent,
    Body,
    LastModified,
    HasStoryPackage,
    Score,
    StandFirst,
    ShortUrl,
    Byline,
    Thumbnail,
    Wordcount,
    Commentable,
    IsPremoderated,
    AllowUgc,
    Publication,
    InternalPageCode,
    ProductionOffice,
    ShouldHideAdverts,
    LiveBloggingNow,
    CommentCloseDate,
    StarRating,
    All,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub message: Option<String>,
    pub response: Option<SearchResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub status: String,
    pub user_tier: String,
    pub total: u32,
    pub start_index: u32,
    pub page_size: u32,
    pub current_page: u32,
    pub pages: u32,
    pub order_by: String,
    pub results: Vec<SearchResult>,
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

#[derive(Debug)]
pub struct GuardianContentClient {
    pub http_client: reqwest::Client,
    pub base_url: String,
    api_key: String,
    pub request: HashMap<String, String>,
}

fn get_headers(client: &GuardianContentClient) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "api-key",
        HeaderValue::from_str(&client.api_key[..]).unwrap(),
    );
    headers
}

impl GuardianContentClient {
    pub fn new(api_key: &str) -> Result<GuardianContentClient, Box<dyn Error>> {
        let client = Self {
            http_client: Client::new(),
            base_url: String::from("https://content.guardianapis.com"),
            api_key: String::from(api_key),
            request: HashMap::new(),
        };
        Ok(client)
    }

    pub fn search(&mut self, q: &str) -> &mut GuardianContentClient {
        self.request.insert(String::from("q"), String::from(q));
        self
    }

    pub fn show_fields(&mut self, show_fields: Vec<Field>) -> &mut GuardianContentClient {
        let field_sequence = if show_fields.contains(&Field::All) {
            let all = Field::All;
            all.to_string()
        } else {
            show_fields
                .iter()
                .map(|enum_field| enum_field.to_string())
                .collect::<Vec<String>>()
                .join(",")
        };

        self.request
            .insert(String::from("show-fields"), String::from(field_sequence));
        self
    }

    pub async fn send(&mut self) -> Result<SearchResponse, Box<dyn Error>> {
        let queries = Vec::from_iter(self.request.iter());
        let search = self
            .http_client
            .get(format!("{}/search", self.base_url))
            .headers(get_headers(&self))
            .query(&queries)
            .send()
            .await?
            .json()
            .await?;

        match search {
            Response {
                message: None,
                response: None,
            } => panic!("No response from the server."),
            Response {
                message: Some(x),
                response: _,
            } => panic!("{:?}", x),
            Response {
                message: _,
                response: Some(x),
            } => {
                self.request.clear();
                Ok(x)
            }
        }
    }
}
