use super::{ActionApiData, ActionApiRunnable, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub type NoTarget = super::NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbsetdescriptionData {
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

impl ActionApiData for ActionApiWbsetdescriptionData {}

impl ActionApiWbsetdescriptionData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbsetdescription".to_string());
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

#[derive(Debug, Clone)]
pub struct ActionApiWbsetdescriptionBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbsetdescriptionData,
}

impl<T> ActionApiWbsetdescriptionBuilder<T> {
    pub fn language<S: AsRef<str>>(mut self, language: S) -> Self {
        self.data.language = Some(language.as_ref().to_string());
        self
    }

    pub fn value<S: AsRef<str>>(mut self, value: S) -> Self {
        self.data.value = Some(value.as_ref().to_string());
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
}

impl ActionApiWbsetdescriptionBuilder<NoTarget> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbsetdescriptionData::default(),
        }
    }

    pub fn id<S: AsRef<str>>(mut self, id: S) -> ActionApiWbsetdescriptionBuilder<NoToken> {
        self.data.id = Some(id.as_ref().to_string());
        ActionApiWbsetdescriptionBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    pub fn site_title<S: AsRef<str>>(
        mut self,
        site: S,
        title: S,
    ) -> ActionApiWbsetdescriptionBuilder<NoToken> {
        self.data.site = Some(site.as_ref().to_string());
        self.data.title = Some(title.as_ref().to_string());
        ActionApiWbsetdescriptionBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiWbsetdescriptionBuilder<NoToken> {
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiWbsetdescriptionBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiWbsetdescriptionBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbsetdescriptionBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbsetdescriptionBuilder<NoTarget> {
        ActionApiWbsetdescriptionBuilder::new()
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
        let params = new_builder()
            .id("Q42")
            .value("British author")
            .data
            .params();
        assert_eq!(params["value"], "British author");
    }

    #[test]
    fn token_set() {
        let params = new_builder().id("Q42").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_wbsetdescription() {
        let params = new_builder().id("Q42").data.params();
        assert_eq!(params["action"], "wbsetdescription");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().id("Q42").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
