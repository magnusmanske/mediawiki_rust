use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub(crate) type NoTitle = NoTitlesOrGenerator;

/// Internal data container for `action=stashedit` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiStasheditData {
    title: Option<String>,
    section: Option<String>,
    sectiontitle: Option<String>,
    text: Option<String>,
    contentmodel: Option<String>,
    contentformat: Option<String>,
    baserevid: Option<u64>,
    summary: Option<String>,
    token: Option<String>,
}

impl ActionApiData for ActionApiStasheditData {}

impl ActionApiStasheditData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "stashedit".to_string());
        Self::add_str(&self.title, "title", &mut params);
        Self::add_str(&self.section, "section", &mut params);
        Self::add_str(&self.sectiontitle, "sectiontitle", &mut params);
        Self::add_str(&self.text, "text", &mut params);
        Self::add_str(&self.contentmodel, "contentmodel", &mut params);
        Self::add_str(&self.contentformat, "contentformat", &mut params);
        if let Some(id) = self.baserevid {
            params.insert("baserevid".to_string(), id.to_string());
        }
        Self::add_str(&self.summary, "summary", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

/// Builder for `action=stashedit`. Call `.title()` then `.token()` to make it runnable.
#[derive(Debug, Clone)]
pub struct ActionApiStasheditBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiStasheditData,
}

impl<T> ActionApiStasheditBuilder<T> {
    /// Section index, `"new"` for new section, or `"T-"` for the intro section (`section`).
    pub fn section<S: AsRef<str>>(mut self, section: S) -> Self {
        self.data.section = Some(section.as_ref().to_string());
        self
    }

    /// Title for a new section (`sectiontitle`).
    pub fn sectiontitle<S: AsRef<str>>(mut self, sectiontitle: S) -> Self {
        self.data.sectiontitle = Some(sectiontitle.as_ref().to_string());
        self
    }

    /// Page content (`text`).
    pub fn text<S: AsRef<str>>(mut self, text: S) -> Self {
        self.data.text = Some(text.as_ref().to_string());
        self
    }

    /// Content model of the new content (`contentmodel`).
    pub fn contentmodel<S: AsRef<str>>(mut self, contentmodel: S) -> Self {
        self.data.contentmodel = Some(contentmodel.as_ref().to_string());
        self
    }

    /// Serialization format used for the input text (`contentformat`).
    pub fn contentformat<S: AsRef<str>>(mut self, contentformat: S) -> Self {
        self.data.contentformat = Some(contentformat.as_ref().to_string());
        self
    }

    /// Revision ID of the base revision (`baserevid`).
    pub fn baserevid(mut self, baserevid: u64) -> Self {
        self.data.baserevid = Some(baserevid);
        self
    }

    /// Edit summary (`summary`).
    pub fn summary<S: AsRef<str>>(mut self, summary: S) -> Self {
        self.data.summary = Some(summary.as_ref().to_string());
        self
    }
}

impl ActionApiStasheditBuilder<NoTitle> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiStasheditData::default(),
        }
    }

    /// Title of the page being edited (`title`).
    pub fn title<S: AsRef<str>>(mut self, title: S) -> ActionApiStasheditBuilder<NoToken> {
        self.data.title = Some(title.as_ref().to_string());
        ActionApiStasheditBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiStasheditBuilder<NoToken> {
    /// CSRF token required to perform the action (`token`).
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiStasheditBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiStasheditBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiStasheditBuilder<Runnable> {
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

    fn new_builder() -> ActionApiStasheditBuilder<NoTitle> {
        ActionApiStasheditBuilder::new()
    }

    #[test]
    fn title_set() {
        let params = new_builder().title("TestPage").data.params();
        assert_eq!(params["title"], "TestPage");
    }

    #[test]
    fn section_set() {
        let params = new_builder().title("TestPage").section("0").data.params();
        assert_eq!(params["section"], "0");
    }

    #[test]
    fn text_set() {
        let params = new_builder().title("TestPage").text("Hello world").data.params();
        assert_eq!(params["text"], "Hello world");
    }

    #[test]
    fn contentmodel_set() {
        let params = new_builder().title("TestPage").contentmodel("wikitext").data.params();
        assert_eq!(params["contentmodel"], "wikitext");
    }

    #[test]
    fn baserevid_set() {
        let params = new_builder().title("TestPage").baserevid(12345).data.params();
        assert_eq!(params["baserevid"], "12345");
    }

    #[test]
    fn summary_set() {
        let params = new_builder().title("TestPage").summary("My edit").data.params();
        assert_eq!(params["summary"], "My edit");
    }

    #[test]
    fn token_set() {
        let params = new_builder().title("TestPage").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_stashedit() {
        let params = new_builder().title("TestPage").data.params();
        assert_eq!(params["action"], "stashedit");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().title("TestPage").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
