//! Aletheia is a client library for the Guardian's content API.
//!
//! It is built on top of `reqwest` and provides a similar interface
//! for building queries.
//!
//! Responses returned by the client are serialized into structs
//! mirroring the types used by the API.
//!
//! # Example
//! ```
//! use std::error::Error;
//! use aletheia::GuardianContentClient;
//! use aletheia::enums::{Field, OrderBy, OrderDate};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let mut client = GuardianContentClient::new("your-api-key")?;
//!
//!     let response = client
//!         .search("rust")
//!         .page_size(10)
//!         .show_fields(vec![Field::Byline, Field::LastModified])
//!         .order_by(OrderBy::Newest)
//!         .order_date(OrderDate::Published)
//!         .send()
//!         .await?;
//!
//!     let results = response.results;
//!
//!     Ok(())
//! }
//! ```

pub mod enums;
pub mod structs;

use crate::enums::*;
use crate::structs::*;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::collections::HashMap;
use std::error::Error;
use std::string::ToString;

/// The main asynchronous client used to build requests to send to the Guardian's
/// content API. This client maintains an internal asynchronous client implemented
/// by `reqwest::Client`, but it is not publicly accessible.
#[derive(Debug)]
pub struct GuardianContentClient {
    http_client: reqwest::Client,
    api_key: String,
    pub base_url: String,
    pub request: HashMap<String, String>,
}

impl GuardianContentClient {
    /// Constructor for the client.
    /// The constructor takes an API key which is then stored internally
    /// in the struct. The client then uses the builder pattern
    /// to add query parameters to the request. These methods
    /// modify the client's internal structure, therefore
    /// the client should be initialised with the `mut` keyword.
    ///
    /// # Example
    /// ```
    /// let mut client = aletheia::GuardianContentClient("api-key-here");
    /// ```
    /// API keys for the Guardian's content API can be requested at
    /// <https://open-platform.theguardian.com/access/>
    pub fn new(api_key: &str) -> Result<GuardianContentClient, Box<dyn Error>> {
        let client = Self {
            http_client: Client::new(),
            base_url: String::from("https://content.guardianapis.com"),
            api_key: String::from(api_key),
            request: HashMap::new(),
        };
        Ok(client)
    }

    fn add_api_key_to_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if self.api_key.len() > 0 {
            headers.insert("api-key", HeaderValue::from_str(&self.api_key[..]).unwrap());
        }
        headers
    }

    pub fn search(&mut self, q: &str) -> &mut GuardianContentClient {
        self.request.insert(String::from("q"), String::from(q));
        self
    }

    pub fn page(&mut self, page: u32) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("page"), String::from(page.to_string()));
        self
    }

    /// Attaches a page size to the request.
    ///
    /// The page value must be between 0 and 200 for a successful response.
    /// This constraint is enforced upstream by the content API.
    pub fn page_size(&mut self, page: u8) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("page-size"), String::from(page.to_string()));
        self
    }

    pub fn order_by(&mut self, order_by: enums::OrderBy) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("order-by"), String::from(order_by.to_string()));
        self
    }

    pub fn order_date(&mut self, order_date: enums::OrderDate) -> &mut GuardianContentClient {
        self.request.insert(
            String::from("order-date"),
            String::from(order_date.to_string()),
        );
        self
    }

    pub fn show_fields(&mut self, show_fields: Vec<enums::Field>) -> &mut GuardianContentClient {
        let field_sequence = crate::helpers::generate_sequence(show_fields);
        self.request
            .insert(String::from("show-fields"), String::from(field_sequence));
        self
    }

    pub fn show_tags(&mut self, show_tags: Vec<enums::Tag>) -> &mut GuardianContentClient {
        let tag_sequence = crate::helpers::generate_sequence(show_tags);
        self.request
            .insert(String::from("show-tags"), String::from(tag_sequence));
        self
    }

    pub fn query_fields(&mut self, query_fields: Vec<enums::Field>) -> &mut GuardianContentClient {
        let field_sequence = crate::helpers::generate_sequence(query_fields);
        self.request
            .insert(String::from("query-fields"), String::from(field_sequence));
        self
    }

    pub fn date_from(&mut self, year: i32, month: u32, day: u32) -> &mut GuardianContentClient {
        self.request.insert(
            String::from("from-date"),
            format!("{}-{}-{}", year, month, day),
        );
        self
    }

    pub fn datetime_from(
        &mut self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        timezone: i32,
    ) -> &mut GuardianContentClient {
        let formatted_datetime =
            crate::helpers::datetime(year, month, day, hour, min, sec, timezone);
        self.request
            .insert(String::from("from-date"), formatted_datetime);
        self
    }

    pub fn date_to(&mut self, year: i32, month: u32, day: u32) -> &mut GuardianContentClient {
        self.request.insert(
            String::from("to-date"),
            format!("{}-{}-{}", year, month, day),
        );
        self
    }

    pub fn datetime_to(
        &mut self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        timezone: i32,
    ) -> &mut GuardianContentClient {
        let formatted_datetime =
            crate::helpers::datetime(year, month, day, hour, min, sec, timezone);
        self.request
            .insert(String::from("to-date"), formatted_datetime);
        self
    }

    pub fn use_date(&mut self, use_date: enums::UseDate) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("use-date"), String::from(use_date.to_string()));
        self
    }

    pub fn show_section(&mut self, show_section: bool) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("show-section"), String::from(show_section.to_string()));
        self
    }

    /// Terminal operation hitting the /search endpoint.
    /// Once this function is called, all the query parameters constructed
    /// via the building methods are dropped.
    pub async fn send(&mut self) -> Result<SearchResponse, Box<dyn Error>> {
        let queries = Vec::from_iter(self.request.iter());
        let search = self
            .http_client
            .get(format!("{}/search", self.base_url))
            .headers(self.add_api_key_to_headers())
            .query(&queries)
            .send()
            .await?
            .json::<Response>()
            .await?;

        crate::helpers::std_err(&search.message, &search.response);

        self.request.clear();

        match search.response {
            Some(r) => Ok(r),
            None => Ok(crate::helpers::mock_response()),
        }
    }
}

mod helpers {
    use crate::{enums, Field, SearchResponse, Tag};
    use chrono::{FixedOffset, TimeZone};

    pub(crate) fn std_err(message: &Option<String>, response: &Option<SearchResponse>) {
        if message.is_some() {
            eprintln!("Error: {}", message.as_ref().unwrap())
        }

        if response.is_some() {
            let response_content = response.as_ref().unwrap();
            if response_content.status == "error" && response_content.message.is_some() {
                eprintln!("Error: {}", response_content.message.as_ref().unwrap());
            }
        }
    }

    pub(crate) fn generate_sequence<T: std::fmt::Display>(items: Vec<T>) -> String {
        let items_to_strings = items.iter().map(|item| item.to_string());
        return if let Some(all) = items_to_strings.clone().find(|x| x == "all") {
            all
        } else {
            items_to_strings.collect::<Vec<String>>().join(",")
        };
    }

    pub(crate) fn datetime(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        timezone: i32,
    ) -> String {
        let offset: fn(i32) -> FixedOffset = if timezone >= 0 {
            FixedOffset::east
        } else {
            FixedOffset::west
        };

        offset(timezone.abs() * 3600)
            .ymd(year, month, day)
            .and_hms(hour, min, sec)
            .to_rfc3339()
    }

    pub(crate) fn mock_response() -> SearchResponse {
        SearchResponse {
            status: "".to_string(),
            user_tier: "".to_string(),
            total: 0,
            start_index: 0,
            page_size: 0,
            current_page: 0,
            pages: 0,
            order_by: "".to_string(),
            results: None,
            message: None
        }
    }
}
