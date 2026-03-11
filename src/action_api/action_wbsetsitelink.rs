use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

/// Internal data container for `action=wbsetsitelink` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiWbsetsitelinkData {
    id: Option<String>,
    site: Option<String>,
    title: Option<String>,
    linksite: Option<String>,
    linktitle: Option<String>,
    badges: Option<Vec<String>>,
    baserevid: Option<u64>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
    bot: bool,
}

impl ActionApiData for ActionApiWbsetsitelinkData {}

impl ActionApiWbsetsitelinkData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbsetsitelink".to_string());
        Self::add_str(&self.id, "id", &mut params);
        Self::add_str(&self.site, "site", &mut params);
        Self::add_str(&self.title, "title", &mut params);
        Self::add_str(&self.linksite, "linksite", &mut params);
        Self::add_str(&self.linktitle, "linktitle", &mut params);
        Self::add_vec(&self.badges, "badges", &mut params);
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

/// Builder for the `action=wbsetsitelink` API action; uses the typestate pattern to enforce required fields.
#[derive(Debug, Clone)]
pub struct ActionApiWbsetsitelinkBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbsetsitelinkData,
}

impl<T> ActionApiWbsetsitelinkBuilder<T> {
    /// Sets the site identifier of the wiki to link to. `linksite`
    pub fn linksite<S: AsRef<str>>(mut self, linksite: S) -> Self {
        self.data.linksite = Some(linksite.as_ref().to_string());
        self
    }

    /// Sets the title on the linked wiki. `linktitle`
    pub fn linktitle<S: AsRef<str>>(mut self, linktitle: S) -> Self {
        self.data.linktitle = Some(linktitle.as_ref().to_string());
        self
    }

    /// Sets the badge item IDs to assign to the sitelink. `badges`
    pub fn badges<S: Into<String> + Clone>(mut self, badges: &[S]) -> Self {
        self.data.badges = Some(badges.iter().map(|s| s.clone().into()).collect());
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

impl ActionApiWbsetsitelinkBuilder<NoTarget> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbsetsitelinkData::default(),
        }
    }

    /// Sets the entity ID to modify. `id`
    pub fn id<S: AsRef<str>>(mut self, id: S) -> ActionApiWbsetsitelinkBuilder<NoToken> {
        self.data.id = Some(id.as_ref().to_string());
        ActionApiWbsetsitelinkBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    /// Sets the site and title to identify the entity to modify. `site`, `title`
    pub fn site_title<S: AsRef<str>>(
        mut self,
        site: S,
        title: S,
    ) -> ActionApiWbsetsitelinkBuilder<NoToken> {
        self.data.site = Some(site.as_ref().to_string());
        self.data.title = Some(title.as_ref().to_string());
        ActionApiWbsetsitelinkBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiWbsetsitelinkBuilder<NoToken> {
    /// Sets the CSRF token required to perform the write action. `token`
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiWbsetsitelinkBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiWbsetsitelinkBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbsetsitelinkBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbsetsitelinkBuilder<NoTarget> {
        ActionApiWbsetsitelinkBuilder::new()
    }

    #[test]
    fn id_set() {
        let params = new_builder().id("Q42").data.params();
        assert_eq!(params["id"], "Q42");
    }

    #[test]
    fn linksite_set() {
        let params = new_builder().id("Q42").linksite("enwiki").data.params();
        assert_eq!(params["linksite"], "enwiki");
    }

    #[test]
    fn linktitle_set() {
        let params = new_builder()
            .id("Q42")
            .linktitle("Douglas Adams")
            .data
            .params();
        assert_eq!(params["linktitle"], "Douglas Adams");
    }

    #[test]
    fn badges_set() {
        let params = new_builder()
            .id("Q42")
            .badges(&["Q17437798", "Q17437796"])
            .data
            .params();
        assert_eq!(params["badges"], "Q17437798|Q17437796");
    }

    #[test]
    fn token_set() {
        let params = new_builder().id("Q42").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_wbsetsitelink() {
        let params = new_builder().id("Q42").data.params();
        assert_eq!(params["action"], "wbsetsitelink");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().id("Q42").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
