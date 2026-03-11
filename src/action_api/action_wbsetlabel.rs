use super::{ActionApiData, ActionApiRunnable, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub type NoTarget = super::NoTitlesOrGenerator;

/// Internal data container for `action=wbsetlabel` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiWbsetlabelData {
    id: Option<String>,
    site: Option<String>,
    title: Option<String>,
    language: Option<String>,
    value: Option<String>,
    baserevid: Option<u64>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
    bot: bool,
}

impl ActionApiData for ActionApiWbsetlabelData {}

impl ActionApiWbsetlabelData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbsetlabel".to_string());
        Self::add_str(&self.id, "id", &mut params);
        Self::add_str(&self.site, "site", &mut params);
        Self::add_str(&self.title, "title", &mut params);
        Self::add_str(&self.language, "language", &mut params);
        Self::add_str(&self.value, "value", &mut params);
        if let Some(v) = self.baserevid {
            params.insert("baserevid".to_string(), v.to_string());
        }
        Self::add_str(&self.summary, "summary", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        Self::add_boolean(self.bot, "bot", &mut params);
        params
    }
}

/// Builder for the `action=wbsetlabel` API action; uses the typestate pattern to enforce required fields.
#[derive(Debug, Clone)]
pub struct ActionApiWbsetlabelBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbsetlabelData,
}

impl<T> ActionApiWbsetlabelBuilder<T> {
    /// Sets the language for which to set the label. `language`
    pub fn language<S: AsRef<str>>(mut self, language: S) -> Self {
        self.data.language = Some(language.as_ref().to_string());
        self
    }

    /// Sets the label text to assign. `value`
    pub fn value<S: AsRef<str>>(mut self, value: S) -> Self {
        self.data.value = Some(value.as_ref().to_string());
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
}

impl ActionApiWbsetlabelBuilder<NoTarget> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbsetlabelData::default(),
        }
    }

    /// Sets the entity ID to modify. `id`
    pub fn id<S: AsRef<str>>(mut self, id: S) -> ActionApiWbsetlabelBuilder<NoToken> {
        self.data.id = Some(id.as_ref().to_string());
        ActionApiWbsetlabelBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    /// Sets the site and title to identify the entity to modify. `site`, `title`
    pub fn site_title<S: AsRef<str>>(
        mut self,
        site: S,
        title: S,
    ) -> ActionApiWbsetlabelBuilder<NoToken> {
        self.data.site = Some(site.as_ref().to_string());
        self.data.title = Some(title.as_ref().to_string());
        ActionApiWbsetlabelBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiWbsetlabelBuilder<NoToken> {
    /// Sets the CSRF token required to perform the write action. `token`
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiWbsetlabelBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiWbsetlabelBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbsetlabelBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbsetlabelBuilder<NoTarget> {
        ActionApiWbsetlabelBuilder::new()
    }

    #[test]
    fn id_set() {
        let params = new_builder().id("Q42").data.params();
        assert_eq!(params["id"], "Q42");
    }

    #[test]
    fn language_set() {
        let params = new_builder().id("Q42").language("en").data.params();
        assert_eq!(params["language"], "en");
    }

    #[test]
    fn value_set() {
        let params = new_builder().id("Q42").value("Douglas Adams").data.params();
        assert_eq!(params["value"], "Douglas Adams");
    }

    #[test]
    fn token_set() {
        let params = new_builder().id("Q42").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_wbsetlabel() {
        let params = new_builder().id("Q42").data.params();
        assert_eq!(params["action"], "wbsetlabel");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().id("Q42").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
