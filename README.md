# Aletheia

> Aletheia is truth or disclosure in philosophy. The literal meaning of the word ἀλήθεια is "the state of not being hidden; the state of being evident."

Aletheia is a client library for [the Guardian](https://www.theguardian.com)'s content API written in Rust.


## How to use
Simply add `aletheia` to the list of dependencies in your `Cargo.toml` file

```
[dependencies]
aletheia = "0.1.0"
```

## Example

Let's say you were interested in finding the five most recent theatre play reviews with a rating of 5 stars.
The code would look something like the example below, and would consist of three steps:

1) Constructing the HTTP client
2) Building the query
3) Parsing the response
```rust
use aletheia::enums::*;
use aletheia::GuardianContentClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    // The client is constructed by passing your API key
    // as the only parameter
    let mut client = GuardianContentClient::new("your_api_key")?;
    
    // Query parameters are built incrementally
    let response = client
        .search("theatre")
        .star_rating(5)
        .page_size(10)
        .show_fields(vec![Field::Byline, Field::ShortUrl])
        .order_by(OrderBy::Newest)
        .send()
        .await?;
    
    // Parsing the response.
    // The response objects are deserialized, for the most part,
    // into Option values that require the use of
    // `if let` or `match` to handle safely.
    if let Some(results) = response.results {
        results.into_iter().for_each(|result| {
            if let Some(fields) = result.fields {
                match (fields.byline, fields.short_url) {
                    (Some(byline), Some(short_url)) => println!(
                        "\"{}\" \nby {} ({})\n",
                        result.web_title.trim(),
                        byline,
                        short_url
                    ),
                    _ => {}
                }
            }
        });
    }
    
    Ok(())
}
```

The above will return the following results.
```
"Hannah Gadsby – Body of Work: a joyful guide to blasting Netflix and messing with Christian bakers" 
by Steve Dow (https://www.theguardian.com/p/kv8vx)

"The Play What I Wrote review – Tom Hiddleston has a laugh in farce masterclass" 
by Mark Lawson (https://www.theguardian.com/p/jq44j)

"West Side Story review – Spielberg’s triumphantly hyperreal remake" 
by Peter Bradshaw (https://www.theguardian.com/p/jnqdx)

"Death of England: Face to Face review – state of the nation drama is a fast, furious triumph" 
by Lucy Mangan (https://www.theguardian.com/p/jypxc)

"Licorice Pizza review – Paul Thomas Anderson’s funniest and most relaxed film yet" 
by Peter Bradshaw (https://www.theguardian.com/p/jtm7m)
```