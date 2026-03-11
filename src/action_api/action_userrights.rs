use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

/// Internal data container for `action=userrights` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiUserrightsData {
    user: Option<String>,
    userid: Option<u64>,
    add: Option<Vec<String>>,
    expiry: Option<Vec<String>>,
    remove: Option<Vec<String>>,
    reason: Option<String>,
    token: Option<String>,
    tags: Option<Vec<String>>,
    watchuser: bool,
    watchlistexpiry: Option<String>,
}

impl ActionApiData for ActionApiUserrightsData {}

impl ActionApiUserrightsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "userrights".to_string());
        Self::add_str(&self.user, "user", &mut params);
        if let Some(v) = self.userid {
            params.insert("userid".to_string(), v.to_string());
        }
        Self::add_vec(&self.add, "add", &mut params);
        Self::add_vec(&self.expiry, "expiry", &mut params);
        Self::add_vec(&self.remove, "remove", &mut params);
        Self::add_str(&self.reason, "reason", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_boolean(self.watchuser, "watchuser", &mut params);
        Self::add_str(&self.watchlistexpiry, "watchlistexpiry", &mut params);
        params
    }
}

/// Builder for the `action=userrights` API call, using a typestate pattern to enforce required fields before execution.
#[derive(Debug, Clone)]
pub struct ActionApiUserrightsBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiUserrightsData,
}

impl<T> ActionApiUserrightsBuilder<T> {
    /// Sets the list of groups to add the user to (`add`).
    pub fn add<S: Into<String> + Clone>(mut self, add: &[S]) -> Self {
        self.data.add = Some(add.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Sets the expiry timestamps for the groups being added (`expiry`).
    pub fn expiry<S: Into<String> + Clone>(mut self, expiry: &[S]) -> Self {
        self.data.expiry = Some(expiry.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Sets the list of groups to remove the user from (`remove`).
    pub fn remove<S: Into<String> + Clone>(mut self, remove: &[S]) -> Self {
        self.data.remove = Some(remove.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Sets the reason for the rights change (`reason`).
    pub fn reason<S: AsRef<str>>(mut self, reason: S) -> Self {
        self.data.reason = Some(reason.as_ref().to_string());
        self
    }

    /// Sets the change tags to apply to the userrights log entry (`tags`).
    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Sets whether to watch the user's talk page (`watchuser`).
    pub fn watchuser(mut self, watchuser: bool) -> Self {
        self.data.watchuser = watchuser;
        self
    }

    /// Sets the expiry timestamp for the watchlist entry on the user's talk page (`watchlistexpiry`).
    pub fn watchlistexpiry<S: AsRef<str>>(mut self, watchlistexpiry: S) -> Self {
        self.data.watchlistexpiry = Some(watchlistexpiry.as_ref().to_string());
        self
    }
}

impl ActionApiUserrightsBuilder<NoTarget> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiUserrightsData::default(),
        }
    }

    /// Sets the username whose rights are to be changed (`user`).
    pub fn user<S: AsRef<str>>(mut self, user: S) -> ActionApiUserrightsBuilder<NoToken> {
        self.data.user = Some(user.as_ref().to_string());
        ActionApiUserrightsBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    /// Sets the user ID whose rights are to be changed (`userid`).
    pub fn userid(mut self, userid: u64) -> ActionApiUserrightsBuilder<NoToken> {
        self.data.userid = Some(userid);
        ActionApiUserrightsBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiUserrightsBuilder<NoToken> {
    /// Sets the userrights token (`token`).
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiUserrightsBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiUserrightsBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiUserrightsBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        self.data.params()
    }

    fn http_method(&self) -> &'static str {
        "POST"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiUserrightsBuilder<NoTarget> {
        ActionApiUserrightsBuilder::new()
    }

    #[test]
    fn user_set() {
        let params = new_builder().user("SomeUser").data.params();
        assert_eq!(params["user"], "SomeUser");
    }

    #[test]
    fn userid_set() {
        let params = new_builder().userid(99).data.params();
        assert_eq!(params["userid"], "99");
    }

    #[test]
    fn add_set() {
        let params = new_builder()
            .user("SomeUser")
            .add(&["sysop", "bureaucrat"])
            .data
            .params();
        assert_eq!(params["add"], "sysop|bureaucrat");
    }

    #[test]
    fn remove_set() {
        let params = new_builder()
            .user("SomeUser")
            .remove(&["rollbacker"])
            .data
            .params();
        assert_eq!(params["remove"], "rollbacker");
    }

    #[test]
    fn reason_set() {
        let params = new_builder()
            .user("SomeUser")
            .reason("promotion")
            .data
            .params();
        assert_eq!(params["reason"], "promotion");
    }

    #[test]
    fn token_set() {
        let params = new_builder().user("SomeUser").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_userrights() {
        let params = new_builder().user("SomeUser").data.params();
        assert_eq!(params["action"], "userrights");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().user("SomeUser").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
