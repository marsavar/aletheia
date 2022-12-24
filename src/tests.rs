#[cfg(test)]
mod tests {
    use crate::{
        enums::{Endpoint, Field, OrderBy, OrderDate, UseDate},
        GuardianContentClient,
    };

    fn setup() -> GuardianContentClient {
        GuardianContentClient::new("test-api-key")
    }

    #[test]
    fn test_api_key() {
        let client = setup();
        assert_eq!(client.api_key, "test-api-key");
    }

    #[test]
    fn test_base_url() {
        let client = setup();
        assert_eq!(client.base_url, "https://content.guardianapis.com");
        assert_eq!(client.endpoint, Endpoint::Content);
    }

    #[test]
    fn test_other_endpoints() {
        let mut client = setup();
        client.endpoint(Endpoint::Sections);
        assert_eq!(client.endpoint, Endpoint::Sections);
        client.endpoint(Endpoint::Editions);
        assert_eq!(client.endpoint, Endpoint::Editions);
        client.endpoint(Endpoint::Tags);
        assert_eq!(client.endpoint, Endpoint::Tags);
        client.endpoint(Endpoint::SingleItem);
        assert_eq!(client.endpoint, Endpoint::SingleItem);
    }

    #[test]
    fn test_search() {
        let mut client = setup();
        client.search("politics");
        assert_eq!(client.request.get("q").unwrap(), "politics")
    }

    #[test]
    fn test_page() {
        let mut client = setup();
        client.page(10);
        assert_eq!(client.request.get("page").unwrap(), "10")
    }

    #[test]
    fn test_page_size() {
        let mut client = setup();
        client.page_size(20);
        assert_eq!(client.request.get("page-size").unwrap(), "20")
    }

    #[test]
    fn test_order_by() {
        let mut client = setup();
        client.order_by(OrderBy::Oldest);
        assert_eq!(client.request.get("order-by").unwrap(), "oldest");
        client.order_by(OrderBy::Newest);
        assert_eq!(client.request.get("order-by").unwrap(), "newest");
        client.order_by(OrderBy::Relevance);
        assert_eq!(client.request.get("order-by").unwrap(), "relevance")
    }

    #[test]
    fn test_order_date() {
        let mut client = setup();
        client.order_date(OrderDate::Published);
        assert_eq!(client.request.get("order-date").unwrap(), "published");
        client.order_date(OrderDate::NewspaperEdition);
        assert_eq!(
            client.request.get("order-date").unwrap(),
            "newspaper-edition"
        );
        client.order_date(OrderDate::LastModified);
        assert_eq!(client.request.get("order-date").unwrap(), "last-modified")
    }

    #[test]
    fn test_show_fields() {
        let mut client = setup();
        client.show_fields(vec![Field::ShortUrl, Field::Byline, Field::StarRating]);
        assert_eq!(
            client.request.get("show-fields").unwrap(),
            "shortUrl,byline,starRating"
        );
        client.show_fields(vec![
            Field::ShortUrl,
            Field::Byline,
            Field::StarRating,
            Field::All,
        ]);
        assert_eq!(client.request.get("show-fields").unwrap(), "all");
    }

    #[test]
    fn test_show_tags() {
        let mut client = setup();
        client.show_tags(vec![
            crate::enums::Tag::Blog,
            crate::enums::Tag::Contributor,
        ]);
        assert_eq!(client.request.get("show-tags").unwrap(), "blog,contributor");
    }

    #[test]
    fn test_query_fields() {
        let mut client = setup();
        client.query_fields(vec![Field::ProductionOffice]);
        assert_eq!(
            client.request.get("query-fields").unwrap(),
            "productionOffice"
        );
    }

    #[test]
    fn test_date_from() {
        let mut client = setup();
        client.date_from(2020, 1, 1);
        assert_eq!(client.request.get("from-date").unwrap(), "2020-1-1");
    }

    #[test]
    fn test_date_to() {
        let mut client = setup();
        client.date_to(2020, 1, 1);
        assert_eq!(client.request.get("to-date").unwrap(), "2020-1-1");
    }

    #[test]
    fn test_datetime_from() {
        let mut client = setup();
        client.datetime_from(2021, 12, 31, 0, 0, 0, 5);
        assert_eq!(
            client.request.get("from-date").unwrap(),
            "2021-12-31T00:00:00+05:00"
        );
    }

    #[test]
    fn test_datetime_to() {
        let mut client = setup();
        client.datetime_to(2021, 12, 31, 0, 0, 0, -5);
        assert_eq!(
            client.request.get("to-date").unwrap(),
            "2021-12-31T00:00:00-05:00"
        );
    }

    #[test]
    fn test_datetime_to_2() {
        let mut client = setup();
        // Invalid offset
        client.datetime_to(2021, 12, 31, 0, 0, 0, 999);
        assert_eq!(
            client.request.get("to-date").unwrap(),
            "2021-12-31T00:00:00+00:00"
        );
    }

    #[test]
    fn test_datetime_to_3() {
        let mut client = setup();
        // Invalid YMD
        client.datetime_to(2021, 13, 40, 0, 0, 0, 5);
        assert_eq!(client.request.get("to-date").unwrap(), "");
    }

    #[test]
    fn test_use_date() {
        let mut client = setup();
        client.use_date(UseDate::FirstPublication);
        assert_eq!(client.request.get("use-date").unwrap(), "first-publication");
    }

    #[test]
    fn test_show_section() {
        let mut client = setup();
        client.show_section(true);
        assert_eq!(client.request.get("show-section").unwrap(), "true");
    }

    #[test]
    fn test_section() {
        let mut client = setup();
        client.section("food");
        assert_eq!(client.request.get("section").unwrap(), "food");
    }

    #[test]
    fn test_reference() {
        let mut client = setup();
        client.reference("isbn");
        assert_eq!(client.request.get("reference").unwrap(), "isbn");
    }

    #[test]
    fn test_reference_type() {
        let mut client = setup();
        client.reference_type("isbn/123456789012");
        assert_eq!(
            client.request.get("reference-type").unwrap(),
            "isbn/123456789012"
        );
    }

    #[test]
    fn test_tag() {
        let mut client = setup();
        client.tag("technology/apple");
        assert_eq!(client.request.get("tag").unwrap(), "technology/apple");
    }

    #[test]
    fn test_ids() {
        let mut client = setup();
        client.ids("world/2022/jan/01/funeral-of-desmond-tutu-takes-place-in-cape-town");
        assert_eq!(
            client.request.get("ids").unwrap(),
            "world/2022/jan/01/funeral-of-desmond-tutu-takes-place-in-cape-town"
        );
    }

    #[test]
    fn test_production_office() {
        let mut client = setup();
        client.production_office("aus");
        assert_eq!(client.request.get("production-office").unwrap(), "aus");
    }

    #[test]
    fn test_lang() {
        let mut client = setup();
        client.lang("fr");
        assert_eq!(client.request.get("lang").unwrap(), "fr");
    }

    #[test]
    fn test_star_rating() {
        let mut client = setup();
        client.star_rating(3);
        assert_eq!(client.request.get("star-rating").unwrap(), "3");
    }

    #[test]
    fn test_tag_type() {
        let mut client = setup();
        client.tag_type("tv-and-radio/us-television");
        assert_eq!(
            client.request.get("type").unwrap(),
            "tv-and-radio/us-television"
        );
    }

    #[test]
    fn test_show_blocks() {
        let mut client = setup();
        client.show_blocks(vec![crate::enums::Block::BodyAroundBlockIdWith(
            "123456789",
            10,
        )]);
        assert_eq!(
            client.request.get("show-blocks").unwrap(),
            "body:around:123456789:10"
        );
        client.show_blocks(vec![crate::enums::Block::BodyPublishedSince(123456)]);
        assert_eq!(
            client.request.get("show-blocks").unwrap(),
            "body:published-since:123456"
        );
        client.show_blocks(vec![
            crate::enums::Block::BodyPublishedSince(123456),
            crate::enums::Block::BodyKeyEvents,
        ]);
        assert_eq!(
            client.request.get("show-blocks").unwrap(),
            "body:published-since:123456,body:key-events"
        );
    }
}
