//! Aletheia is a client library for the Guardian's content API.
//!
//! It is built on top of `reqwest` and provides a similar interface
//! for building queries.
//!
//! Responses returned by the client are deserialized into structs
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
/// content API. This client maintains a private internal asynchronous client
/// implemented by `reqwest::Client`
#[derive(Debug)]
pub struct GuardianContentClient {
    http_client: reqwest::Client,
    api_key: String,
    base_url: String,
    request: HashMap<String, String>,
    endpoint: Endpoint,
}

impl GuardianContentClient {
    /// Constructor for the client.
    /// The constructor takes an API key which is then stored internally
    /// in the struct. The client then uses the builder pattern
    /// to add query parameters to the request. These methods
    /// modify the client's internal structure, therefore
    /// the client should be initialised with the `mut` keyword.
    ///
    /// API keys for the Guardian's content API can be requested at
    /// <https://open-platform.theguardian.com/access/>
    ///
    /// # Example
    /// ```
    /// let mut client = aletheia::GuardianContentClient("api-key-here")?;
    /// ```
    pub fn new(api_key: &str) -> Result<GuardianContentClient, Box<dyn Error>> {
        let client = Self {
            http_client: Client::new(),
            base_url: String::from("https://content.guardianapis.com"),
            api_key: String::from(api_key),
            request: HashMap::new(),
            endpoint: Endpoint::Content,
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

    /// Add a search query to the request.
    ///
    /// Supports AND, OR and NOT operators, and exact phrase queries using double quotes.
    /// Examples of valid queries:
    /// - "Barack Obama"
    /// - Music
    /// - Programming AND coding
    ///
    /// This field is only valid for the following endpoints:
    ///
    /// - `Endpoint::Content` (default endpoint, no need to explicitly set it)
    /// - `Endpoint::Tags`
    /// - `Endpoint::Sections`
    /// - `Endpoint::Editions`
    ///
    /// Calling this method on `Endpoint::SingleItem` will
    /// have no effect.
    ///
    /// # Example
    /// ```
    /// let response = client
    ///         .search("Elections")
    ///         .send()
    ///         .await?;
    /// ```
    pub fn search(&mut self, q: &str) -> &mut GuardianContentClient {
        self.request.insert(String::from("q"), q.to_string());
        self
    }

    /// Add a page number to the request.
    ///
    /// Results are returned as a paginated list, with a default of 10 results.
    /// In order to page through the results, you can pass the page number
    /// as a parameter to this function.
    ///
    /// # Example
    /// ```
    /// let response = client
    ///         .search("Elections")
    ///         .page(10)
    ///         .send()
    ///         .await?;
    /// ```
    ///
    pub fn page(&mut self, page: u32) -> &mut GuardianContentClient {
        self.request.insert(String::from("page"), page.to_string());
        self
    }

    /// Attach a page size to the request.
    ///
    /// Results are returned as a paginated list, with a default of 10 results.
    /// This function overrides the default.
    /// The page value must be between 0 and 200 for a successful response.
    ///
    /// # Example
    /// ```
    /// let response = client
    ///         .search("Elections")
    ///         .page_size(20)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn page_size(&mut self, page: u8) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("page-size"), page.to_string());
        self
    }

    /// Return results in the specified order.
    ///
    /// The function only accepts one of three `aletheia::enums` enum values:
    /// - `OrderBy::Oldest`
    /// - `OrderBy::Oldest`
    /// - `OrderBy::Relevance`
    ///
    /// # Example
    /// ```
    /// let response = client
    ///         .search("Elections")
    ///         .order_by(OrderBy::Oldest)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn order_by(&mut self, order_by: enums::OrderBy) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("order-by"), order_by.to_string());
        self
    }


    /// Change which type of date is used to order the results
    ///
    /// The function only accepts one of three `aletheia::enums` enum values:
    /// - `OrderDate::Published`
    /// - `OrderDate::NewspaperEdition`
    /// - `OrderDate::LastModified`
    ///
    /// # Example
    /// ```
    /// let response = client
    ///         .search("Elections")
    ///         .order_by(OrderDate::NewspaperEdition)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn order_date(&mut self, order_date: enums::OrderDate) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("order-date"), order_date.to_string());
        self
    }

    /// Add fields associated with the content.
    ///
    /// The function accepts a vector of `aletheia::enums` values of type `Field`,
    /// e.g.
    /// - `Field::TrailText`
    /// - `Field::Body`
    /// - `Field::Byline`
    ///
    /// If `Field::All` is included in the vector, it will override all other fields.
    ///
    /// See <https://open-platform.theguardian.com/documentation/search>
    /// for more information on all the possible fields,
    /// or check the `aletheia::enums` section of the documentation.
    ///
    /// # Example
    /// ```
    /// let response = client
    ///         .search("Elections")
    ///         .show_fields(vec![Field::StarRating, Field::ShortUrl])
    ///         .send()
    ///         .await?;
    /// ```
    pub fn show_fields(&mut self, show_fields: Vec<enums::Field>) -> &mut GuardianContentClient {
        let field_sequence = crate::helpers::generate_sequence(show_fields);
        self.request
            .insert(String::from("show-fields"), field_sequence);
        self
    }

    /// Add associated metadata tags.
    ///
    /// The function accepts a vector of `aletheia::enums` values of type `Tag`,
    /// e.g.
    /// - `Tag::Blog`
    /// - `Tag::Contributor`
    /// - `Tag::Tone`
    ///
    /// If `Tag::All` is included in the vector, it will override all other tags.
    ///
    /// See <https://open-platform.theguardian.com/documentation/search>
    /// for more information on all the possible tags,
    /// or check the `aletheia::enums` section of the documentation.
    ///
    /// # Example
    /// ```
    /// let response = client
    ///         .search("Elections")
    ///         .show_tags(vec![Tag::Contributor, Tag::Type, Tag::Tone])
    ///         .send()
    ///         .await?;
    /// ```
    pub fn show_tags(&mut self, show_tags: Vec<enums::Tag>) -> &mut GuardianContentClient {
        let tag_sequence = crate::helpers::generate_sequence(show_tags);
        self.request.insert(String::from("show-tags"), tag_sequence);
        self
    }

    /// Specify in which indexed fields query terms should be searched on
    ///
    /// The function accepts a vector of `aletheia::enums` values of type `Field`,
    /// e.g.
    /// - `Field::TrailText`
    /// - `Field::Body`
    /// - `Field::Byline`
    ///
    /// See <https://open-platform.theguardian.com/documentation/search>
    /// for more information on all the possible fields,
    /// or check the `aletheia::enums` section of the documentation.
    ///
    /// # Example
    /// ```
    /// let response = client
    ///         .search("Elections")
    ///         .query_fields(vec![Field::Body])
    ///         .send()
    ///         .await?;
    /// ```
    pub fn query_fields(&mut self, query_fields: Vec<enums::Field>) -> &mut GuardianContentClient {
        let field_sequence = crate::helpers::generate_sequence(query_fields);
        self.request
            .insert(String::from("query-fields"), field_sequence);
        self
    }

    /// Return only content published on or after that date.
    ///
    /// # Example
    /// ```
    /// let response = client
    ///         .search("Elections")
    ///         .date_from(2020, 1, 1)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn date_from(&mut self, year: i32, month: u32, day: u32) -> &mut GuardianContentClient {
        self.request.insert(
            String::from("from-date"),
            format!("{}-{}-{}", year, month, day),
        );
        self
    }

    /// Return only content published on or after that date.
    ///
    /// It is more specific than date_from() as it accepts
    /// hours, minutes, seconds as well as a timezone offset.
    ///
    /// # Example
    /// ```
    /// let response = client
    ///         .search("Elections")
    ///         .datetime_from(2020, 1, 1, 12, 0, 0, 2)
    ///         .send()
    ///         .await?;
    /// ```
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

    /// Return only content published on or before that date.
    ///
    /// # Example
    /// ```
    /// let response = client
    ///         .search("Elections")
    ///         .date_from(2008, 1, 1)
    ///         .date_to(2010, 12, 31)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn date_to(&mut self, year: i32, month: u32, day: u32) -> &mut GuardianContentClient {
        self.request.insert(
            String::from("to-date"),
            format!("{}-{}-{}", year, month, day),
        );
        self
    }

    /// Return only content published on or before that date.
    ///
    /// It is more specific than datetime_to() as it accepts
    /// hours, minutes, seconds as well as a timezone offset.
    ///
    /// # Example
    /// ```
    /// let response = client
    ///         .search("Elections")
    ///         .datetime_to(2016, 1, 1, 12, 0, 0, 5)
    ///         .send()
    ///         .await?;
    /// ```
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
            .insert(String::from("use-date"), use_date.to_string());
        self
    }

    pub fn show_section(&mut self, show_section: bool) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("show-section"), show_section.to_string());
        self
    }

    pub fn section(&mut self, section: &str) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("section"), section.to_string());
        self
    }

    pub fn reference(&mut self, reference: &str) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("reference"), reference.to_string());
        self
    }

    pub fn reference_type(&mut self, reference_type: &str) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("reference-type"), reference_type.to_string());
        self
    }

    pub fn tag(&mut self, tag: &str) -> &mut GuardianContentClient {
        self.request.insert(String::from("tag"), tag.to_string());
        self
    }

    pub fn ids(&mut self, ids: &str) -> &mut GuardianContentClient {
        self.request.insert(String::from("ids"), ids.to_string());
        self
    }

    pub fn production_office(&mut self, production_office: &str) -> &mut GuardianContentClient {
        self.request.insert(
            String::from("production-office"),
            production_office.to_string(),
        );
        self
    }

    pub fn lang(&mut self, lang: &str) -> &mut GuardianContentClient {
        self.request.insert(String::from("lang"), lang.to_string());
        self
    }

    pub fn star_rating(&mut self, star_rating: u8) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("star-rating"), star_rating.to_string());
        self
    }

    pub fn r#type(&mut self, r#type: &str) -> &mut GuardianContentClient {
        self.request
            .insert(String::from("type"), r#type.to_string());
        self
    }

    pub fn show_blocks(&mut self, show_blocks: Vec<enums::Block>) -> &mut GuardianContentClient {
        let block_sequence = crate::helpers::generate_blocks(show_blocks);
        self.request
            .insert(String::from("show-blocks"), block_sequence);
        self
    }

    pub fn endpoint(&mut self, endpoint: enums::Endpoint) -> &mut GuardianContentClient {
        self.endpoint = endpoint;
        self
    }

    /// Terminal operation that sends a GET request to the Guardian API.
    /// Once this function is called, all the query parameters constructed
    /// via the building methods are dropped.
    pub async fn send(&mut self) -> Result<SearchResponse, Box<dyn Error>> {
        let endpoint = match self.endpoint {
            Endpoint::Content => String::from("search"),
            Endpoint::Tags => String::from(self.endpoint.to_string()),
            Endpoint::Sections => String::from(self.endpoint.to_string()),
            Endpoint::Editions => String::from(self.endpoint.to_string()),
            Endpoint::SingleItem => self.request.get("q").unwrap().to_string(),
        };

        let queries = Vec::from_iter(self.request.iter());

        let search = self
            .http_client
            .get(format!("{}/{}", self.base_url, endpoint))
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
    use crate::enums::Block;
    use crate::{enums, SearchResponse};
    use chrono::{FixedOffset, TimeZone};

    pub(crate) fn std_err(message: &Option<String>, response: &Option<SearchResponse>) {
        if message.is_some() {
            eprintln!("Error: {}", message.as_ref().unwrap())
        }

        if response.is_some() {
            let response_content = response.as_ref().unwrap();
            if response_content.status.is_some() {
                if response_content.status.as_ref().unwrap() == "error"
                    && response_content.message.is_some()
                {
                    eprintln!("Error: {}", response_content.message.as_ref().unwrap());
                }
            }
        }
    }

    pub(crate) fn generate_sequence<T: std::fmt::Display>(items: Vec<T>) -> String {
        let items_to_strings = items
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();
        return if items_to_strings.contains(&String::from("all")) {
            String::from("all")
        } else {
            items_to_strings.join(",")
        };
    }

    pub(crate) fn generate_blocks(items: Vec<enums::Block>) -> String {
        let items_to_strings = items
            .into_iter()
            .map(|item| match item {
                Block::Main => item.to_string(),
                Block::Body => item.to_string(),
                Block::All => item.to_string(),
                Block::BodyLatest => String::from("body:latest"),
                Block::BodyLatestWith(n) => format!("body:latest:{}", n),
                Block::BodyOldest => String::from("body:latest"),
                Block::BodyOldestWith(n) => format!("body:oldest:{}", n),
                Block::BodyBlockId(id) => format!("body:{}", id),
                Block::BodyAroundBlockId(id) => format!("body:around:{}", id),
                Block::BodyAroundBlockIdWith(id, n) => {
                    format!("body:around:{}:{}", String::from(id), n)
                }
                Block::BodyKeyEvents => String::from("body:key-events"),
                Block::BodyPublishedSince(n) => format!("body:published-since:{}", n),
            })
            .collect::<Vec<String>>();

        return if items_to_strings.contains(&String::from("all")) {
            String::from("all")
        } else {
            items_to_strings.join(",")
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
            status: None,
            user_tier: None,
            total: None,
            start_index: None,
            page_size: None,
            current_page: None,
            pages: None,
            order_by: None,
            results: None,
            message: None,
            content: None,
        }
    }
}
