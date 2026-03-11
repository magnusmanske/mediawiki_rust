use super::{ActionApiData, ActionApiRunnable, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub type NoTarget = super::NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbsetaliasesData {
    id: Option<String>,
    site: Option<String>,
    title: Option<String>,
    language: Option<String>,
    add: Option<Vec<String>>,
    remove: Option<Vec<String>>,
    set: Option<Vec<String>>,
    baserevid: Option<u64>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
    bot: bool,
}

impl ActionApiData for ActionApiWbsetaliasesData {}

impl ActionApiWbsetaliasesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbsetaliases".to_string());
        Self::add_str(&self.id, "id", &mut params);
        Self::add_str(&self.site, "site", &mut params);
        Self::add_str(&self.title, "title", &mut params);
        Self::add_str(&self.language, "language", &mut params);
        Self::add_vec(&self.add, "add", &mut params);
        Self::add_vec(&self.remove, "remove", &mut params);
        Self::add_vec(&self.set, "set", &mut params);
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
pub struct ActionApiWbsetaliasesBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbsetaliasesData,
}

impl<T> ActionApiWbsetaliasesBuilder<T> {
    pub fn language<S: AsRef<str>>(mut self, language: S) -> Self {
        self.data.language = Some(language.as_ref().to_string());
        self
    }

    pub fn add<S: Into<String> + Clone>(mut self, add: &[S]) -> Self {
        self.data.add = Some(add.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn remove<S: Into<String> + Clone>(mut self, remove: &[S]) -> Self {
        self.data.remove = Some(remove.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn set_aliases<S: Into<String> + Clone>(mut self, set: &[S]) -> Self {
        self.data.set = Some(set.iter().map(|s| s.clone().into()).collect());
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

impl ActionApiWbsetaliasesBuilder<NoTarget> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbsetaliasesData::default(),
        }
    }

    pub fn id<S: AsRef<str>>(mut self, id: S) -> ActionApiWbsetaliasesBuilder<NoToken> {
        self.data.id = Some(id.as_ref().to_string());
        ActionApiWbsetaliasesBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    pub fn site_title<S: AsRef<str>>(
        mut self,
        site: S,
        title: S,
    ) -> ActionApiWbsetaliasesBuilder<NoToken> {
        self.data.site = Some(site.as_ref().to_string());
        self.data.title = Some(title.as_ref().to_string());
        ActionApiWbsetaliasesBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiWbsetaliasesBuilder<NoToken> {
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiWbsetaliasesBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiWbsetaliasesBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbsetaliasesBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbsetaliasesBuilder<NoTarget> {
        ActionApiWbsetaliasesBuilder::new()
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
    fn add_set() {
        let params = new_builder().id("Q42").add(&["foo", "bar"]).data.params();
        assert_eq!(params["add"], "foo|bar");
    }

    #[test]
    fn remove_set() {
        let params = new_builder().id("Q42").remove(&["foo"]).data.params();
        assert_eq!(params["remove"], "foo");
    }

    #[test]
    fn set_aliases_set() {
        let params = new_builder()
            .id("Q42")
            .set_aliases(&["foo", "bar"])
            .data
            .params();
        assert_eq!(params["set"], "foo|bar");
    }

    #[test]
    fn token_set() {
        let params = new_builder().id("Q42").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_wbsetaliases() {
        let params = new_builder().id("Q42").data.params();
        assert_eq!(params["action"], "wbsetaliases");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().id("Q42").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
