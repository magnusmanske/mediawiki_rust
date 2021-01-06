extern crate config;

use config::*;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

/*
extern crate mediawiki;
extern crate wikibase;
use wikibase::entity_diff::*;

use wikibase::*;

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
    api.login(lgname, lgpassword).unwrap();

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
    let mut ec = wikibase::entity_container::EntityContainer::new();
    ec.load_entities(&api, &qs).unwrap();
}

fn _wikidata_item_tester() {
    let mut settings = Config::default();
    // File::with_name(..) is shorthand for File::from(Path::new(..))
    settings.merge(File::with_name("test.ini")).unwrap();
    let lgname = settings.get_str("user.user").unwrap();
    let lgpassword = settings.get_str("user.pass").unwrap();

    // Create API and log in
    let mut api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php").unwrap();
    api.login(lgname, lgpassword).unwrap();

    // Load existing item
    let q = "Q4115189"; // Sandbox item
    let mut ec = wikibase::entity_container::EntityContainer::new();
    let orig_i = ec.load_entity(&api, q).unwrap().clone();
    let mut i = orig_i.clone();

    // Alter item
    i.add_claim(Statement::new(
        "statement",
        StatementRank::Normal,
        Snak::new(
            "wikibase-item",
            "P31",
            SnakType::Value,
            Some(DataValue::new(
                DataValueType::EntityId,
                wikibase::Value::Entity(EntityValue::new(EntityType::Item, "Q12345")),
            )),
        ),
        vec![],
        vec![],
    ));

    // Compute diff between old and new item
    let mut diff_params = EntityDiffParams::none();
    diff_params.claims.add = vec!["P31".to_string()];
    let diff = EntityDiff::new(&orig_i, &i, &diff_params);
    println!("{}\n", diff.as_str().unwrap());

    // Apply diff
    let new_json =
        EntityDiff::apply_diff(&mut api, &diff, EditTarget::Entity(q.to_string())).unwrap();
    let entity_id = EntityDiff::get_entity_id(&new_json).unwrap();
    println!("=> {}", &entity_id);

    //println!("{}", ::serde_json::to_string_pretty(&new_json).unwrap());
}

fn main() {
    //_einstein_categories();
    //_wikidata_edit();
    //_wikidata_sparql();
    _wikidata_item_tester();
}*/

async fn _edit_sandbox_item(api: &mut mediawiki::api::Api) -> Result<Value, Box<dyn Error>> {
    let q = "Q13406268"; // Second sandbox item
    let token = api.get_edit_token().await.unwrap();
    let params: HashMap<String, String> = vec![
        ("action".to_string(), "wbcreateclaim".to_string()),
        ("entity".to_string(), q.to_string()),
        ("property".to_string(), "P31".to_string()),
        ("snaktype".to_string(), "value".to_string()),
        (
            "value".to_string(),
            "{\"entity-type\":\"item\",\"id\":\"Q12345\"}".to_string(),
        ),
        ("token".to_string(), token.to_string()),
    ]
    .into_iter()
    .collect();

    api.post_query_api_json(&params).await
}

async fn _login_api_from_config(api: &mut mediawiki::api::Api) {
    let mut settings = Config::default();
    // File::with_name(..) is shorthand for File::from(Path::new(..))
    settings.merge(config::File::with_name("test.ini")).unwrap();
    let lgname = settings.get_str("user.user").unwrap();
    let lgpassword = settings.get_str("user.pass").unwrap();
    api.login(lgname, lgpassword).await.unwrap();
}

async fn _oauth_edit(api: &mut mediawiki::api::Api) {
    let sandbox_item = "Q13406268";
    let file = File::open("oauth_test.json").expect("File oauth_test.json not found");
    let j =
        serde_json::from_reader(file).expect("Reading/parsing JSON from oauth_test.json failed");
    let oauth_params = mediawiki::api::OAuthParams::new_from_json(&j);
    api.set_oauth(Some(oauth_params));
    //let _x = api.oauth().clone();

    let mut params: HashMap<String, String> = vec![
        ("action", "wbeditentity"),
        ("id", sandbox_item),
        (
            "data",
            "{\"labels\":[{\"language\":\"no\",\"value\":\"Baz\",\"add\":\"\"}]}",
        ),
        ("summary", "testing"),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect();

    params.insert(
        "token".to_string(),
        api.get_edit_token()
            .await
            .expect("Could not get edit token"),
    );

    match api.post_query_api_json_mut(&params).await {
        Ok(_) => println!("Edited https://www.wikidata.org/wiki/{}", sandbox_item),
        Err(e) => panic!("{:?}", &e),
    }
}

#[tokio::main]
async fn main() {
    /*
        if false {
            let mut settings = Config::default();
            // File::with_name(..) is shorthand for File::from(Path::new(..))
            settings.merge(File::with_name("test.ini")).unwrap();
            let lgname = settings.get_str("user.user").unwrap();
            let lgpassword = settings.get_str("user.pass").unwrap();

            // Create API and log in
            let mut api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php").unwrap();
            api.set_user_agent("Rust mediawiki crate test script");
            api.login(lgname, lgpassword).unwrap();

            let q = "Q4115189"; // Sandbox item
            let token = api.get_edit_token().unwrap();
            let params: HashMap<String, String> = vec![
                ("action".to_string(), "wbcreateclaim".to_string()),
                ("entity".to_string(), q.to_string()),
                ("property".to_string(), "P31".to_string()),
                ("snaktype".to_string(), "value".to_string()),
                (
                    "value".to_string(),
                    "{\"entity-type\":\"item\",\"id\":\"Q12345\"}".to_string(),
                ),
                ("token".to_string(), token.to_string()),
            ]
            .into_iter()
            .collect();

            let res = api.post_query_api_json(&params).unwrap();
            dbg!(&res);
        }
    */

    let api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php")
        .await
        .unwrap();
    let x = api.get_namespace_info(0);
    println!("{:?}", x);
    let x = api.get_local_namespace_name(0);
    println!("{:?}", x);
    let x = api.get_canonical_namespace_name(0);
    println!("{:?}", x);

    //login_api_from_config(&mut api);
    //println!("{}", api.user_agent_full());
    //_oauth_edit(&mut api);

    /*
    let mut user = mediawiki::user::User::new();
    user.load_user_info(&api).unwrap();
    dbg!(user.has_right("createaccount"));
    */

    /*
        let res = edit_sandbox_item(&mut api);
        println!("{:?}", res.unwrap());
    */
}
