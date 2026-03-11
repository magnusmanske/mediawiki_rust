use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

/// Internal data container for `action=rollback` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiRollbackData {
    title: Option<String>,
    pageid: Option<u64>,
    user: Option<String>,
    summary: Option<String>,
    markbot: bool,
    watchlist: Option<String>,
    watchlistexpiry: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
}

impl ActionApiData for ActionApiRollbackData {}

impl ActionApiRollbackData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "rollback".to_string());
        Self::add_str(&self.title, "title", &mut params);
        if let Some(v) = self.pageid {
            params.insert("pageid".to_string(), v.to_string());
        }
        Self::add_str(&self.user, "user", &mut params);
        Self::add_str(&self.summary, "summary", &mut params);
        Self::add_boolean(self.markbot, "markbot", &mut params);
        Self::add_str(&self.watchlist, "watchlist", &mut params);
        Self::add_str(&self.watchlistexpiry, "watchlistexpiry", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

/// Builder for the `action=rollback` API call, using a typestate pattern to enforce required fields before execution.
#[derive(Debug, Clone)]
pub struct ActionApiRollbackBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiRollbackData,
}

impl<T> ActionApiRollbackBuilder<T> {
    /// Sets the username whose edits are to be rolled back (`user`).
    pub fn user<S: AsRef<str>>(mut self, user: S) -> Self {
        self.data.user = Some(user.as_ref().to_string());
        self
    }

    /// Sets the custom edit summary for the rollback (`summary`).
    pub fn summary<S: AsRef<str>>(mut self, summary: S) -> Self {
        self.data.summary = Some(summary.as_ref().to_string());
        self
    }

    /// Sets whether to mark the rollback and reverted edits as bot edits (`markbot`).
    pub fn markbot(mut self, markbot: bool) -> Self {
        self.data.markbot = markbot;
        self
    }

    /// Sets the watchlist update mode for the rolled-back page (`watchlist`).
    pub fn watchlist<S: AsRef<str>>(mut self, watchlist: S) -> Self {
        self.data.watchlist = Some(watchlist.as_ref().to_string());
        self
    }

    /// Sets the expiry timestamp for the watchlist entry (`watchlistexpiry`).
    pub fn watchlistexpiry<S: AsRef<str>>(mut self, watchlistexpiry: S) -> Self {
        self.data.watchlistexpiry = Some(watchlistexpiry.as_ref().to_string());
        self
    }

    /// Sets the change tags to apply to the rollback (`tags`).
    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

}

impl ActionApiRollbackBuilder<NoTarget> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiRollbackData::default(),
        }
    }

    /// Sets the title of the page to roll back (`title`).
    pub fn title<S: AsRef<str>>(mut self, title: S) -> ActionApiRollbackBuilder<NoToken> {
        self.data.title = Some(title.as_ref().to_string());
        ActionApiRollbackBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    /// Sets the page ID of the page to roll back (`pageid`).
    pub fn pageid(mut self, pageid: u64) -> ActionApiRollbackBuilder<NoToken> {
        self.data.pageid = Some(pageid);
        ActionApiRollbackBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRollbackBuilder<NoToken> {
    /// Sets the rollback token (`token`).
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiRollbackBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiRollbackBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiRollbackBuilder<Runnable> {
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

    fn new_builder() -> ActionApiRollbackBuilder<NoTarget> {
        ActionApiRollbackBuilder::new()
    }

    #[test]
    fn title_set() {
        let params = new_builder().title("Some Page").data.params();
        assert_eq!(params["title"], "Some Page");
    }

    #[test]
    fn pageid_set() {
        let params = new_builder().pageid(55).data.params();
        assert_eq!(params["pageid"], "55");
    }

    #[test]
    fn user_set() {
        let params = new_builder().title("Foo").user("Vandal").data.params();
        assert_eq!(params["user"], "Vandal");
    }

    #[test]
    fn markbot_flag() {
        let params = new_builder().title("Foo").markbot(true).data.params();
        assert_eq!(params["markbot"], "");
    }

    #[test]
    fn markbot_false_absent() {
        let params = new_builder().title("Foo").data.params();
        assert!(!params.contains_key("markbot"));
    }

    #[test]
    fn token_set() {
        let params = new_builder().title("Foo").token("rollback+\\").data.params();
        assert_eq!(params["token"], "rollback+\\");
    }

    #[test]
    fn action_is_rollback() {
        let params = new_builder().title("Foo").data.params();
        assert_eq!(params["action"], "rollback");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().title("Foo").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
