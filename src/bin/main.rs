extern crate config;
extern crate mediawiki;

use config::*;
use std::collections::HashMap;

fn _einstein_categories() {
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

    // Run query
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
}

fn _wikidata_edit() {
    let mut settings = Config::default();
    // File::with_name(..) is shorthand for File::from(Path::new(..))
    settings.merge(File::with_name("test.ini")).unwrap();
    let lgname = settings.get_str("user.user").unwrap();
    let lgpassword = settings.get_str("user.pass").unwrap();

    let mut api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php");
    api.login(&lgname, &lgpassword).unwrap();

    let token = api.get_edit_token().unwrap();
    let params: HashMap<_, _> = vec![
        ("action", "wbeditentity"),
        ("id", "Q4115189"),
        ("data",r#"{"claims":[{"mainsnak":{"snaktype":"value","property":"P1810","datavalue":{"value":"ExampleString","type":"string"}},"type":"statement","rank":"normal"}]}"#),
        ("token", &token),
    ]
    .into_iter()
    .collect();
    let _res = api.post_query_api_json(&params).unwrap();
    //    dbg!(res["success"].as_u64().unwrap());
}

fn _wikidata_sparql() {
    let mut api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php");
    let res = api.sparql_query ( "SELECT ?q ?qLabel ?fellow_id { ?q wdt:P31 wd:Q5 ; wdt:P6594 ?fellow_id . SERVICE wikibase:label { bd:serviceParam wikibase:language '[AUTO_LANGUAGE],en'. } }" ).unwrap() ;
    println!("{}", ::serde_json::to_string_pretty(&res).unwrap());
}

fn main() {
    _wikidata_sparql();
}
