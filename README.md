# Aletheia

> The literal meaning of the word ἀλήθεια is "the state of not being hidden; the state of being evident."

Aletheia is a client library for the Guardian's content API written in Rust.


## Example

Let's say you were interested in finding the ten most recent theatre play reviews with a rating of 5 stars.

```rust
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
                        "\"{}\" by {} ({})",
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
"Hannah Gadsby – Body of Work: a joyful guide to blasting Netflix and messing with Christian bakers" by Steve Dow (https://www.theguardian.com/p/kv8vx)
"The Play What I Wrote review – Tom Hiddleston has a laugh in farce masterclass" by Mark Lawson (https://www.theguardian.com/p/jq44j)
"West Side Story review – Spielberg’s triumphantly hyperreal remake" by Peter Bradshaw (https://www.theguardian.com/p/jnqdx)
"Death of England: Face to Face review – state of the nation drama is a fast, furious triumph" by Lucy Mangan (https://www.theguardian.com/p/jypxc)
"Licorice Pizza review – Paul Thomas Anderson’s funniest and most relaxed film yet" by Peter Bradshaw (https://www.theguardian.com/p/jtm7m)
"The week in classical: Life, Letters & Friendship; Bluebeard’s Castle – review" by Stephen Pritchard (https://www.theguardian.com/p/jhazq)
"Bluebeard’s Castle review – Bartók’s opera wields devastating power in contrasting performances" by Tim Ashley (https://www.theguardian.com/p/jgb5a)
"Brief Encounter review – sparkling revival of Emma Rice’s forbidden romance" by Anya Ryan (https://www.theguardian.com/p/ja66d)
"Liz Kingsman: One-Woman Show review – wicked, whip-smart skewering of Fleabag and co" by Brian Logan (https://www.theguardian.com/p/j8gnz)
"Belfast review – Kenneth Branagh’s euphoric eulogy to his home city" by Peter Bradshaw (https://www.theguardian.com/p/j62tm)

```