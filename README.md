# Aletheia

> Aletheia is truth or disclosure (...) The literal meaning of the word ἀλήθεια is "the state of not being hidden; the state of being evident."

Aletheia is an HTTP client library for [the Guardian](https://www.theguardian.com)'s [content API](https://open-platform.theguardian.com) written in Rust.

## How to use it
Aletheia requires Tokio as a dependency to execute asynchronous code.\
Simply add `aletheia` and `tokio` to the list of dependencies in your `Cargo.toml` file.

```toml
[dependencies]
aletheia = "0.1.4"
tokio = { version = "1", features = ["full"] }
```

You also need an API key to be able to make requests. 
Keys can be requested [here](https://open-platform.theguardian.com/access/). 

## Example

Let's say you were interested in finding five film, play or album reviews with a rating of 5 stars 
containing the word "politics" published from January to December 2022.
The code would look something like the example below, and would consist of three steps:

1) Constructing the HTTP client
2) Building the query
3) Parsing the response [*](#debug)
```rust
use aletheia::enums::*;
use aletheia::{GuardianContentClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // The client is constructed by passing your API key
    // as the only parameter
    let mut client = GuardianContentClient::new("your-api-key");

    // Query parameters are built incrementally
    let response = client
        .search("politics")
        .date_from(2022, 1, 1)
        .date_to(2022, 12, 31)
        .star_rating(5)
        .page_size(5)
        .show_fields(vec![Field::Byline])
        .order_by(OrderBy::Newest)
        .send()
        .await?;

    // Parsing the response.
    // The response objects are deserialized, for the most part,
    // into Option values that need to be handled safely with
    // `let else` or `if let`.
    if let Some(results) = response.results {
        for result in results {
            let Some(pub_date) = result.web_publication_date else { continue };
            let Some(fields) = result.fields else { continue };
            let Some(byline) = fields.byline else { continue };

            println!(
                "[{}] {} ({})\n{}\n",
                pub_date.format("%Y-%m-%d"),
                result.web_title.trim(),
                byline,
                result.web_url,
            )
        }
    }

    Ok(())
}
```

The above will return the following results.
```
[2022-12-15] Children of the Taliban review – this beautiful documentary is an absolute must-watch (Rebecca Nicholson)
https://www.theguardian.com/tv-and-radio/2022/dec/15/children-of-the-taliban-review-this-beautiful-documentary-is-an-absolute-must-watch

[2022-10-25] The White Lotus season two review – this immaculate show’s writing is utterly unrivalled (Lucy Mangan)
https://www.theguardian.com/tv-and-radio/2022/oct/25/the-white-lotus-season-two-review-this-immaculate-seriess-writing-is-utterly-unrivalled

[2022-10-09] The Doctor review – a repeat prescription for acute intellectual stimulation (Arifa Akbar)
https://www.theguardian.com/stage/2022/oct/10/the-doctor-review-duke-of-yorks-theatre-robert-icke-juliet-stevenson

[2022-09-27] Make Me Prime Minister review – absolute, exquisite agony (Lucy Mangan)
https://www.theguardian.com/tv-and-radio/2022/sep/27/make-me-prime-minister-review-absolute-exquisite-agony

[2022-09-02] Bones and All review – cannibal romance is a heartbreaking banquet of brilliance (Peter Bradshaw)
https://www.theguardian.com/film/2022/sep/02/bones-and-all-review-luca-guadagnino-timothee-chalamet-venice-film-festival
```

#### Debug
[*] You can pretty-print the whole output response with the format specifier `#?`:
```rust
println!("{response:#?}");
```
