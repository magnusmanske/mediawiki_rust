extern crate config;
extern crate mediawiki;

use config::*;
use mediawiki::hashmap;

fn main() {
    let mut api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php");

    let mut settings = Config::default();
    // File::with_name(..) is shorthand for File::from(Path::new(..))
    settings.merge(File::with_name("test.ini")).unwrap();
    let lgname = settings.get_str("user.user").unwrap();
    let lgpassword = settings.get_str("user.pass").unwrap();

    api.login(&lgname, &lgpassword);

    let _res = api
        .get_query_api_json(&hashmap!["action"=>"query","meta"=>"userinfo","format"=>"json"])
        .unwrap();

    dbg!(_res);

    /*
          let token = api.get_token("").unwrap();
          let mut params = HashMap::new();
          params.insert("action", "wbsetaliases");
          params.insert("id", "Q4115189");
          params.insert("token", &token);
          params.insert("language", "en");
          params.insert("add", "test123");
          params.insert("format", "json");

          let _res = api.post_query_api_json(&params).unwrap(); // TODO check error
          dbg!(_res);
    */

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
