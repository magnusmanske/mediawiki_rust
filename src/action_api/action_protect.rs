use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiProtectData {
    title: Option<String>,
    pageid: Option<u64>,
    protections: Option<Vec<String>>,
    expiry: Option<Vec<String>>,
    reason: Option<String>,
    tags: Option<Vec<String>>,
    cascade: bool,
    watchlist: Option<String>,
    watchlistexpiry: Option<String>,
    token: Option<String>,
}

impl ActionApiData for ActionApiProtectData {}

impl ActionApiProtectData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "protect".to_string());
        Self::add_str(&self.title, "title", &mut params);
        if let Some(v) = self.pageid {
            params.insert("pageid".to_string(), v.to_string());
        }
        Self::add_vec(&self.protections, "protections", &mut params);
        Self::add_vec(&self.expiry, "expiry", &mut params);
        Self::add_str(&self.reason, "reason", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_boolean(self.cascade, "cascade", &mut params);
        Self::add_str(&self.watchlist, "watchlist", &mut params);
        Self::add_str(&self.watchlistexpiry, "watchlistexpiry", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiProtectBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiProtectData,
}

impl<T> ActionApiProtectBuilder<T> {
    pub fn protections<S: Into<String> + Clone>(mut self, protections: &[S]) -> Self {
        self.data.protections = Some(protections.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn expiry<S: Into<String> + Clone>(mut self, expiry: &[S]) -> Self {
        self.data.expiry = Some(expiry.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn reason<S: AsRef<str>>(mut self, reason: S) -> Self {
        self.data.reason = Some(reason.as_ref().to_string());
        self
    }

    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn cascade(mut self, cascade: bool) -> Self {
        self.data.cascade = cascade;
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

}

impl ActionApiProtectBuilder<NoTarget> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiProtectData::default(),
        }
    }

    pub fn title<S: AsRef<str>>(mut self, title: S) -> ActionApiProtectBuilder<NoToken> {
        self.data.title = Some(title.as_ref().to_string());
        ActionApiProtectBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    pub fn pageid(mut self, pageid: u64) -> ActionApiProtectBuilder<NoToken> {
        self.data.pageid = Some(pageid);
        ActionApiProtectBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiProtectBuilder<NoToken> {
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiProtectBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiProtectBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiProtectBuilder<Runnable> {
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

    fn new_builder() -> ActionApiProtectBuilder<NoTarget> {
        ActionApiProtectBuilder::new()
    }

    #[test]
    fn title_set() {
        let params = new_builder().title("Some Page").data.params();
        assert_eq!(params["title"], "Some Page");
    }

    #[test]
    fn pageid_set() {
        let params = new_builder().pageid(7).data.params();
        assert_eq!(params["pageid"], "7");
    }

    #[test]
    fn protections_set() {
        let params = new_builder()
            .title("Foo")
            .protections(&["edit=sysop", "move=sysop"])
            .data
            .params();
        assert_eq!(params["protections"], "edit=sysop|move=sysop");
    }

    #[test]
    fn cascade_flag() {
        let params = new_builder().title("Foo").cascade(true).data.params();
        assert_eq!(params["cascade"], "");
    }

    #[test]
    fn cascade_false_absent() {
        let params = new_builder().title("Foo").data.params();
        assert!(!params.contains_key("cascade"));
    }

    #[test]
    fn token_set() {
        let params = new_builder().title("Foo").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_protect() {
        let params = new_builder().title("Foo").data.params();
        assert_eq!(params["action"], "protect");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().title("Foo").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
