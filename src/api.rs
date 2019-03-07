extern crate reqwest;

use serde_json::Value;
use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug)]
pub struct Api {
    api_url: String,
    siteinfo: Option<Value>,
}

impl Api {
    pub fn new(api_url: &str) -> Api {
        let ret = Api {
            api_url: api_url.to_string(),
            siteinfo: None,
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

    fn get_site_info(&self) -> Result<Value, Box<::std::error::Error>> {
        let mut params = HashMap::new();
        params.insert("action", "query");
        params.insert("meta", "siteinfo");
        params.insert(
            "siprop",
            "general|namespaces|namespacealiases|libraries|extensions|statistics",
        );
        self.get_query_api_json(&params)
    }

    pub fn get_token(&self, token_type: &str) -> String {
        let mut params = HashMap::new();
        params.insert("action", "query");
        params.insert("meta", "tokens");
        params.insert("type", token_type);
        let x = self.get_query_api_json_all(&params).unwrap();
        let token;
        match &x["query"]["tokens"]["logintoken"] {
            serde_json::Value::String(s) => token = s,
            _ => panic!("No token!"),
        }
        token.to_string()
    }

    pub fn get_query_api_json_all(
        &self,
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
        &self,
        params: &HashMap<&str, &str>,
    ) -> Result<Value, Box<::std::error::Error>> {
        let t = self.get_query_api_raw(params)?;
        let v: Value = serde_json::from_str(&t)?;
        Ok(v)
    }

    pub fn get_query_api_raw(
        &self,
        params: &HashMap<&str, &str>,
    ) -> Result<String, Box<::std::error::Error>> {
        let mut url = self.api_url.clone();
        url += "?format=json"; // Enforce JSON
        params.into_iter().for_each(|x| {
            if *x.0 != "format" {
                url += "&";
                url += x.0;
                url += "=";
                url += &encode(x.1);
            }
        });
        println!("{}", &url);
        let mut resp = reqwest::get(&url)?;
        let t = resp.text()?;
        Ok(t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
