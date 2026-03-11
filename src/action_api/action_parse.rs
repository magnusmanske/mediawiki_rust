use super::{ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ActionApiParseData {
    title: Option<String>,
    text: Option<String>,
    revid: Option<u64>,
    summary: Option<String>,
    page: Option<String>,
    pageid: Option<u64>,
    redirects: bool,
    oldid: Option<u64>,
    prop: Option<Vec<String>>,
    wrapoutputclass: Option<String>,
    parsoid: bool,
    pst: bool,
    onlypst: bool,
    section: Option<String>,
    sectiontitle: Option<String>,
    disablelimitreport: bool,
    disableeditsection: bool,
    disabletoc: bool,
    preview: bool,
    sectionpreview: bool,
    useskin: Option<String>,
    contentformat: Option<String>,
    contentmodel: Option<String>,
    mobileformat: bool,
}

impl ActionApiData for ActionApiParseData {}

impl ActionApiParseData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "parse".to_string());
        Self::add_str(&self.title, "title", &mut params);
        Self::add_str(&self.text, "text", &mut params);
        if let Some(revid) = self.revid {
            params.insert("revid".to_string(), revid.to_string());
        }
        Self::add_str(&self.summary, "summary", &mut params);
        Self::add_str(&self.page, "page", &mut params);
        if let Some(pageid) = self.pageid {
            params.insert("pageid".to_string(), pageid.to_string());
        }
        Self::add_boolean(self.redirects, "redirects", &mut params);
        if let Some(oldid) = self.oldid {
            params.insert("oldid".to_string(), oldid.to_string());
        }
        Self::add_vec(&self.prop, "prop", &mut params);
        Self::add_str(&self.wrapoutputclass, "wrapoutputclass", &mut params);
        Self::add_boolean(self.parsoid, "parsoid", &mut params);
        Self::add_boolean(self.pst, "pst", &mut params);
        Self::add_boolean(self.onlypst, "onlypst", &mut params);
        Self::add_str(&self.section, "section", &mut params);
        Self::add_str(&self.sectiontitle, "sectiontitle", &mut params);
        Self::add_boolean(self.disablelimitreport, "disablelimitreport", &mut params);
        Self::add_boolean(self.disableeditsection, "disableeditsection", &mut params);
        Self::add_boolean(self.disabletoc, "disabletoc", &mut params);
        Self::add_boolean(self.preview, "preview", &mut params);
        Self::add_boolean(self.sectionpreview, "sectionpreview", &mut params);
        Self::add_str(&self.useskin, "useskin", &mut params);
        Self::add_str(&self.contentformat, "contentformat", &mut params);
        Self::add_str(&self.contentmodel, "contentmodel", &mut params);
        Self::add_boolean(self.mobileformat, "mobileformat", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiParseBuilder {
    pub(crate) data: ActionApiParseData,
}

impl ActionApiParseBuilder {
    pub fn new() -> Self {
        Self {
            data: ActionApiParseData::default(),
        }
    }

    pub fn title<S: AsRef<str>>(mut self, title: S) -> Self {
        self.data.title = Some(title.as_ref().to_string());
        self
    }

    pub fn text<S: AsRef<str>>(mut self, text: S) -> Self {
        self.data.text = Some(text.as_ref().to_string());
        self
    }

    pub fn revid(mut self, revid: u64) -> Self {
        self.data.revid = Some(revid);
        self
    }

    pub fn summary<S: AsRef<str>>(mut self, summary: S) -> Self {
        self.data.summary = Some(summary.as_ref().to_string());
        self
    }

    pub fn page<S: AsRef<str>>(mut self, page: S) -> Self {
        self.data.page = Some(page.as_ref().to_string());
        self
    }

    pub fn pageid(mut self, pageid: u64) -> Self {
        self.data.pageid = Some(pageid);
        self
    }

    pub fn redirects(mut self, redirects: bool) -> Self {
        self.data.redirects = redirects;
        self
    }

    pub fn oldid(mut self, oldid: u64) -> Self {
        self.data.oldid = Some(oldid);
        self
    }

    pub fn prop<S: Into<String> + Clone>(mut self, prop: &[S]) -> Self {
        self.data.prop = Some(prop.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn wrapoutputclass<S: AsRef<str>>(mut self, wrapoutputclass: S) -> Self {
        self.data.wrapoutputclass = Some(wrapoutputclass.as_ref().to_string());
        self
    }

    pub fn pst(mut self, pst: bool) -> Self {
        self.data.pst = pst;
        self
    }

    pub fn section<S: AsRef<str>>(mut self, section: S) -> Self {
        self.data.section = Some(section.as_ref().to_string());
        self
    }

    pub fn disablelimitreport(mut self, disablelimitreport: bool) -> Self {
        self.data.disablelimitreport = disablelimitreport;
        self
    }

    pub fn disableeditsection(mut self, disableeditsection: bool) -> Self {
        self.data.disableeditsection = disableeditsection;
        self
    }

    pub fn disabletoc(mut self, disabletoc: bool) -> Self {
        self.data.disabletoc = disabletoc;
        self
    }

    pub fn preview(mut self, preview: bool) -> Self {
        self.data.preview = preview;
        self
    }

    pub fn useskin<S: AsRef<str>>(mut self, useskin: S) -> Self {
        self.data.useskin = Some(useskin.as_ref().to_string());
        self
    }

    pub fn contentformat<S: AsRef<str>>(mut self, contentformat: S) -> Self {
        self.data.contentformat = Some(contentformat.as_ref().to_string());
        self
    }

    pub fn contentmodel<S: AsRef<str>>(mut self, contentmodel: S) -> Self {
        self.data.contentmodel = Some(contentmodel.as_ref().to_string());
        self
    }

    pub fn mobileformat(mut self, mobileformat: bool) -> Self {
        self.data.mobileformat = mobileformat;
        self
    }
}

impl ActionApiRunnable for ActionApiParseBuilder {
    fn params(&self) -> HashMap<String, String> {
        self.data.params()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApi};

    #[test]
    fn page_set() {
        let params = ActionApiParseBuilder::new().page("Albert Einstein").data.params();
        assert_eq!(params["page"], "Albert Einstein");
    }

    #[test]
    fn text_with_contentmodel() {
        let params = ActionApiParseBuilder::new()
            .text("Hello '''world'''")
            .contentmodel("wikitext")
            .data
            .params();
        assert_eq!(params["text"], "Hello '''world'''");
        assert_eq!(params["contentmodel"], "wikitext");
    }

    #[test]
    fn prop_set() {
        let params =
            ActionApiParseBuilder::new().page("Foo").prop(&["text", "links"]).data.params();
        assert_eq!(params["prop"], "text|links");
    }

    #[test]
    fn pageid_set() {
        let params = ActionApiParseBuilder::new().pageid(736).data.params();
        assert_eq!(params["pageid"], "736");
    }

    #[test]
    fn oldid_set() {
        let params = ActionApiParseBuilder::new().oldid(12345).data.params();
        assert_eq!(params["oldid"], "12345");
    }

    #[test]
    fn disabletoc_set() {
        let params = ActionApiParseBuilder::new().page("Foo").disabletoc(true).data.params();
        assert!(params.contains_key("disabletoc"));
    }

    #[test]
    fn action_is_parse() {
        let params = ActionApiParseBuilder::new().data.params();
        assert_eq!(params["action"], "parse");
    }

    #[tokio::test]
    async fn test_parse_page() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApi::parse()
            .page("Albert Einstein")
            .prop(&["text"])
            .disablelimitreport(true)
            .run(&api)
            .await
            .unwrap();
        assert!(result["parse"]["text"].is_object() || result["parse"]["text"].is_string());
    }
}
