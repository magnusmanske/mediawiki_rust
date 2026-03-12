[![Build Status](https://travis-ci.org/magnusmanske/mediawiki_rust.svg?branch=master)](https://travis-ci.org/magnusmanske/mediawiki_rust)
[![crates.io](https://img.shields.io/crates/v/mediawiki.svg)](https://crates.io/crates/mediawiki)
[![docs.rs](https://docs.rs/mediawiki/badge.svg)](https://docs.rs/mediawiki)

# A MediaWiki client library in Rust

# Introduction

This crate lets you interact with a MediaWiki API service.
To establish a connection to a MediaWiki API, use `Api` (async) or `ApiSync`.
Optionally, log in, as a bot or via OAuth2 (recommended).
You can then query the API directly via `get_query_api_json` and similar methods. You can get an edit token via `get_edit_token`.
Alternatively, use the high-level `ActionApi` and `ActionApiQuery` structs to interact with the API through structs instead of JSON. All MediaWiki and Wikibase API actions are implemented. Use `PageInfo` and `PageInfoList` to parse page info results.

# Examples

## Get all categories of "Albert Einstein" on English Wikipedia
```rust
use mediawiki::prelude::*;

let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();

let result = ActionApiQuery::categories()
    .cllimit(500)
    .titles(&["Albert Einstein"])
    .run(&api)
    .await
    .unwrap();

let categories: Vec<&str> = result["query"]["pages"]
    .as_object()
    .unwrap()
    .values()
    .flat_map(|page| {
        page["categories"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|c| c["title"].as_str())
    })
    .collect();

dbg!(&categories);
```

## Edit the Wikimedia Sandbox (as a bot)
```rust
use mediawiki::prelude::*;

let mut api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
api.login("MY BOT USER NAME", "MY BOT PASSWORD").await.unwrap();

let token = api.get_edit_token().await.unwrap();

let res = ActionApi::edit()
    .title("Wikimedia:Sandbox")
    .bot(true)
    .text("Hello from the MediaWiki Rust client!")
    .summary("test edit")
    .token(&token)
    .run(&api)
    .await
    .unwrap();

dbg!(res);
```

## Edit via OAuth
```rust
use mediawiki::prelude::*;

let json = serde_json::json!({"g_consumer_key":"YOUR_CONSUMER_KEY","g_token_key":"YOUR_TOKEN_KEY"});
let oauth = mediawiki::api::OAuthParams::new_from_json(&json);
let mut api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
api.set_oauth(Some(oauth));

// Now use ActionApi or ActionApiQuery as normal; OAuth is applied automatically.
let token = api.get_edit_token().await.unwrap();
let res = ActionApi::edit()
    .title("Wikimedia:Sandbox")
    .text("Hello from the MediaWiki Rust client!")
    .summary("test edit via OAuth")
    .token(&token)
    .run(&api)
    .await
    .unwrap();
```

## Get typed page info (no raw JSON needed)
```rust
use mediawiki::prelude::*;

let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();

let result = ActionApiQuery::info()
    .inprop(&["protection", "url", "displaytitle"])
    .titles(&["Albert Einstein", "Physics"])
    .run(&api)
    .await
    .unwrap();

let list = PageInfoList::from_result(&result);
for page in list.pages() {
    println!("{} (id {}): {}",
        page.title,
        page.pageid.unwrap_or(0),
        page.fullurl.as_deref().unwrap_or("n/a"),
    );
}
```

## Query Wikidata using SPARQL
```rust
use mediawiki::prelude::*;

// Will determine the SPARQL API URL via site info data
let api = Api::new("https://www.wikidata.org/w/api.php").await.unwrap();
let res = api.sparql_query(
    "SELECT ?q ?qLabel ?fellow_id { ?q wdt:P31 wd:Q5 ; wdt:P6594 ?fellow_id . \
     SERVICE wikibase:label { bd:serviceParam wikibase:language '[AUTO_LANGUAGE],en'. } }"
).await.unwrap();
println!("{}", serde_json::to_string_pretty(&res).unwrap());
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
