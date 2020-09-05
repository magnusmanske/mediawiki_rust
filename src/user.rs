/*!
The `User` class deals with the (current) ApiSync user.
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

use crate::api::ApiError;
use serde_json::Value;

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

    /// Checks is the user has a spefic right (e.g. "bot", "autocinfirmed")
    pub fn has_right(&self, right: &str) -> bool {
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
    pub fn set_from_login(&mut self, login: &Value) -> Result<(), ApiError> {
        if login["result"] == "Success" {
            match login["lgusername"].as_str() {
                Some(s) => self.lgusername = s.to_string(),
                None => {
                    return Err(ApiError::LoginFailure(
                        "No lgusername in login result".to_string(),
                    ))
                }
            }
            match login["lguserid"].as_u64() {
                Some(u) => self.lguserid = u,
                None => {
                    return Err(ApiError::LoginFailure(
                        "No lguserid in login result".to_string(),
                    ))
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

    fn wd_api() -> ApiSync {
        ApiSync::new("https://www.wikidata.org/w/api.php").unwrap()
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
        let mut user = User::new();
        wd_api().load_user_info(&mut user).unwrap();
        assert!(!user.is_bot());
        assert!(user.can_edit());
        assert!(!user.can_upload());
        assert!(user.has_right("createaccount"));
        assert!(!user.has_right("thisisnotaright"));
    }

    #[test]
    fn user_has_info() {
        let mut user = User::new();
        assert!(!user.has_user_info());
        wd_api().load_user_info(&mut user).unwrap();
        assert!(user.has_user_info());
    }
}
