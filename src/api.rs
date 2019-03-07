extern crate reqwest;

//use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::StatusCode;
use reqwest::Url;
use serde_json::Value;
use std::collections::HashMap;
use urlencoding::encode;

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

//#[derive(Debug)]
pub struct Api {
    api_url: String,
    siteinfo: Option<Value>,
    //    client: reqwest::Client,
    session: user_agent::Session<reqwest::Client>,
}

impl Api {
    pub fn new(api_url: &str) -> Api {
        let ret = Api {
            api_url: api_url.to_string(),
            siteinfo: None,
            session: user_agent::Session::new(reqwest::Client::new()),
        };
        ret
    }

    pub fn site_info(&self) -> &Option<Value> {
        return &self.siteinfo;
    }

    pub fn load_site_info(&mut self) {
        self.siteinfo = self.get_site_info().ok();
    }

    fn json2string(v: &Value) -> String {
        serde_json::from_value(v.clone()).unwrap()
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
        dbg!(&key);
        let x = dbg!(self.get_query_api_json_all(&params)?);
        match &x["query"]["tokens"][&key] {
            serde_json::Value::String(s) => Ok(s.to_string()),
            _ => Err(From::from("Could not get token")),
        }
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

    pub fn generate_parameter_string(&self, params: &HashMap<&str, &str>) -> String {
        let mut ret = "".to_string();
        params.into_iter().for_each(|x| {
            if !ret.is_empty() {
                ret += "&";
            }
            ret += x.0;
            ret += "=";
            ret += &encode(x.1);
        });
        ret
    }

    pub fn query_api_raw(
        &mut self,
        params: &HashMap<&str, &str>,
        method: &str,
    ) -> Result<String, Box<::std::error::Error>> {
        let mut resp;
        if method == "GET" {
            let url = Url::parse(&self.api_url)?;
            resp = self
                .session
                .get_with(url, |r| dbg!(r.query(params)))
                .unwrap();
        } else if method == "POST" {
            let url = Url::parse(&self.api_url)?;
            resp = self
                .session
                .post_with(url, |r| dbg!(r.form(params)))
                .unwrap();
        } else {
            panic!("Unsupported method");
        }

        match resp.status() {
            StatusCode::OK => Ok(resp.text().unwrap()),
            _ => Err(From::from("Bad things happened")),
        }
    }

    pub fn login(&mut self, lgname: &str, lgpassword: &str) {
        let lgtoken = self.get_token("login").unwrap();
        let params = hashmap!("action"=>"login","lgname"=>&lgname,"lgpassword"=>&lgpassword,"lgtoken"=>&lgtoken);
        let _res = self.post_query_api_json(&params).unwrap(); // TODO check error
        dbg!(_res);
        // dbg!(&self.session.store);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4); // TODO
    }
}
