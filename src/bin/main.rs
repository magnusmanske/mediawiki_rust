use config::*;
use mediawiki::page::Page;
use mediawiki::Title;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;

use mediawiki::Api;
use mediawiki::MediaWikiError;

async fn edit_sandbox_item(api: &mut Api) -> Result<Value, MediaWikiError> {
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

async fn login_api_from_config(api: &mut Api) {
    let settings = Config::builder()
        .add_source(config::File::with_name("test.ini"))
        .build()
        .expect("Could not build config");
    let lgname = settings.get_string("user.user").unwrap();
    let lgpassword = settings.get_string("user.pass").unwrap();
    api.login(lgname, lgpassword).await.unwrap();
}

async fn oauth_edit(api: &mut Api) {
    let sandbox_item = "Q13406268";
    let file = File::open("oauth_test.json").expect("File oauth_test.json not found");
    let j =
        serde_json::from_reader(file).expect("Reading/parsing JSON from oauth_test.json failed");
    let oauth_params = mediawiki::api::OAuthParams::new_from_json(&j);
    api.set_oauth(Some(oauth_params));

    let mut params: HashMap<String, String> = [
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

fn check_namespaces(api: &Api) {
    let x = api.get_canonical_namespace_name(6).unwrap();
    println!("{x}"); // "File"
    let x = api.get_local_namespace_name(6).unwrap();
    println!("{x}"); // "Datei"
}

async fn check_page(api: &Api) {
    let title = Title::new_from_full("Jimmy Wales", api);
    let page = Page::new(title.clone());
    let wikitext = page.text(api).await.unwrap();
    println!(
        "{title} has something to do with Wikipedia: {}",
        wikitext.contains("Wikipedia")
    ); // "Jimmy Wales has something to do with Wikipedia: true"
}

#[tokio::main]
async fn main() {
    // German Wikipedia
    let api = Api::new("https://de.wikipedia.org/w/api.php")
        .await
        .unwrap();

    check_namespaces(&api);
    check_page(&api).await;

    // Wikidata
    // Deactivated, because editing...
    if false {
        let mut api = Api::new("https://www.wikipedia.org/w/api.php")
            .await
            .unwrap();

        login_api_from_config(&mut api).await;
        oauth_edit(&mut api).await;
        edit_sandbox_item(&mut api).await.unwrap();
    }
}
