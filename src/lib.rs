use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::cell::{Cell, Ref, RefCell};
use std::collections::HashMap;
use std::error::Error;
use chrono::{DateTime, FixedOffset, ParseResult, Utc};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub response: SearchResponse,
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
}

pub struct GuardianContentClient {
    http_client: reqwest::Client,
    pub base_url: &'static str,
    api_key: String,
}

fn get_headers(client: &GuardianContentClient) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("api-key", HeaderValue::from_str(&client.api_key[..]).unwrap());
    headers
}

impl GuardianContentClient {
    pub fn new(api_key: &str) -> Result<GuardianContentClient, Box<dyn Error>> {
        let client = Self {
            http_client: Client::new(),
            base_url: "https://content.guardianapis.com",
            api_key: String::from(api_key),
        };
        Ok(client)
    }

    pub async fn search(&self, q: &str) -> Result<Vec<SearchResult>, Box<dyn Error>> {
        let search = self
            .http_client
            .get(format!("{}/search", self.base_url))
            .headers(get_headers(&self))
            .query(&[("q", q)])
            .send()
            .await?
            .json::<Response>()
            .await?;
        Ok(search.response.results)
    }

}
