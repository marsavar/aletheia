use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::string::ToString;
use strum_macros::Display;

#[derive(Display, Debug)]
pub enum Field {
    #[strum(serialize = "trailText")]
    TrailText,
    #[strum(serialize = "headLine")]
    Headline,
    #[strum(serialize = "showInRelatedContent")]
    ShowInRelatedContent,
    #[strum(serialize = "body")]
    Body,
    #[strum(serialize = "lastModified")]
    LastModified,
    #[strum(serialize = "hasStoryPackage")]
    HasStoryPackage,
    #[strum(serialize = "score")]
    Score,
    #[strum(serialize = "standFirst")]
    StandFirst,
    #[strum(serialize = "shortUrl")]
    ShortUrl,
    #[strum(serialize = "byline")]
    Byline,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub byline: Option<String>,
    pub short_url: Option<String>,
    pub trail_text: Option<String>,
    pub headline: Option<String>,
    pub body: String,
}

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
    http_client: reqwest::Client,
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
        self.request.insert(
            String::from("show-fields"),
            String::from(
                show_fields
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ),
        );
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
            .json::<Response>()
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
            } => Ok(x),
        }
    }
}
