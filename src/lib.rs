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

use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::error::Error;
use std::string::ToString;
use crate::enums::*;
use crate::structs::*;

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
        headers.insert(
            "api-key",
            HeaderValue::from_str(&self.api_key[..]).unwrap(),
        );
        headers
    }

    fn generate_field_sequence(&self, fields: Vec<enums::Field>) -> String {
        let field_sequence = if fields.contains(&Field::All) {
            let all = Field::All;
            all.to_string()
        } else {
            fields
                .iter()
                .map(|enum_field| enum_field.to_string())
                .collect::<Vec<String>>()
                .join(",")
        };
        field_sequence
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
        let field_sequence = self.generate_field_sequence(show_fields);
        self.request
            .insert(String::from("show-fields"), String::from(field_sequence));
        self
    }

    pub fn query_fields(&mut self, query_fields: Vec<enums::Field>) -> &mut GuardianContentClient {
        let field_sequence = self.generate_field_sequence(query_fields);
        self.request
            .insert(String::from("query-fields"), String::from(field_sequence));
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
            .json()
            .await?;

        match search {
            Response {
                message: None,
                response: None,
            } => panic!("Placeholder"),
            Response {
                message: Some(x),
                response: _,
            } => panic!("{}", x),
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
