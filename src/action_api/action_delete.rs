use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiDeleteData {
    title: Option<String>,
    pageid: Option<u64>,
    reason: Option<String>,
    tags: Option<Vec<String>>,
    deletetalk: bool,
    watchlist: Option<String>,
    watchlistexpiry: Option<String>,
    oldimage: Option<String>,
    token: Option<String>,
}

impl ActionApiData for ActionApiDeleteData {}

impl ActionApiDeleteData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "delete".to_string());
        Self::add_str(&self.title, "title", &mut params);
        if let Some(v) = self.pageid {
            params.insert("pageid".to_string(), v.to_string());
        }
        Self::add_str(&self.reason, "reason", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_boolean(self.deletetalk, "deletetalk", &mut params);
        Self::add_str(&self.watchlist, "watchlist", &mut params);
        Self::add_str(&self.watchlistexpiry, "watchlistexpiry", &mut params);
        Self::add_str(&self.oldimage, "oldimage", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiDeleteBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiDeleteData,
}

impl<T> ActionApiDeleteBuilder<T> {
    pub fn reason<S: AsRef<str>>(mut self, reason: S) -> Self {
        self.data.reason = Some(reason.as_ref().to_string());
        self
    }

    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn deletetalk(mut self, deletetalk: bool) -> Self {
        self.data.deletetalk = deletetalk;
        self
    }

    pub fn watchlist<S: AsRef<str>>(mut self, watchlist: S) -> Self {
        self.data.watchlist = Some(watchlist.as_ref().to_string());
        self
    }

    pub fn watchlistexpiry<S: AsRef<str>>(mut self, watchlistexpiry: S) -> Self {
        self.data.watchlistexpiry = Some(watchlistexpiry.as_ref().to_string());
        self
    }

    pub fn oldimage<S: AsRef<str>>(mut self, oldimage: S) -> Self {
        self.data.oldimage = Some(oldimage.as_ref().to_string());
        self
    }

    pub fn token<S: AsRef<str>>(mut self, token: S) -> Self {
        self.data.token = Some(token.as_ref().to_string());
        self
    }
}

impl ActionApiDeleteBuilder<NoTarget> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiDeleteData::default(),
        }
    }

    pub fn title<S: AsRef<str>>(mut self, title: S) -> ActionApiDeleteBuilder<Runnable> {
        self.data.title = Some(title.as_ref().to_string());
        ActionApiDeleteBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    pub fn pageid(mut self, pageid: u64) -> ActionApiDeleteBuilder<Runnable> {
        self.data.pageid = Some(pageid);
        ActionApiDeleteBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiDeleteBuilder<Runnable> {
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

    fn new_builder() -> ActionApiDeleteBuilder<NoTarget> {
        ActionApiDeleteBuilder::new()
    }

    #[test]
    fn title_set() {
        let params = new_builder().title("Some Page").data.params();
        assert_eq!(params["title"], "Some Page");
    }

    #[test]
    fn pageid_set() {
        let params = new_builder().pageid(42).data.params();
        assert_eq!(params["pageid"], "42");
    }

    #[test]
    fn reason_set() {
        let params = new_builder().title("Foo").reason("Vandalism").data.params();
        assert_eq!(params["reason"], "Vandalism");
    }

    #[test]
    fn deletetalk_flag() {
        let params = new_builder().title("Foo").deletetalk(true).data.params();
        assert_eq!(params["deletetalk"], "");
    }

    #[test]
    fn deletetalk_false_absent() {
        let params = new_builder().title("Foo").data.params();
        assert!(!params.contains_key("deletetalk"));
    }

    #[test]
    fn token_set() {
        let params = new_builder().title("Foo").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_delete() {
        let params = new_builder().title("Foo").data.params();
        assert_eq!(params["action"], "delete");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().title("Foo");
        assert_eq!(builder.http_method(), "POST");
    }
}
