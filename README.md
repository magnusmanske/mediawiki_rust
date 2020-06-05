[![Build Status](https://travis-ci.org/magnusmanske/mediawiki_rust.svg?branch=master)](https://travis-ci.org/magnusmanske/mediawiki_rust)
[![crates.io](https://img.shields.io/crates/v/mediawiki.svg)](https://crates.io/crates/mediawiki)
[![docs.rs](https://docs.rs/mediawiki/badge.svg)](https://docs.rs/mediawiki)

# A MediaWiki client library in Rust

# Examples

## Get all categories of "Albert Einstein" on English Wikipedia
```rust
let mut api = mediawiki::api::Api::new("https://en.wikipedia.org/w/api.php").unwrap();

// Query parameters
let params = api.params_into(&[
    ("action", "query"),
    ("prop", "categories"),
    ("titles", "Albert Einstein"),
    ("cllimit", "500"),
]);

// Run query; this will automatically continue if more results are available, and merge all results into one
let res = api.get_query_api_json_all(&params).unwrap();

// Parse result
let categories: Vec<&str> = res["query"]["pages"]
    .as_object()
    .unwrap()
    .iter()
    .flat_map(|(_page_id, page)| {
        page["categories"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["title"].as_str().unwrap())
    })
    .collect();

dbg!(&categories);
```

## Edit the Wikidata Sandbox Item (as a bot)
```rust
let mut api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php").unwrap();
api.login("MY BOT USER NAME", "MY BOT PASSWORD").unwrap();

let token = api.get_edit_token().unwrap();

let params = api.params_into(&[
    ("action", "wbeditentity"),
    ("id", "Q4115189"),
    ("data", r#"{"claims":[{"mainsnak":{"snaktype":"value","property":"P1810","datavalue":{"value":"ExampleString","type":"string"}},"type":"statement","rank":"normal"}]}"#),
    ("token", &token),
]);

let res = api.post_query_api_json(&params).unwrap();
dbg!(res);
```

## Edit via OAuth
```rust
let json = json!({"g_consumer_key":"YOUR_CONSUMER_KEY","g_token_key":"YOUR_TOKEN_KEY"});
let oauth = mediawiki::api::OAuthParams::new_from_json(&json);
let mut api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php").unwrap();
api.set_oauth(Some(oauth));
```

## Query Wikidata using SPARQL
```rust
let api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php").unwrap(); // Will determine the SPARQL API URL via site info data
let res = api.sparql_query ( "SELECT ?q ?qLabel ?fellow_id { ?q wdt:P31 wd:Q5 ; wdt:P6594 ?fellow_id . SERVICE wikibase:label { bd:serviceParam wikibase:language '[AUTO_LANGUAGE],en'. } }" ).unwrap() ;
println!("{}", ::serde_json::to_string_pretty(&res).unwrap());
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
