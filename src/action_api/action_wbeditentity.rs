use super::{ActionApiData, ActionApiRunnable, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub type NoTarget = super::NoTitlesOrGenerator;

/// Internal data container for `action=wbeditentity` parameters.
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

/// Builder for the `action=wbeditentity` API action; uses the typestate pattern to enforce required parameters before execution.
#[derive(Debug, Clone)]
pub struct ActionApiWbeditentityBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbeditentityData,
}

impl<T> ActionApiWbeditentityBuilder<T> {
    /// Sets the site identifier used to look up the entity by title. `site`
    pub fn site<S: AsRef<str>>(mut self, site: S) -> Self {
        self.data.site = Some(site.as_ref().to_string());
        self
    }

    /// Sets the page title used with `site` to identify the entity. `title`
    pub fn title<S: AsRef<str>>(mut self, title: S) -> Self {
        self.data.title = Some(title.as_ref().to_string());
        self
    }

    /// Sets the base revision ID for conflict detection. `baserevid`
    pub fn baserevid(mut self, baserevid: u64) -> Self {
        self.data.baserevid = Some(baserevid);
        self
    }

    /// Sets the edit summary. `summary`
    pub fn summary<S: AsRef<str>>(mut self, summary: S) -> Self {
        self.data.summary = Some(summary.as_ref().to_string());
        self
    }

    /// Sets the change tags to apply to the edit. `tags`
    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Marks the edit as a bot edit. `bot`
    pub fn bot(mut self, bot: bool) -> Self {
        self.data.bot = bot;
        self
    }

    /// Sets the serialized JSON data describing the entity changes. `data`
    pub fn data<S: AsRef<str>>(mut self, data: S) -> Self {
        self.data.data = Some(data.as_ref().to_string());
        self
    }

    /// If true, clears all existing data on the entity before applying changes. `clear`
    pub fn clear(mut self, clear: bool) -> Self {
        self.data.clear = clear;
        self
    }
}

impl ActionApiWbeditentityBuilder<NoTarget> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbeditentityData::default(),
        }
    }

    /// Sets the entity ID to edit, advancing the builder state. `id`
    pub fn id<S: AsRef<str>>(mut self, id: S) -> ActionApiWbeditentityBuilder<NoToken> {
        self.data.id = Some(id.as_ref().to_string());
        ActionApiWbeditentityBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    /// Sets the entity type to create a new entity of that type, advancing the builder state. `new`
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
    /// Sets the CSRF token, advancing the builder to the runnable state. `token`
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
