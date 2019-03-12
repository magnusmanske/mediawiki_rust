extern crate config;
extern crate mediawiki;
extern crate reqwest;

use mediawiki::hashmap;

use config::*;

fn main() {
    let mut settings = Config::default();
    // File::with_name(..) is shorthand for File::from(Path::new(..))
    settings.merge(File::with_name("test.ini")).unwrap();
    let lgname = settings.get_str("user.user").unwrap();
    let lgpassword = settings.get_str("user.pass").unwrap();

    let mut api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php");
    api.login(&lgname, &lgpassword).unwrap();

    let token = api.get_token("edit").unwrap();
    dbg!(&token);
    let mut params = hashmap!("action"=>"wbeditentity","id"=>"Q4115189","lgtoken"=>&token);
    let data = r#"{"claims":[{"mainsnak":{"snaktype":"value","property":"P56","datavalue":{"value":"ExampleString","type":"string"}},"type":"statement","rank":"normal"}]}"# ;
    params.insert("data", data);
    dbg!(&params);
    api.post_query_api_json(&params).unwrap();
}
