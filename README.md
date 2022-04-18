# Aletheia

> Aletheia is truth or disclosure (...) The literal meaning of the word ἀλήθεια is "the state of not being hidden; the state of being evident."

Aletheia is an HTTP client library for [the Guardian](https://www.theguardian.com)'s [content API](https://open-platform.theguardian.com) written in Rust.

## How to use it
Aletheia requires Tokio as a dependency to execute asynchronous code.\
Simply add `aletheia` and `tokio` to the list of dependencies in your `Cargo.toml` file.

```toml
[dependencies]
aletheia = "0.1.3"
tokio = { version = "1", features = ["full"] }
```

You also need an API key to be able to make requests. 
Keys can be requested [here](https://open-platform.theguardian.com/access/). 

## Example

Let's say you were interested in finding five film, play or album reviews with a rating of 5 stars 
containing the word "politics" published from October to December 2021.
The code would look something like the example below, and would consist of three steps:

1) Constructing the HTTP client
2) Building the query
3) Parsing the response [*](#debug)
```rust
use aletheia::enums::*;
use aletheia::GuardianContentClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    // The client is constructed by passing your API key
    // as the only parameter
    let mut client = GuardianContentClient::new("your_api_key");
    
    // Query parameters are built incrementally
    let response = client
        .search("politics")
        .date_from(2021, 10, 1)
        .date_to(2021, 12, 31)
        .star_rating(5)
        .page_size(5)
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
                if let (Some(byline), Some(short_url)) = (fields.byline, fields.short_url) {
                    println!(
                        "\"{}\" \nby {} ({})\n",
                        result.web_title.trim(),
                        byline,
                        short_url
                    )
                }
            }
        });
    }
    
    Ok(())
}
```

The above will return the following results.
```
"Licorice Pizza review – Paul Thomas Anderson’s funniest and most relaxed film yet" 
by Peter Bradshaw (https://www.theguardian.com/p/jtm7m)

"Rina Sawayama review – superstar status cemented by pop’s politician" 
by Fergal Kinney (https://www.theguardian.com/p/jh9tx)

"Burning review – the searing black summer documentary that Australia deserves" 
by Luke Buckmaster (https://www.theguardian.com/p/jeqg5)

"Harry Potter and the Philosopher’s Stone review – 20 years on, it’s a nostalgic spectacular" 
by Peter Bradshaw (https://www.theguardian.com/p/japa7)

"‘Some of art’s most luxurious orgies’ – Poussin and the Dance review" 
by Jonathan Jones (https://www.theguardian.com/p/j5kkp)
```

#### Debug
[*] You can pretty-print the whole output response with the format specifier `#?`:
```rust
println!("{:#?}", response);
```
