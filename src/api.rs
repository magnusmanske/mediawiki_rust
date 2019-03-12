extern crate cookie;
extern crate reqwest;

use cookie::{Cookie, CookieJar};
use serde_json::Value;
use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[derive(Debug)]
struct MWuser {
    lgusername: String,
    lguserid: u64,
    is_logged_in: bool,
}

impl MWuser {
    pub fn new() -> MWuser {
        MWuser {
            lgusername: "".into(),
            lguserid: 0,
            is_logged_in: false,
        }
    }

    pub fn set_from_login(&mut self, login: &serde_json::Value) {
        if login["result"] == "Success" {
            self.lgusername = login["lgusername"].as_str().unwrap().to_string();
            self.lguserid = login["lguserid"].as_u64().unwrap();
            self.is_logged_in = true;
        } else {
            self.is_logged_in = false;
        }
    }
}

//#[derive(Debug)]
pub struct Api {
    api_url: String,
    siteinfo: Option<serde_json::Value>,
    client: reqwest::Client,
    cookie_jar: CookieJar,
    user: MWuser,
}

impl Api {
    pub fn new(api_url: &str) -> Api {
        let ret = Api {
            api_url: api_url.to_string(),
            siteinfo: None,
            client: reqwest::Client::builder().build().unwrap(),
            cookie_jar: CookieJar::new(),
            user: MWuser::new(),
        };
        ret
    }

    pub fn site_info(&self) -> &Option<Value> {
        return &self.siteinfo;
    }

    pub fn load_site_info(&mut self) {
        self.siteinfo = self.get_site_info().ok();
    }

    pub fn json2string(v: &Value) -> String {
        v.as_str().unwrap().to_string()
    }

    fn json_merge(&self, a: &mut Value, b: Value) {
        match (a, b) {
            (a @ &mut Value::Object(_), Value::Object(b)) => {
                let a = a.as_object_mut().unwrap();
                for (k, v) in b {
                    self.json_merge(a.entry(k).or_insert(Value::Null), v);
                }
            }
            (a @ &mut Value::Array(_), Value::Array(b)) => {
                let a = a.as_array_mut().unwrap();
                for v in b {
                    a.push(v);
                }
            }
            (a, b) => *a = b,
        }
    }

    fn get_site_info(&mut self) -> Result<Value, Box<::std::error::Error>> {
        self.get_query_api_json(&hashmap!["action"=>"query","meta"=>"siteinfo","siprop"=>"general|namespaces|namespacealiases|libraries|extensions|statistics"])
    }

    pub fn get_token(&mut self, token_type: &str) -> Result<String, Box<::std::error::Error>> {
        let mut params = hashmap!["action"=>"query","meta"=>"tokens"];
        if token_type.len() != 0 {
            params.insert("type", token_type);
        }
        let mut key = token_type.to_string();
        key += &"token".to_string();
        if token_type.len() == 0 {
            key = "csrftoken".into()
        }
        let x = self.get_query_api_json_all(&params)?;
        match &x["query"]["tokens"][&key] {
            serde_json::Value::String(s) => Ok(s.to_string()),
            _ => Err(From::from("Could not get token")),
        }
    }

    pub fn get_edit_token(&mut self) -> Result<String, Box<::std::error::Error>> {
        self.get_token("csrf")
    }

    pub fn get_query_api_json_all(
        &mut self,
        params: &HashMap<&str, &str>,
    ) -> Result<Value, Box<::std::error::Error>> {
        let mut cont = HashMap::<String, String>::new();
        let mut ret = serde_json::json!({});
        loop {
            let mut params_cont = params.clone();
            for (k, v) in &cont {
                params_cont.insert(k, v);
            }
            let result = self.get_query_api_json(&params_cont)?;
            cont.clear();
            let conti = result["continue"].clone();
            self.json_merge(&mut ret, result);
            match conti {
                Value::Object(obj) => {
                    for (k, v) in obj {
                        if k != "continue" {
                            cont.insert(k.clone(), Self::json2string(&v));
                        }
                    }
                }
                _ => {
                    break;
                }
            }
        }
        ret.as_object_mut().unwrap().remove("continue");
        Ok(ret)
    }

    pub fn get_query_api_json(
        &mut self,
        params: &HashMap<&str, &str>,
    ) -> Result<Value, Box<::std::error::Error>> {
        let mut params = params.clone();
        params.insert("format", "json");
        let t = self.query_api_raw(&params, "GET")?;
        let v: Value = serde_json::from_str(&t)?;
        Ok(v)
    }

    pub fn post_query_api_json(
        &mut self,
        params: &HashMap<&str, &str>,
    ) -> Result<Value, Box<::std::error::Error>> {
        let mut params = params.clone();
        params.insert("format", "json");
        let t = self.query_api_raw(&params, "POST")?;
        let v: Value = serde_json::from_str(&t)?;
        Ok(v)
    }

    pub fn set_cookies_from_response(&mut self, resp: &reqwest::Response) {
        let cookie_strings = resp
            .headers()
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .map(|v| v.to_str().unwrap().to_string())
            .collect::<Vec<String>>();
        for cs in cookie_strings {
            let cookie = Cookie::parse(cs.clone()).unwrap();
            self.cookie_jar.add(cookie);
        }
    }

    pub fn cookies_to_string(&self) -> String {
        self.cookie_jar
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("; ")
    }

    pub fn query_api_raw(
        &mut self,
        params: &HashMap<&str, &str>,
        method: &str,
    ) -> Result<String, Box<::std::error::Error>> {
        let mut resp;
        if method == "GET" {
            resp = self
                .client
                .get(self.api_url.as_str())
                .header(reqwest::header::COOKIE, self.cookies_to_string())
                .query(&params)
                .send()?;
            self.set_cookies_from_response(&resp);
        } else if method == "POST" {
            resp = self
                .client
                .post(self.api_url.as_str())
                .header(reqwest::header::COOKIE, self.cookies_to_string())
                .form(&params)
                .send()?;
            self.set_cookies_from_response(&resp);
        } else {
            panic!("Unsupported method");
        }

        let t = resp.text()?;
        Ok(t)
    }

    pub fn login(
        &mut self,
        lgname: &str,
        lgpassword: &str,
    ) -> Result<(), Box<::std::error::Error>> {
        let lgtoken = self.get_token("login")?;
        let params = hashmap!("action"=>"login","lgname"=>&lgname,"lgpassword"=>&lgpassword,"lgtoken"=>&lgtoken);
        let res = self.post_query_api_json(&params)?;
        if res["login"]["result"] == "Success" {
            self.user.set_from_login(&res["login"]);
            Ok(())
        } else {
            panic!("Login failed") // TODO proper error return
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Api;

    #[test]
    fn site_info() {
        let mut api = Api::new("https://www.wikidata.org/w/api.php");
        api.load_site_info();
        let site_info = api.site_info();
        match site_info {
            Some(info) => assert_eq!(
                info["query"]["general"]["sitename"].as_str().unwrap(),
                "Wikidata"
            ),
            _ => panic!("Oh no"),
        }
    }
}
