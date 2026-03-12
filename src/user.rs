/*!
The `User` class deals with the (current) ApiSync user.
*/

#![deny(missing_docs)]

use serde_json::Value;

use crate::media_wiki_error::MediaWikiError;

/// `User` contains the login data for the `ApiSync`
#[derive(Debug, Default, Clone)]
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

    /// Checks if user info has been loaded
    pub fn has_user_info(&self) -> bool {
        self.user_info.is_some()
    }

    /// Checks if the user has a specific right (e.g. "bot", "autoconfirmed")
    pub fn has_right(&self, right: &str) -> bool {
        self.user_info
            .as_ref()
            .and_then(|ui| ui["query"]["userinfo"]["rights"].as_array())
            .map(|rights| rights.iter().any(|r| r.as_str() == Some(right)))
            .unwrap_or(false)
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

    /// Sets the user_info
    pub fn set_user_info(&mut self, user_info: Option<Value>) {
        self.user_info = user_info;
    }

    /// Returns the user name ("" if not logged in)
    pub fn user_name(&self) -> &str {
        &self.lgusername
    }

    /// Returns the user id (0 if not logged in)
    pub fn user_id(&self) -> u64 {
        self.lguserid
    }

    /// Tries to set user information from the `ApiSync` call
    pub fn set_from_login(&mut self, login: &Value) -> Result<(), MediaWikiError> {
        if login["result"] == "Success" {
            match login["lgusername"].as_str() {
                Some(s) => self.lgusername = s.to_string(),
                None => {
                    return Err(MediaWikiError::Login(
                        "No lgusername in login result".to_string(),
                    ));
                }
            }
            match login["lguserid"].as_u64() {
                Some(u) => self.lguserid = u,
                None => {
                    return Err(MediaWikiError::Login(
                        "No lguserid in login result".to_string(),
                    ));
                }
            }

            self.is_logged_in = true;
        } else {
            self.is_logged_in = false;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_sync::*;
    use wiremock::matchers::query_param;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn start_mock_sync() -> MockServer {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let server = crate::test_helpers::test_helpers_mod::start_wikidata_mock().await;
            Mock::given(query_param("meta", "userinfo"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "batchcomplete": "",
                    "query": {"userinfo": {
                        "id": 0, "anon": "", "name": "Anonymous",
                        "rights": ["edit", "createaccount", "read", "writeapi", "createpage"]
                    }}
                })))
                .mount(&server)
                .await;
            server
        })
    }

    #[test]
    fn user_not_logged_in_by_default() {
        let user = User::new();
        assert!(!user.logged_in());
    }

    #[test]
    fn user_login() {
        let user_name = "test user 1234";
        let user_id = 12345;
        let mut user = User::new();
        let login = json!({"result":"Success","lgusername":user_name,"lguserid":user_id});
        user.set_from_login(&login).unwrap();
        assert!(user.logged_in());
        assert_eq!(user.user_name(), user_name);
        assert_eq!(user.user_id(), user_id);
    }

    #[test]
    fn user_rights() {
        let server = start_mock_sync();
        let api = ApiSync::new(&server.uri()).unwrap();
        let mut user = User::new();
        api.load_user_info(&mut user).unwrap();
        assert!(!user.is_bot());
        assert!(user.can_edit());
        assert!(!user.can_upload());
        assert!(user.has_right("createaccount"));
        assert!(!user.has_right("thisisnotaright"));
    }

    #[test]
    fn user_has_info() {
        let server = start_mock_sync();
        let api = ApiSync::new(&server.uri()).unwrap();
        let mut user = User::new();
        assert!(!user.has_user_info());
        api.load_user_info(&mut user).unwrap();
        assert!(user.has_user_info());
    }

    #[test]
    fn user_default_values() {
        let user = User::new();
        assert_eq!(user.user_name(), "");
        assert_eq!(user.user_id(), 0);
        assert!(!user.logged_in());
        assert!(!user.has_user_info());
    }

    #[test]
    fn set_from_login_failure() {
        let mut user = User::new();
        let login = json!({"result": "Failed"});
        user.set_from_login(&login).unwrap();
        assert!(!user.logged_in());
    }

    #[test]
    fn set_from_login_missing_username() {
        let mut user = User::new();
        let login = json!({"result": "Success", "lguserid": 123});
        assert!(user.set_from_login(&login).is_err());
    }

    #[test]
    fn set_from_login_missing_userid() {
        let mut user = User::new();
        let login = json!({"result": "Success", "lgusername": "test"});
        assert!(user.set_from_login(&login).is_err());
    }

    #[test]
    fn has_right_without_user_info() {
        let user = User::new();
        assert!(!user.has_right("bot"));
        assert!(!user.is_bot());
        assert!(!user.is_autoconfirmed());
        assert!(!user.can_edit());
        assert!(!user.can_create_page());
        assert!(!user.can_upload());
        assert!(!user.can_move());
        assert!(!user.can_patrol());
    }

    #[test]
    fn set_user_info() {
        let mut user = User::new();
        assert!(!user.has_user_info());
        user.set_user_info(Some(
            json!({"query": {"userinfo": {"rights": ["bot", "edit"]}}}),
        ));
        assert!(user.has_user_info());
        assert!(user.is_bot());
        assert!(user.can_edit());
        assert!(!user.can_upload());
    }

    #[test]
    fn set_user_info_to_none() {
        let mut user = User::new();
        user.set_user_info(Some(json!({})));
        assert!(user.has_user_info());
        user.set_user_info(None);
        assert!(!user.has_user_info());
    }
}
