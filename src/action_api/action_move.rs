use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoSource = NoTitlesOrGenerator;

/// Internal data container for `action=move` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiMoveData {
    from: Option<String>,
    fromid: Option<u64>,
    to: Option<String>,
    reason: Option<String>,
    movetalk: bool,
    movesubpages: bool,
    noredirect: bool,
    watchlist: Option<String>,
    watchlistexpiry: Option<String>,
    ignorewarnings: bool,
    tags: Option<Vec<String>>,
    token: Option<String>,
}

impl ActionApiData for ActionApiMoveData {}

impl ActionApiMoveData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "move".to_string());
        Self::add_str(&self.from, "from", &mut params);
        if let Some(v) = self.fromid {
            params.insert("fromid".to_string(), v.to_string());
        }
        Self::add_str(&self.to, "to", &mut params);
        Self::add_str(&self.reason, "reason", &mut params);
        Self::add_boolean(self.movetalk, "movetalk", &mut params);
        Self::add_boolean(self.movesubpages, "movesubpages", &mut params);
        Self::add_boolean(self.noredirect, "noredirect", &mut params);
        Self::add_str(&self.watchlist, "watchlist", &mut params);
        Self::add_str(&self.watchlistexpiry, "watchlistexpiry", &mut params);
        Self::add_boolean(self.ignorewarnings, "ignorewarnings", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

/// Builder for `action=move`. Call `.from()` or `.fromid()` to set the source page and `.to()` for the destination, then `.token()` to make it runnable.
#[derive(Debug, Clone)]
pub struct ActionApiMoveBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiMoveData,
}

impl<T> ActionApiMoveBuilder<T> {
    /// Title to rename the page to (`to`).
    pub fn to<S: AsRef<str>>(mut self, to: S) -> Self {
        self.data.to = Some(to.as_ref().to_string());
        self
    }

    /// Reason for the move (`reason`).
    pub fn reason<S: AsRef<str>>(mut self, reason: S) -> Self {
        self.data.reason = Some(reason.as_ref().to_string());
        self
    }

    /// Whether to also move the associated talk page (`movetalk`).
    pub fn movetalk(mut self, movetalk: bool) -> Self {
        self.data.movetalk = movetalk;
        self
    }

    /// Whether to also move subpages of the source page (`movesubpages`).
    pub fn movesubpages(mut self, movesubpages: bool) -> Self {
        self.data.movesubpages = movesubpages;
        self
    }

    /// Whether to suppress creation of a redirect from the old title (`noredirect`).
    pub fn noredirect(mut self, noredirect: bool) -> Self {
        self.data.noredirect = noredirect;
        self
    }

    /// Watchlist treatment for the moved page (`watchlist`).
    pub fn watchlist<S: AsRef<str>>(mut self, watchlist: S) -> Self {
        self.data.watchlist = Some(watchlist.as_ref().to_string());
        self
    }

    /// Expiry time for the watchlist entry (`watchlistexpiry`).
    pub fn watchlistexpiry<S: AsRef<str>>(mut self, watchlistexpiry: S) -> Self {
        self.data.watchlistexpiry = Some(watchlistexpiry.as_ref().to_string());
        self
    }

    /// Whether to ignore any warnings about the move (`ignorewarnings`).
    pub fn ignorewarnings(mut self, ignorewarnings: bool) -> Self {
        self.data.ignorewarnings = ignorewarnings;
        self
    }

    /// Change tags to apply to the move log entry (`tags`).
    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

}

impl ActionApiMoveBuilder<NoSource> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiMoveData::default(),
        }
    }

    /// Title of the page to move (`from`).
    pub fn from<S: AsRef<str>>(mut self, from: S) -> ActionApiMoveBuilder<NoToken> {
        self.data.from = Some(from.as_ref().to_string());
        ActionApiMoveBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    /// Page ID of the page to move (`fromid`).
    pub fn fromid(mut self, fromid: u64) -> ActionApiMoveBuilder<NoToken> {
        self.data.fromid = Some(fromid);
        ActionApiMoveBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiMoveBuilder<NoToken> {
    /// CSRF token required to perform the move (`token`).
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiMoveBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiMoveBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiMoveBuilder<Runnable> {
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

    fn new_builder() -> ActionApiMoveBuilder<NoSource> {
        ActionApiMoveBuilder::new()
    }

    #[test]
    fn from_set() {
        let params = new_builder().from("Old Title").data.params();
        assert_eq!(params["from"], "Old Title");
    }

    #[test]
    fn fromid_set() {
        let params = new_builder().fromid(99).data.params();
        assert_eq!(params["fromid"], "99");
    }

    #[test]
    fn to_set() {
        let params = new_builder().from("Old").to("New Title").data.params();
        assert_eq!(params["to"], "New Title");
    }

    #[test]
    fn movetalk_flag() {
        let params = new_builder().from("Old").movetalk(true).data.params();
        assert_eq!(params["movetalk"], "");
    }

    #[test]
    fn movetalk_false_absent() {
        let params = new_builder().from("Old").data.params();
        assert!(!params.contains_key("movetalk"));
    }

    #[test]
    fn noredirect_flag() {
        let params = new_builder().from("Old").noredirect(true).data.params();
        assert_eq!(params["noredirect"], "");
    }

    #[test]
    fn token_set() {
        let params = new_builder().from("Old").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_move() {
        let params = new_builder().from("Old").data.params();
        assert_eq!(params["action"], "move");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().from("Old").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
