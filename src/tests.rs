#[cfg(test)]
mod tests {
    use reqwest::Url;

    use crate::{
        enums::{Endpoint, Field, OrderBy, OrderDate, UseDate},
        error::Error,
        GuardianContentClient,
    };

    fn client_setup() -> GuardianContentClient {
        GuardianContentClient::new("test-api-key")
    }

    #[test]
    fn test_api_key() {
        let client = client_setup();
        assert_eq!(client.api_key, "test-api-key");
    }

    #[test]
    fn test_base_url() {
        let client = client_setup();
        assert_eq!(
            client.base_url,
            Url::parse("https://content.guardianapis.com").unwrap()
        );

        let client = client_setup().build_request();
        assert_eq!(client.endpoint, Endpoint::Content);
    }

    #[test]
    fn test_other_endpoints() {
        let client = client_setup().build_request().endpoint(Endpoint::Sections);
        assert_eq!(client.endpoint, Endpoint::Sections);

        let client = client_setup().build_request().endpoint(Endpoint::Editions);
        assert_eq!(client.endpoint, Endpoint::Editions);

        let client = client_setup().build_request().endpoint(Endpoint::Tags);
        assert_eq!(client.endpoint, Endpoint::Tags);

        let client = client_setup()
            .build_request()
            .endpoint(Endpoint::SingleItem);
        assert_eq!(client.endpoint, Endpoint::SingleItem);
    }

    #[test]
    fn test_search() {
        let client = client_setup().build_request().search("politics");
        assert_eq!(client.request.get("q").unwrap(), "politics")
    }

    #[test]
    fn test_page() {
        let client = client_setup().build_request().page(10);
        assert_eq!(client.request.get("page").unwrap(), "10")
    }

    #[test]
    fn test_page_size() {
        let client = client_setup().build_request().page_size(20);
        assert_eq!(client.request.get("page-size").unwrap(), "20")
    }

    #[test]
    fn test_order_by() {
        let client = client_setup().build_request().order_by(OrderBy::Oldest);
        assert_eq!(client.request.get("order-by").unwrap(), "oldest");

        let client = client_setup().build_request().order_by(OrderBy::Newest);
        assert_eq!(client.request.get("order-by").unwrap(), "newest");

        let client = client_setup().build_request().order_by(OrderBy::Relevance);
        assert_eq!(client.request.get("order-by").unwrap(), "relevance")
    }

    #[test]
    fn test_order_date() {
        let client = client_setup()
            .build_request()
            .order_date(OrderDate::Published);
        assert_eq!(client.request.get("order-date").unwrap(), "published");

        let client = client_setup()
            .build_request()
            .order_date(OrderDate::NewspaperEdition);
        assert_eq!(
            client.request.get("order-date").unwrap(),
            "newspaper-edition"
        );

        let client = client_setup()
            .build_request()
            .order_date(OrderDate::LastModified);
        assert_eq!(client.request.get("order-date").unwrap(), "last-modified")
    }

    #[test]
    fn test_show_fields() {
        let client = client_setup().build_request().show_fields(vec![
            Field::ShortUrl,
            Field::Byline,
            Field::StarRating,
        ]);
        assert_eq!(
            client.request.get("show-fields").unwrap(),
            "shortUrl,byline,starRating"
        );

        let client = client_setup().build_request().show_fields(vec![
            Field::ShortUrl,
            Field::Byline,
            Field::StarRating,
            Field::All,
        ]);
        assert_eq!(client.request.get("show-fields").unwrap(), "all");
    }

    #[test]
    fn test_show_tags() {
        let client = client_setup().build_request().show_tags(vec![
            crate::enums::Tag::Blog,
            crate::enums::Tag::Contributor,
        ]);
        assert_eq!(client.request.get("show-tags").unwrap(), "blog,contributor");
    }

    #[test]
    fn test_query_fields() {
        let client = client_setup()
            .build_request()
            .query_fields(vec![Field::ProductionOffice]);
        assert_eq!(
            client.request.get("query-fields").unwrap(),
            "productionOffice"
        );
    }

    #[test]
    fn test_date_from() {
        let client = client_setup().build_request().date_from(2020, 1, 1);
        assert_eq!(client.request.get("from-date").unwrap(), "2020-1-1");
    }

    #[test]
    fn test_date_to() {
        let client = client_setup().build_request().date_to(2020, 1, 1);
        assert_eq!(client.request.get("to-date").unwrap(), "2020-1-1");
    }

    #[test]
    fn test_datetime_from() {
        let client = client_setup()
            .build_request()
            .datetime_from(2021, 12, 31, 0, 0, 0, 5);
        assert_eq!(
            client.request.get("from-date").unwrap(),
            "2021-12-31T00:00:00+05:00"
        );
    }

    #[test]
    fn test_datetime_to() {
        let client = client_setup()
            .build_request()
            .datetime_to(2021, 12, 31, 0, 0, 0, -5);
        assert_eq!(
            client.request.get("to-date").unwrap(),
            "2021-12-31T00:00:00-05:00"
        );
    }

    #[test]
    fn test_datetime_from_wrong_offset() {
        let client = client_setup()
            .build_request()
            // Invalid offset
            .datetime_from(2021, 12, 31, 0, 0, 0, 1024);
        assert_eq!(
            client.request.get("from-date").unwrap(),
            "2021-12-31T00:00:00+00:00"
        );
    }

    #[test]
    fn test_datetime_from_wrong_ymd_hms() {
        let client = client_setup()
            .build_request()
            // Invalid YMD
            .datetime_from(2021, 13, 40, 0, 999, 0, 5);
        assert_eq!(client.request.get("from-date"), None);
    }

    #[test]
    fn test_datetime_to_wrong_offset() {
        let client = client_setup()
            .build_request()
            // Invalid offset
            .datetime_to(2021, 12, 31, 0, 0, 0, 999);
        assert_eq!(
            client.request.get("to-date").unwrap(),
            "2021-12-31T00:00:00+00:00"
        );
    }

    #[test]
    fn test_datetime_to_wrong_ymd_hms() {
        let client = client_setup()
            .build_request()
            // Invalid YMD
            .datetime_to(2021, 13, 40, 0, 999, 0, 5);
        assert_eq!(client.request.get("to-date"), None);
    }

    #[test]
    fn test_use_date() {
        let client = client_setup()
            .build_request()
            .use_date(UseDate::FirstPublication);
        assert_eq!(client.request.get("use-date").unwrap(), "first-publication");
    }

    #[test]
    fn test_show_section() {
        let client = client_setup().build_request().show_section(true);
        assert_eq!(client.request.get("show-section").unwrap(), "true");
    }

    #[test]
    fn test_section() {
        let client = client_setup().build_request().section("food");
        assert_eq!(client.request.get("section").unwrap(), "food");
    }

    #[test]
    fn test_reference() {
        let client = client_setup().build_request().reference("isbn");
        assert_eq!(client.request.get("reference").unwrap(), "isbn");
    }

    #[test]
    fn test_reference_type() {
        let client = client_setup()
            .build_request()
            .reference_type("isbn/123456789012");
        assert_eq!(
            client.request.get("reference-type").unwrap(),
            "isbn/123456789012"
        );
    }

    #[test]
    fn test_tag() {
        let client = client_setup().build_request().tag("technology/apple");
        assert_eq!(client.request.get("tag").unwrap(), "technology/apple");
    }

    #[test]
    fn test_ids() {
        let client = client_setup()
            .build_request()
            .ids("world/2022/jan/01/funeral-of-desmond-tutu-takes-place-in-cape-town");
        assert_eq!(
            client.request.get("ids").unwrap(),
            "world/2022/jan/01/funeral-of-desmond-tutu-takes-place-in-cape-town"
        );
    }

    #[test]
    fn test_production_office() {
        let client = client_setup().build_request().production_office("aus");
        assert_eq!(client.request.get("production-office").unwrap(), "aus");
    }

    #[test]
    fn test_lang() {
        let client = client_setup().build_request().lang("fr");
        assert_eq!(client.request.get("lang").unwrap(), "fr");
    }

    #[test]
    fn test_star_rating() {
        let client = client_setup().build_request().star_rating(3);
        assert_eq!(client.request.get("star-rating").unwrap(), "3");
    }

    #[test]
    fn test_tag_type() {
        let client = client_setup()
            .build_request()
            .tag_type("tv-and-radio/us-television");
        assert_eq!(
            client.request.get("type").unwrap(),
            "tv-and-radio/us-television"
        );
    }

    #[test]
    fn test_show_blocks() {
        let client = client_setup().build_request().show_blocks(vec![
            crate::enums::Block::BodyAroundBlockIdWith("123456789", 10),
        ]);
        assert_eq!(
            client.request.get("show-blocks").unwrap(),
            "body:around:123456789:10"
        );

        let client = client_setup()
            .build_request()
            .show_blocks(vec![crate::enums::Block::BodyPublishedSince(123456)]);
        assert_eq!(
            client.request.get("show-blocks").unwrap(),
            "body:published-since:123456"
        );

        let client = client_setup().build_request().show_blocks(vec![
            crate::enums::Block::BodyPublishedSince(123456),
            crate::enums::Block::BodyKeyEvents,
        ]);
        assert_eq!(
            client.request.get("show-blocks").unwrap(),
            "body:published-since:123456,body:key-events"
        );
    }

    #[tokio::test]
    async fn test_error_missing_parameter() {
        let result = client_setup()
            .build_request()
            .endpoint(Endpoint::SingleItem)
            .send()
            .await;

        assert!(result.is_err());

        let err = result.err().unwrap();
        assert!(matches!(err, Error::MissingQueryParameter("q")));
    }
}
