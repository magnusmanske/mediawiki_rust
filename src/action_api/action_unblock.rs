use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

/// Internal data container for `action=unblock` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiUnblockData {
    id: Option<u64>,
    user: Option<String>,
    reason: Option<String>,
    tags: Option<Vec<String>>,
    watchuser: bool,
    watchlistexpiry: Option<String>,
    token: Option<String>,
}

impl ActionApiData for ActionApiUnblockData {}

impl ActionApiUnblockData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "unblock".to_string());
        if let Some(v) = self.id {
            params.insert("id".to_string(), v.to_string());
        }
        Self::add_str(&self.user, "user", &mut params);
        Self::add_str(&self.reason, "reason", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_boolean(self.watchuser, "watchuser", &mut params);
        Self::add_str(&self.watchlistexpiry, "watchlistexpiry", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

/// Builder for the `action=unblock` API call, using a typestate pattern to enforce required fields before execution.
#[derive(Debug, Clone)]
pub struct ActionApiUnblockBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiUnblockData,
}

impl<T> ActionApiUnblockBuilder<T> {
    /// Sets the reason for the unblock (`reason`).
    pub fn reason<S: AsRef<str>>(mut self, reason: S) -> Self {
        self.data.reason = Some(reason.as_ref().to_string());
        self
    }

    /// Sets the change tags to apply to the unblock log entry (`tags`).
    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Sets whether to watch the unblocked user's talk page (`watchuser`).
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

impl ActionApiUnblockBuilder<NoTarget> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiUnblockData::default(),
        }
    }

    /// Sets the block ID to unblock (`id`).
    pub fn id(mut self, id: u64) -> ActionApiUnblockBuilder<NoToken> {
        self.data.id = Some(id);
        ActionApiUnblockBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    /// Sets the username to unblock (`user`).
    pub fn user<S: AsRef<str>>(mut self, user: S) -> ActionApiUnblockBuilder<NoToken> {
        self.data.user = Some(user.as_ref().to_string());
        ActionApiUnblockBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiUnblockBuilder<NoToken> {
    /// Sets the CSRF token (`token`).
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiUnblockBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiUnblockBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiUnblockBuilder<Runnable> {
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

    fn new_builder() -> ActionApiUnblockBuilder<NoTarget> {
        ActionApiUnblockBuilder::new()
    }

    #[test]
    fn id_set() {
        let params = new_builder().id(42).data.params();
        assert_eq!(params["id"], "42");
    }

    #[test]
    fn user_set() {
        let params = new_builder().user("SomeUser").data.params();
        assert_eq!(params["user"], "SomeUser");
    }

    #[test]
    fn reason_set() {
        let params = new_builder().id(1).reason("mistake").data.params();
        assert_eq!(params["reason"], "mistake");
    }

    #[test]
    fn token_set() {
        let params = new_builder().id(1).token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_unblock() {
        let params = new_builder().id(1).data.params();
        assert_eq!(params["action"], "unblock");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().id(1).token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
