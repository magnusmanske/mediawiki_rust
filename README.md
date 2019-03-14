# A MediaWiki client library in Rust

## Examples
Get all categories of "Albert Einstein" on English Wikipedia:
```rust
let mut api = mediawiki::api::Api::new("https://en.wikipedia.org/w/api.php");

// Query parameters
let params: HashMap<_, _> = vec![
    ("action", "query"),
    ("prop", "categories"),
    ("titles", "Albert Einstein"),
    ("cllimit", "500"),
]
.into_iter()
.collect();

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

Edit the Wikidata Sandbox Item (as a bot):
```rust
let mut api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php");
api.login("MY BOT USER NAME", "MY BOT PASSWORD").unwrap();

let token = api.get_edit_token().unwrap();
let params: HashMap<_, _> = vec![
    ("action", "wbeditentity"),
    ("id", "Q4115189"),
    ("data",r#"{"claims":[{"mainsnak":{"snaktype":"value","property":"P1810","datavalue":{"value":"ExampleString","type":"string"}},"type":"statement","rank":"normal"}]}"#),
    ("token", &token),
]
.into_iter()
.collect();
let res = api.post_query_api_json(&params).unwrap();
```
Query Wikidata using SPARQL:
```rust
let mut api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php"); // Will determine the SPARQL API URL via site info data
let res = api.sparql_query ( "SELECT ?q ?qLabel ?fellow_id { ?q wdt:P31 wd:Q5 ; wdt:P6594 ?fellow_id . SERVICE wikibase:label { bd:serviceParam wikibase:language '[AUTO_LANGUAGE],en'. } }" ).unwrap() ;
println!("{}", ::serde_json::to_string_pretty(&res).unwrap());
```
