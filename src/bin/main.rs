extern crate config;
extern crate mediawiki;

use config::*;
use mediawiki::entity_diff::*;
use std::collections::HashMap;

fn _einstein_categories() {
    let api = mediawiki::api::Api::new("https://en.wikipedia.org/w/api.php").unwrap();

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

    let mut api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php").unwrap();
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
    let api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php").unwrap();
    let res = api.sparql_query ( "SELECT ?q ?qLabel ?fellow_id { ?q wdt:P31 wd:Q5 ; wdt:P6594 ?fellow_id . SERVICE wikibase:label { bd:serviceParam wikibase:language '[AUTO_LANGUAGE],en'. } }" ).unwrap() ;
    //println!("{}", ::serde_json::to_string_pretty(&res).unwrap());

    let mut qs = vec![];
    for b in res["results"]["bindings"].as_array().unwrap() {
        match b["q"]["value"].as_str() {
            Some(entity_url) => {
                qs.push(api.extract_entity_from_uri(entity_url).unwrap());
            }
            None => {}
        }
    }
    //println!("{}: {:?}", qs.len(), qs);
    let mut ec = mediawiki::entity_container::EntityContainer::new();
    ec.load_entities(&api, &qs).unwrap();
}

fn _wikidata_item_tester() {
    let api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php").unwrap();
    let mut ec = mediawiki::entity_container::EntityContainer::new();
    let i = ec.load_entity(&api, &"Q42".to_string()).unwrap();
    //let p31 = i.claims_with_property(&"P31".to_string());
    let new_i = wikibase::Entity::new_empty();
    let mut params = EntityDiffParams::none();
    params.labels.remove = vec!["en".to_string()];
    let diff = EntityDiff::new(&i, &new_i, &params);
    println!("{}", serde_json::to_string_pretty(diff.actions()).unwrap());
}

fn main() {
    //_einstein_categories();
    //_wikidata_edit();
    //_wikidata_sparql();
    _wikidata_item_tester();
}
