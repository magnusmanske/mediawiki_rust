use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub(crate) type NoTitle = NoTitlesOrGenerator;

/// Internal data container for `action=undelete` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiUndeleteData {
    title: Option<String>,
    reason: Option<String>,
    tags: Option<Vec<String>>,
    timestamps: Option<Vec<String>>,
    fileids: Option<Vec<u64>>,
    watchlist: Option<String>,
    undeletetalk: bool,
    token: Option<String>,
}

impl ActionApiData for ActionApiUndeleteData {}

impl ActionApiUndeleteData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "undelete".to_string());
        Self::add_str(&self.title, "title", &mut params);
        Self::add_str(&self.reason, "reason", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_vec(&self.timestamps, "timestamps", &mut params);
        if let Some(ref ids) = self.fileids {
            let s: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
            params.insert("fileids".to_string(), s.join("|"));
        }
        Self::add_str(&self.watchlist, "watchlist", &mut params);
        Self::add_boolean(self.undeletetalk, "undeletetalk", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

/// Builder for `action=undelete`. Call `.title()` then `.token()` to make it runnable.
#[derive(Debug, Clone)]
pub struct ActionApiUndeleteBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiUndeleteData,
}

impl<T> ActionApiUndeleteBuilder<T> {
    /// Reason for undeletion (`reason`).
    pub fn reason<S: AsRef<str>>(mut self, reason: S) -> Self {
        self.data.reason = Some(reason.as_ref().to_string());
        self
    }

    /// Tags to apply to the deletion log entry (`tags`).
    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Timestamps of revisions to undelete (`timestamps`).
    pub fn timestamps<S: Into<String> + Clone>(mut self, timestamps: &[S]) -> Self {
        self.data.timestamps = Some(timestamps.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// File IDs to undelete (`fileids`).
    pub fn fileids(mut self, fileids: &[u64]) -> Self {
        self.data.fileids = Some(fileids.to_vec());
        self
    }

    /// Watchlist action (`watchlist`).
    pub fn watchlist<S: AsRef<str>>(mut self, watchlist: S) -> Self {
        self.data.watchlist = Some(watchlist.as_ref().to_string());
        self
    }

    /// Undelete talk page of the page (`undeletetalk`).
    pub fn undeletetalk(mut self, undeletetalk: bool) -> Self {
        self.data.undeletetalk = undeletetalk;
        self
    }
}

impl ActionApiUndeleteBuilder<NoTitle> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiUndeleteData::default(),
        }
    }

    /// Title of the page to undelete (`title`).
    pub fn title<S: AsRef<str>>(mut self, title: S) -> ActionApiUndeleteBuilder<NoToken> {
        self.data.title = Some(title.as_ref().to_string());
        ActionApiUndeleteBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiUndeleteBuilder<NoToken> {
    /// CSRF token required to perform the undeletion (`token`).
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiUndeleteBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiUndeleteBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiUndeleteBuilder<Runnable> {
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

    fn new_builder() -> ActionApiUndeleteBuilder<NoTitle> {
        ActionApiUndeleteBuilder::new()
    }

    #[test]
    fn title_set() {
        let params = new_builder().title("TestPage").data.params();
        assert_eq!(params["title"], "TestPage");
    }

    #[test]
    fn reason_set() {
        let params = new_builder().title("TestPage").reason("Mistake").data.params();
        assert_eq!(params["reason"], "Mistake");
    }

    #[test]
    fn tags_set() {
        let params = new_builder().title("TestPage").tags(&["tag1", "tag2"]).data.params();
        assert_eq!(params["tags"], "tag1|tag2");
    }

    #[test]
    fn timestamps_set() {
        let params = new_builder()
            .title("TestPage")
            .timestamps(&["2024-01-01T00:00:00Z"])
            .data
            .params();
        assert_eq!(params["timestamps"], "2024-01-01T00:00:00Z");
    }

    #[test]
    fn fileids_set() {
        let params = new_builder().title("TestPage").fileids(&[1, 2, 3]).data.params();
        assert_eq!(params["fileids"], "1|2|3");
    }

    #[test]
    fn watchlist_set() {
        let params = new_builder().title("TestPage").watchlist("watch").data.params();
        assert_eq!(params["watchlist"], "watch");
    }

    #[test]
    fn undeletetalk_flag() {
        let params = new_builder().title("TestPage").undeletetalk(true).data.params();
        assert_eq!(params["undeletetalk"], "");
    }

    #[test]
    fn token_set() {
        let params = new_builder().title("TestPage").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_undelete() {
        let params = new_builder().title("TestPage").data.params();
        assert_eq!(params["action"], "undelete");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().title("TestPage").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
