//! Aletheia is a client library for the Guardian's content API.
//!
//! It is built on top of `reqwest` and provides a similar interface
//! for building queries. Responses returned by the client are deserialized
//! into structs, mirroring the types used by the API.
//!
//! Keys to start querying the API can be obtained here:
//! <https://open-platform.theguardian.com/access/>
//!
//! More information on the API can be found here:
//! <https://open-platform.theguardian.com/documentation/>
//!
//! # Example
//! ```ignore
//! use aletheia::error::Error;
//! use aletheia::GuardianContentClient;
//! use aletheia::enums::{Field, OrderBy, OrderDate};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let client = GuardianContentClient::new("YOUR_API_KEY");
//!
//!     let response = client
//!         .build_request()
//!         .search("Elections")
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
pub mod error;
pub mod structs;
mod tests;

use crate::enums::*;
use crate::error::Error;
use crate::structs::*;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use std::collections::HashMap;
use std::string::ToString;

const GUARDIAN_CONTENT_API_URL: &str = "https://content.guardianapis.com";

/// The main asynchronous client used to build requests to send to the Guardian's
/// content API. This client maintains a private internal asynchronous client
/// implemented by [`reqwest::Client`]
#[derive(Clone, Debug)]
pub struct GuardianContentClient {
    http_client: reqwest::Client,
    api_key: String,
    base_url: Url,
}

#[derive(Debug, Clone)]
pub struct GuardianRequestBuilder {
    http_client: Client,
    base_url: Url,
    api_key: String,
    request: HashMap<String, String>,
    endpoint: Endpoint,
}

impl GuardianRequestBuilder {
    /// Specify the Guardian API endpoint to target.
    ///
    /// Can be one of:
    /// - [`Endpoint::Content`] (default): returns all pieces of content in the API.
    /// - [`Endpoint::Tags`]: returns all tags in the API. All Guardian content is manually
    ///   categorised using these tags, of which there are more than 50,000.
    /// - [`Endpoint::Sections`]:  returns all sections in the API.
    /// - [`Endpoint::Editions`]: returns all editions in the API. Editions are the different
    ///   front pages of the Guardian site (currently, there are editions for the United
    ///   Kingdom, the United States and Australia).
    /// - [`Endpoint::SingleItem`]: returns all the data for a given single item id.
    ///   Here the term 'item' refers to either a piece of content, a tag, or a section.
    ///   The item endpoint matches the paths on theguardian.com.
    ///
    /// # Example 1
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .endpoint(Endpoint::Tags)
    ///         .search("food")
    ///         .send()
    ///         .await?;
    /// ```
    ///
    /// # Example 2
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .endpoint(Endpoint::SingleItem)
    ///         .search("books/2022/jan/01/2022-in-books-highlights-for-the-year-ahead")
    ///         .send()
    ///         .await?;
    /// ```
    pub fn endpoint(mut self, endpoint: Endpoint) -> Self {
        self.endpoint = endpoint;
        self
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
    /// - [`Endpoint::Content`] (default endpoint, no need to explicitly set it)
    /// - [`Endpoint::Tags`]
    /// - [`Endpoint::Sections`]
    /// - [`Endpoint::Editions`]
    ///
    /// Calling this method on [`Endpoint::SingleItem`] will
    /// have no effect.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .send()
    ///         .await?;
    /// ```
    pub fn search(mut self, q: &str) -> Self {
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
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .page(10)
    ///         .send()
    ///         .await?;
    /// ```
    ///
    pub fn page(mut self, page: u32) -> GuardianRequestBuilder {
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
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .page_size(20)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn page_size(mut self, page: u8) -> GuardianRequestBuilder {
        self.request
            .insert(String::from("page-size"), page.to_string());
        self
    }

    /// Return results in the specified order.
    ///
    /// The function only accepts one of three [`enums::OrderBy`] variants:
    /// - [`OrderBy::Newest`]
    /// - [`OrderBy::Oldest`]
    /// - [`OrderBy::Relevance`]
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .order_by(OrderBy::Oldest)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn order_by(mut self, order_by: OrderBy) -> GuardianRequestBuilder {
        self.request
            .insert(String::from("order-by"), order_by.to_string());
        self
    }

    /// Change which type of date is used to order the results
    ///
    /// The function only accepts one of three [`enums::OrderDate`] variants:
    /// - [`OrderDate::Published`]
    /// - [`OrderDate::NewspaperEdition`]
    /// - [`OrderDate::LastModified`]
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .order_by(OrderDate::NewspaperEdition)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn order_date(mut self, order_date: OrderDate) -> GuardianRequestBuilder {
        self.request
            .insert(String::from("order-date"), order_date.to_string());
        self
    }

    /// Add fields associated with the content.
    ///
    /// The function accepts a vector of [`enums::Field`] variants,
    /// e.g.
    /// - [`Field::TrailText`]
    /// - [`Field::Body`]
    /// - [`Field::Byline`]
    ///
    /// If [`Field::All`] is included in the vector, it will override all other fields.
    ///
    /// See <https://open-platform.theguardian.com/documentation/search>
    /// for more information on all the possible fields,
    /// or check the [`crate::enums`] section of the documentation.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .show_fields(vec![Field::StarRating, Field::ShortUrl])
    ///         .send()
    ///         .await?;
    /// ```
    pub fn show_fields(mut self, show_fields: Vec<Field>) -> GuardianRequestBuilder {
        let field_sequence = crate::helpers::generate_sequence(show_fields);
        self.request
            .insert(String::from("show-fields"), field_sequence);
        self
    }

    /// Add associated metadata tags.
    ///
    /// The function accepts a vector of [`enums::Tag`] variants,
    /// e.g.
    /// - [`Tag::Blog`]
    /// - [`Tag::Contributor`]
    /// - [`Tag::Tone`]
    ///
    /// If [`Tag::All`] is included in the vector, it will override all other tags.
    ///
    /// See <https://open-platform.theguardian.com/documentation/search>
    /// for more information on all the possible tags,
    /// or check the [`crate::enums`] section of the documentation.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .show_tags(vec![Tag::Contributor, Tag::Type, Tag::Tone])
    ///         .send()
    ///         .await?;
    /// ```
    pub fn show_tags(mut self, show_tags: Vec<enums::Tag>) -> GuardianRequestBuilder {
        let tag_sequence = crate::helpers::generate_sequence(show_tags);
        self.request.insert(String::from("show-tags"), tag_sequence);
        self
    }

    /// Specify in which indexed fields query terms should be searched on
    ///
    /// The function accepts a vector of [`enums::Field`] variants,
    /// e.g.
    /// - [`Field::TrailText`]
    /// - [`Field::Body`]
    /// - [`Field::Byline`]
    ///
    /// See <https://open-platform.theguardian.com/documentation/search>
    /// for more information on all the possible fields,
    /// or check the [`crate::enums`] section of the documentation.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .query_fields(vec![Field::Body])
    ///         .send()
    ///         .await?;
    /// ```
    pub fn query_fields(mut self, query_fields: Vec<Field>) -> GuardianRequestBuilder {
        let field_sequence = crate::helpers::generate_sequence(query_fields);
        self.request
            .insert(String::from("query-fields"), field_sequence);
        self
    }

    /// Only return content published on or after the specified date.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .date_from(2020, 1, 1)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn date_from(mut self, year: i32, month: u32, day: u32) -> GuardianRequestBuilder {
        self.request
            .insert(String::from("from-date"), format!("{year}-{month}-{day}"));
        self
    }

    /// Only return content published on or after the specified date.
    ///
    /// It is more specific than `date_from()` as it accepts
    /// hours, minutes, seconds as well as a timezone offset.
    ///
    /// Note: Providing invalid YMD-HMS does not append query parameters
    /// to the API request.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .datetime_from(2020, 1, 1, 12, 0, 0, 2)
    ///         .send()
    ///         .await?;
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn datetime_from(
        mut self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        timezone: i32,
    ) -> GuardianRequestBuilder {
        let formatted_datetime =
            crate::helpers::datetime(year, month, day, hour, min, sec, timezone);

        if !formatted_datetime.is_empty() {
            self.request
                .insert(String::from("from-date"), formatted_datetime);
        }
        self
    }

    /// Only return content published on or before the specified date.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .date_from(2008, 1, 1)
    ///         .date_to(2010, 12, 31)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn date_to(mut self, year: i32, month: u32, day: u32) -> GuardianRequestBuilder {
        self.request
            .insert(String::from("to-date"), format!("{year}-{month}-{day}"));
        self
    }

    /// Only return content published on or before the specified date.
    ///
    /// It is more specific than `date_to()` as it accepts
    /// hours, minutes, seconds as well as a timezone offset.
    ///
    /// Note: Providing invalid YMD-HMS does not append query parameters
    /// to the API request.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .datetime_to(2016, 1, 1, 12, 0, 0, 5)
    ///         .send()
    ///         .await?;
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn datetime_to(
        mut self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        timezone: i32,
    ) -> GuardianRequestBuilder {
        let formatted_datetime =
            crate::helpers::datetime(year, month, day, hour, min, sec, timezone);

        if !formatted_datetime.is_empty() {
            self.request
                .insert(String::from("to-date"), formatted_datetime);
        }

        self
    }

    /// Change which type of date is used to filter the results using `date_from()`,
    /// `datetime_from()`, `date_to()` and `datetime_to()`.
    ///
    /// The function only accepts one of four [`enums::UseDate`] variants:
    /// - [`UseDate::Published`] (default)
    /// - [`UseDate::FirstPublication`]
    /// - [`UseDate::NewspaperEdition`]
    /// - [`UseDate::LastModified`]
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .date_from(2015, 1, 1)
    ///         .date_to(2018, 12, 31)
    ///         .use_date(UseDate::FirstPublication)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn use_date(mut self, use_date: UseDate) -> GuardianRequestBuilder {
        self.request
            .insert(String::from("use-date"), use_date.to_string());
        self
    }

    /// Add associated metadata section.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .show_section(true)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn show_section(mut self, show_section: bool) -> GuardianRequestBuilder {
        self.request
            .insert(String::from("show-section"), show_section.to_string());
        self
    }

    /// Return only content in those sections.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .section("football")
    ///         .send()
    ///         .await?;
    /// ```
    pub fn section(mut self, section: &str) -> GuardianRequestBuilder {
        self.request
            .insert(String::from("section"), section.to_string());
        self
    }

    /// Return only content with those references.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .reference("isbn/9780718178949")
    ///         .send()
    ///         .await?;
    /// ```
    pub fn reference(mut self, reference: &str) -> GuardianRequestBuilder {
        self.request
            .insert(String::from("reference"), reference.to_string());
        self
    }

    /// Return only content with references of those types.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .reference_type("isbn")
    ///         .send()
    ///         .await?;
    /// ```
    pub fn reference_type(mut self, reference_type: &str) -> GuardianRequestBuilder {
        self.request
            .insert(String::from("reference-type"), reference_type.to_string());
        self
    }

    /// Return only content with those tags.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .tag("technology/apple")
    ///         .send()
    ///         .await?;
    /// ```
    pub fn tag(mut self, tag: &str) -> GuardianRequestBuilder {
        self.request.insert(String::from("tag"), tag.to_string());
        self
    }

    /// Return only content with those IDs.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .ids("world/2022/jan/01/funeral-of-desmond-tutu-takes-place-in-cape-town")
    ///         .send()
    ///         .await?;
    /// ```
    pub fn ids(mut self, ids: &str) -> GuardianRequestBuilder {
        self.request.insert(String::from("ids"), ids.to_string());
        self
    }

    /// Return only content from those production offices.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .production_office("UK")
    ///         .send()
    ///         .await?;
    /// ```
    pub fn production_office(mut self, production_office: &str) -> GuardianRequestBuilder {
        self.request.insert(
            String::from("production-office"),
            production_office.to_string(),
        );
        self
    }

    /// Return only content in those languages.
    /// Accepts ISO language codes, e.g. en, fr.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .lang("en")
    ///         .send()
    ///         .await?;
    /// ```
    pub fn lang(mut self, lang: &str) -> GuardianRequestBuilder {
        self.request.insert(String::from("lang"), lang.to_string());
        self
    }

    /// Return only content with a given star rating
    /// ranging from 1 to 5.
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .star_rating(5)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn star_rating(mut self, star_rating: u8) -> GuardianRequestBuilder {
        self.request
            .insert(String::from("star-rating"), star_rating.to_string());
        self
    }

    /// Only return tags of the specified type.
    /// Only valid if the endpoint is set to [`Endpoint::Tags`]
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .endpoint(Endpoint::Tags)
    ///         .search("Elections")
    ///         .tag_type("tv-and-radio/us-television")
    ///         .send()
    ///         .await?;
    ///
    /// ```
    pub fn tag_type(mut self, r#type: &str) -> GuardianRequestBuilder {
        self.request
            .insert(String::from("type"), r#type.to_string());
        self
    }

    /// Add associated blocks (single block for content, one or more for liveblogs).
    ///
    /// Supports the following [`enums::Block`] variants:
    ///
    /// - [`Block::Main`]
    /// - [`Block::Body`]
    /// - [`Block::All`]
    /// - [`Block::BodyLatest`] (limit defaults to 20)
    /// - [`Block::BodyLatestWith(i32)`] (override the limits)
    /// - [`Block::BodyOldest`]
    /// - [`Block::BodyOldestWith(i32)`]
    /// - [`Block::BodyBlockId(&'a str)`] (only the block with that ID)
    /// - [`Block::BodyAroundBlockId(&'a str)`] (the specified block and 20 blocks either side of it)
    /// - [`Block::BodyAroundBlockIdWith(&'a str, i32)`] (the specified block and n blocks either side of it)
    /// - [`Block::BodyKeyEvents`]
    /// - [`Block::BodyPublishedSince(i64)`]  (only blocks since given timestamp)
    ///
    /// # Example
    /// ```ignore
    /// let response = client
    ///         .build_request()
    ///         .endpoint(Endpoint::Tags)
    ///         .search("Elections")
    ///         .show_blocks(Block::BodyPublishedSince(1556529318000))
    ///         .send()
    ///         .await?;
    /// ```
    pub fn show_blocks(mut self, show_blocks: Vec<enums::Block>) -> GuardianRequestBuilder {
        let block_sequence = crate::helpers::generate_blocks(show_blocks);
        self.request
            .insert(String::from("show-blocks"), block_sequence);
        self
    }

    /// Terminal operation that sends a GET request to the Guardian API.
    /// Once this function is called, all the query parameters constructed
    /// via the building methods are dropped.
    pub async fn send(&mut self) -> Result<SearchResponse, Error> {
        let mut headers = HeaderMap::new();
        if !self.api_key.is_empty() {
            headers.insert("api-key", HeaderValue::from_str(&self.api_key).unwrap());
        }

        let endpoint = match self.endpoint {
            Endpoint::Content => String::from("search"),
            Endpoint::Tags => self.endpoint.to_string(),
            Endpoint::Sections => self.endpoint.to_string(),
            Endpoint::Editions => self.endpoint.to_string(),
            Endpoint::SingleItem => self
                .request
                .get("q")
                .ok_or(Error::MissingQueryParameter("q"))?
                .to_owned(),
        };

        let queries = Vec::from_iter(self.request.iter());

        let mut url = self.base_url.clone();
        url.path_segments_mut().unwrap().push(&endpoint);

        let search = self
            .http_client
            .get(url)
            .headers(headers)
            .query(&queries)
            .send()
            .await?
            .json::<Response>()
            .await?;

        if let Some(err) = search.message {
            return Err(Error::ApiError(err));
        }

        if let Some(response_content) = &search.response {
            if response_content.status.as_deref() == Some("error") {
                if let Some(message) = &response_content.message {
                    return Err(Error::ApiError(message.to_owned()));
                }
            }
        }

        self.request.clear();

        match search.response {
            Some(r) => Ok(r),
            None => Ok(crate::helpers::mock_response()),
        }
    }
}

impl GuardianContentClient {
    /// Constructor for the client.
    /// The constructor takes an API key which is then stored internally
    /// in the struct. The client then uses the builder pattern
    /// to add query parameters to the request. The query-building
    /// methods as well as the terminal operation take a mutable borrow
    /// of self. This allows the client not to be consumed after a request.
    /// As a result, the client should be initialised with the `mut` keyword.
    ///
    /// API keys for the Guardian's content API can be requested at
    /// <https://open-platform.theguardian.com/access/>
    ///
    /// # Example
    /// ```ignore
    /// let client = aletheia::GuardianContentClient::new("YOUR_API_KEY");
    /// ```
    pub fn new(api_key: &str) -> GuardianContentClient {
        Self {
            http_client: Client::new(),
            // Safety: it's ok to unwrap here since we are passing a valid URL string
            base_url: Url::parse(GUARDIAN_CONTENT_API_URL).unwrap(),
            api_key: String::from(api_key),
        }
    }

    /// Start building a new request.
    /// This method returns `GuardianRequestBuilder`, which allows you to
    /// build queries using the builder pattern.
    ///
    /// # Example
    /// ```ignore
    /// let client = aletheia::GuardianContentClient::new("YOUR_API_KEY");
    /// let response = client
    ///         .build_request()
    ///         .search("Elections")
    ///         .datetime_from(2020, 1, 1, 12, 0, 0, 2)
    ///         .send()
    ///         .await?;
    /// ```
    pub fn build_request(&self) -> GuardianRequestBuilder {
        GuardianRequestBuilder {
            http_client: self.http_client.clone(),
            base_url: self.base_url.clone(),
            api_key: self.api_key.clone(),
            request: HashMap::new(),
            endpoint: Endpoint::default(),
        }
    }
}

mod helpers {
    use crate::enums::{Block, IsAll};
    use crate::SearchResponse;
    use chrono::{FixedOffset, LocalResult, TimeZone};
    use std::fmt::Display;

    pub(crate) fn generate_sequence<T>(items: Vec<T>) -> String
    where
        T: Display + IsAll,
    {
        let mut sequence = String::new();
        let mut first = true;

        for item in items {
            if item.is_all() {
                return "all".to_owned();
            }

            if !first {
                sequence.push(',');
            } else {
                first = false;
            }
            sequence.push_str(&item.to_string());
        }

        sequence
    }

    pub(crate) fn generate_blocks(items: Vec<Block>) -> String {
        if items.contains(&Block::All) {
            return "all".to_owned();
        }

        let items_to_strings = items
            .into_iter()
            .map(|item| match item {
                Block::Main => item.to_string(),
                Block::Body => item.to_string(),
                Block::All => item.to_string(),
                Block::BodyLatest => String::from("body:latest"),
                Block::BodyLatestWith(n) => format!("body:latest:{n}"),
                Block::BodyOldest => String::from("body:latest"),
                Block::BodyOldestWith(n) => format!("body:oldest:{n}"),
                Block::BodyBlockId(id) => format!("body:{id}"),
                Block::BodyAroundBlockId(id) => format!("body:around:{id}"),
                Block::BodyAroundBlockIdWith(id, n) => {
                    format!("body:around:{id}:{n}")
                }
                Block::BodyKeyEvents => String::from("body:key-events"),
                Block::BodyPublishedSince(n) => format!("body:published-since:{n}"),
            })
            .collect::<Vec<String>>();

        items_to_strings.join(",")
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
        let offset: fn(i32) -> Option<FixedOffset> = if timezone >= 0 {
            FixedOffset::east_opt
        } else {
            FixedOffset::west_opt
        };

        let offset =
            offset(timezone.abs() * 3600).unwrap_or_else(|| FixedOffset::west_opt(0).unwrap());

        if let LocalResult::Single(date) = offset.with_ymd_and_hms(year, month, day, hour, min, sec)
        {
            date.to_rfc3339()
        } else {
            String::new()
        }
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
