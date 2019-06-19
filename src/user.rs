/*!
The `User` class deals with the (current) Api user.
*/

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

extern crate base64;
extern crate cookie;
extern crate crypto;
extern crate reqwest;

use crate::api::Api;
use serde_json::Value;
use std::collections::HashMap;

/// `User` contains the login data for the `Api`
#[derive(Debug, Clone)]
pub struct User {
    lgusername: String,
    lguserid: u64,
    is_logged_in: bool,
    user_info: Option<Value>,
}

impl User {
    /// Returns a new, blank, not-logged-in user
    pub fn new() -> User {
        User {
            lgusername: "".into(),
            lguserid: 0,
            is_logged_in: false,
            user_info: None,
        }
    }

    /// Checks if the user is logged in
    pub fn logged_in(&self) -> bool {
        self.is_logged_in
    }

    /// Checks is the user has a spefic right (e.g. "bot", "autocinfirmed")
    pub fn has_right(&self, right: &str) -> bool {
        if !self.logged_in() {
            return false;
        }
        match &self.user_info {
            Some(ui) => {
                ui["query"]["userinfo"]["rights"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter(|x| x.as_str().unwrap_or("") == right)
                    .count()
                    > 0
            }
            None => false,
        }
    }

    /// Checks if the user has a bot flag
    pub fn is_bot(&self) -> bool {
        self.has_right("bot")
    }

    /// Checks if the user is autoconfirmed
    pub fn is_autoconfirmed(&self) -> bool {
        self.has_right("autoconfirmed")
    }

    /// Checks if the user is allowed to edit
    pub fn can_edit(&self) -> bool {
        self.has_right("edit")
    }

    /// Checks if the user is allowed to create a page
    pub fn can_create_page(&self) -> bool {
        self.has_right("createpage")
    }

    /// Checks if the user is allowed to upload a file
    pub fn can_upload(&self) -> bool {
        self.has_right("upload")
    }

    /// Checks if the user is allowed to move (rename) a page
    pub fn can_move(&self) -> bool {
        self.has_right("move")
    }

    /// Checks if the user is allowed to patrol edits
    pub fn can_patrol(&self) -> bool {
        self.has_right("patrol")
    }

    /// Loads the user info, which is stored in the object; returns Ok(()) if successful
    pub fn load_user_info(&mut self, api: &Api) -> Result<(), Box<::std::error::Error>> {
        match self.user_info {
            Some(_) => return Ok(()),
            None => {
                //let params = hashmap!("action".to_string()=>"query".to_string(),"meta".to_string()=>"userinfo","lgpassword".to_string()=>lgpassword.into(),"lgtoken".to_string()=>lgtoken.into());
                let params: HashMap<String, String> = vec![
                    ("action", "query"),
                    ("meta", "userinfo"),
                    ("uiprop", "blockinfo|groups|groupmemberships|implicitgroups|rights|options|ratelimits|realname|registrationdate|unreadcount|centralids|hasmsg"),
                ]
                .iter()
                .map(|x| (x.0.to_string(), x.1.to_string()))
                .collect();
                let res = api.query_api_json(&params, "GET")?;
                self.user_info = Some(res);
                Ok(())
            }
        }
    }

    /// Tries to set user information from the `Api` call
    pub fn set_from_login(&mut self, login: &Value) -> Result<(), String> {
        if login["result"] == "Success" {
            match login["lgusername"].as_str() {
                Some(s) => self.lgusername = s.to_string(),
                None => return Err("No lgusername in login result".to_string()),
            }
            match login["lguserid"].as_u64() {
                Some(u) => self.lguserid = u,
                None => return Err("No lguserid in login result".to_string()),
            }

            self.is_logged_in = true;
        } else {
            self.is_logged_in = false;
        }
        Ok(())
    }
}
