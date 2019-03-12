extern crate config;
extern crate mediawiki;
extern crate reqwest;

use config::*;

fn main() {
    let mut settings = Config::default();
    // File::with_name(..) is shorthand for File::from(Path::new(..))
    settings.merge(File::with_name("test.ini")).unwrap();
    let lgname = settings.get_str("user.user").unwrap();
    let lgpassword = settings.get_str("user.pass").unwrap();

    let mut api = mediawiki::api::Api::new("https://www.wikidata.org/w/api.php");
    api.login(&lgname, &lgpassword).unwrap();
}
