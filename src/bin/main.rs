extern crate mediawiki;

use std::collections::HashMap;

fn main() {
    let api = mediawiki::api::Api::new("https://en.wikipedia.org/w/api.php");

    let token = api.get_token("login").unwrap();

    //    let token = x["query"]["tokens"]["logintoken"].clone();
    dbg!(token);

    /*
        let mut params = HashMap::new();
        params.insert("action", "query");
        params.insert("prop", "categories");
        params.insert("titles", "Albert Einstein");
        params.insert("cllimit", "500");
        let x = api.get_query_api_json_all(&params).unwrap();

        println!("{}", x);
    */
    /*
        api.load_site_info();
        let si = api.site_info();
        println!("{:#?}", si);
    */
}
