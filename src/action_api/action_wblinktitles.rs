use super::{ActionApiData, ActionApiRunnable, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub type NoTarget = super::NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWblinktitlesData {
    tosite: Option<String>,
    totitle: Option<String>,
    fromsite: Option<String>,
    fromtitle: Option<String>,
    token: Option<String>,
    bot: bool,
}

impl ActionApiData for ActionApiWblinktitlesData {}

impl ActionApiWblinktitlesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wblinktitles".to_string());
        Self::add_str(&self.tosite, "tosite", &mut params);
        Self::add_str(&self.totitle, "totitle", &mut params);
        Self::add_str(&self.fromsite, "fromsite", &mut params);
        Self::add_str(&self.fromtitle, "fromtitle", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        Self::add_boolean(self.bot, "bot", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiWblinktitlesBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWblinktitlesData,
}

impl<T> ActionApiWblinktitlesBuilder<T> {
    pub fn tosite<S: AsRef<str>>(mut self, tosite: S) -> Self {
        self.data.tosite = Some(tosite.as_ref().to_string());
        self
    }

    pub fn totitle<S: AsRef<str>>(mut self, totitle: S) -> Self {
        self.data.totitle = Some(totitle.as_ref().to_string());
        self
    }

    pub fn fromsite<S: AsRef<str>>(mut self, fromsite: S) -> Self {
        self.data.fromsite = Some(fromsite.as_ref().to_string());
        self
    }

    pub fn fromtitle<S: AsRef<str>>(mut self, fromtitle: S) -> Self {
        self.data.fromtitle = Some(fromtitle.as_ref().to_string());
        self
    }

    pub fn bot(mut self, bot: bool) -> Self {
        self.data.bot = bot;
        self
    }
}

impl ActionApiWblinktitlesBuilder<NoTarget> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWblinktitlesData::default(),
        }
    }

    pub fn link<S: AsRef<str>>(
        mut self,
        tosite: S,
        totitle: S,
        fromsite: S,
        fromtitle: S,
    ) -> ActionApiWblinktitlesBuilder<NoToken> {
        self.data.tosite = Some(tosite.as_ref().to_string());
        self.data.totitle = Some(totitle.as_ref().to_string());
        self.data.fromsite = Some(fromsite.as_ref().to_string());
        self.data.fromtitle = Some(fromtitle.as_ref().to_string());
        ActionApiWblinktitlesBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiWblinktitlesBuilder<NoToken> {
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiWblinktitlesBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiWblinktitlesBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWblinktitlesBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWblinktitlesBuilder<NoTarget> {
        ActionApiWblinktitlesBuilder::new()
    }

    #[test]
    fn link_sets_all_params() {
        let params = new_builder()
            .link("enwiki", "Douglas Adams", "dewiki", "Douglas Adams")
            .data
            .params();
        assert_eq!(params["tosite"], "enwiki");
        assert_eq!(params["totitle"], "Douglas Adams");
        assert_eq!(params["fromsite"], "dewiki");
        assert_eq!(params["fromtitle"], "Douglas Adams");
    }

    #[test]
    fn token_set() {
        let params = new_builder()
            .link("enwiki", "Foo", "dewiki", "Foo")
            .token("csrf+\\")
            .data
            .params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn bot_flag() {
        let params = new_builder()
            .link("enwiki", "Foo", "dewiki", "Foo")
            .bot(true)
            .data
            .params();
        assert_eq!(params["bot"], "");
    }

    #[test]
    fn action_is_wblinktitles() {
        let params = new_builder()
            .link("enwiki", "Foo", "dewiki", "Foo")
            .data
            .params();
        assert_eq!(params["action"], "wblinktitles");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().link("enwiki", "Foo", "dewiki", "Foo").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
