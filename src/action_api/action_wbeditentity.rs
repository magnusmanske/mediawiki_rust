use super::{ActionApiData, ActionApiRunnable, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub type NoTarget = super::NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbeditentityData {
    id: Option<String>,
    new_type: Option<String>,
    site: Option<String>,
    title: Option<String>,
    baserevid: Option<u64>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
    bot: bool,
    data: Option<String>,
    clear: bool,
}

impl ActionApiData for ActionApiWbeditentityData {}

impl ActionApiWbeditentityData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbeditentity".to_string());
        Self::add_str(&self.id, "id", &mut params);
        Self::add_str(&self.new_type, "new", &mut params);
        Self::add_str(&self.site, "site", &mut params);
        Self::add_str(&self.title, "title", &mut params);
        if let Some(v) = self.baserevid {
            params.insert("baserevid".to_string(), v.to_string());
        }
        Self::add_str(&self.summary, "summary", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        Self::add_boolean(self.bot, "bot", &mut params);
        Self::add_str(&self.data, "data", &mut params);
        Self::add_boolean(self.clear, "clear", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiWbeditentityBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbeditentityData,
}

impl<T> ActionApiWbeditentityBuilder<T> {
    pub fn site<S: AsRef<str>>(mut self, site: S) -> Self {
        self.data.site = Some(site.as_ref().to_string());
        self
    }

    pub fn title<S: AsRef<str>>(mut self, title: S) -> Self {
        self.data.title = Some(title.as_ref().to_string());
        self
    }

    pub fn baserevid(mut self, baserevid: u64) -> Self {
        self.data.baserevid = Some(baserevid);
        self
    }

    pub fn summary<S: AsRef<str>>(mut self, summary: S) -> Self {
        self.data.summary = Some(summary.as_ref().to_string());
        self
    }

    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn bot(mut self, bot: bool) -> Self {
        self.data.bot = bot;
        self
    }

    pub fn data<S: AsRef<str>>(mut self, data: S) -> Self {
        self.data.data = Some(data.as_ref().to_string());
        self
    }

    pub fn clear(mut self, clear: bool) -> Self {
        self.data.clear = clear;
        self
    }
}

impl ActionApiWbeditentityBuilder<NoTarget> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbeditentityData::default(),
        }
    }

    pub fn id<S: AsRef<str>>(mut self, id: S) -> ActionApiWbeditentityBuilder<NoToken> {
        self.data.id = Some(id.as_ref().to_string());
        ActionApiWbeditentityBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    pub fn new_type<S: AsRef<str>>(
        mut self,
        new_type: S,
    ) -> ActionApiWbeditentityBuilder<NoToken> {
        self.data.new_type = Some(new_type.as_ref().to_string());
        ActionApiWbeditentityBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiWbeditentityBuilder<NoToken> {
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiWbeditentityBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiWbeditentityBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbeditentityBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbeditentityBuilder<NoTarget> {
        ActionApiWbeditentityBuilder::new()
    }

    #[test]
    fn id_set() {
        let params = new_builder().id("Q42").data.params();
        assert_eq!(params["id"], "Q42");
    }

    #[test]
    fn new_type_set() {
        let params = new_builder().new_type("item").data.params();
        assert_eq!(params["new"], "item");
    }

    #[test]
    fn data_set() {
        let params = new_builder().id("Q42").data("{}").data.params();
        assert_eq!(params["data"], "{}");
    }

    #[test]
    fn bot_flag() {
        let params = new_builder().id("Q42").bot(true).data.params();
        assert_eq!(params["bot"], "");
    }

    #[test]
    fn token_set() {
        let params = new_builder().id("Q42").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_wbeditentity() {
        let params = new_builder().id("Q42").data.params();
        assert_eq!(params["action"], "wbeditentity");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().id("Q42").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
