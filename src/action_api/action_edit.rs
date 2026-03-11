use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiEditData {
    title: Option<String>,
    pageid: Option<u64>,
    section: Option<String>,
    sectiontitle: Option<String>,
    text: Option<String>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    minor: bool,
    notminor: bool,
    bot: bool,
    baserevid: Option<u64>,
    basetimestamp: Option<String>,
    starttimestamp: Option<String>,
    recreate: bool,
    createonly: bool,
    nocreate: bool,
    watchlist: Option<String>,
    watchlistexpiry: Option<String>,
    md5: Option<String>,
    prependtext: Option<String>,
    appendtext: Option<String>,
    undo: Option<u64>,
    undoafter: Option<u64>,
    redirect: bool,
    contentformat: Option<String>,
    contentmodel: Option<String>,
    token: Option<String>,
}

impl ActionApiData for ActionApiEditData {}

impl ActionApiEditData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "edit".to_string());
        Self::add_str(&self.title, "title", &mut params);
        if let Some(v) = self.pageid {
            params.insert("pageid".to_string(), v.to_string());
        }
        Self::add_str(&self.section, "section", &mut params);
        Self::add_str(&self.sectiontitle, "sectiontitle", &mut params);
        Self::add_str(&self.text, "text", &mut params);
        Self::add_str(&self.summary, "summary", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_boolean(self.minor, "minor", &mut params);
        Self::add_boolean(self.notminor, "notminor", &mut params);
        Self::add_boolean(self.bot, "bot", &mut params);
        if let Some(v) = self.baserevid {
            params.insert("baserevid".to_string(), v.to_string());
        }
        Self::add_str(&self.basetimestamp, "basetimestamp", &mut params);
        Self::add_str(&self.starttimestamp, "starttimestamp", &mut params);
        Self::add_boolean(self.recreate, "recreate", &mut params);
        Self::add_boolean(self.createonly, "createonly", &mut params);
        Self::add_boolean(self.nocreate, "nocreate", &mut params);
        Self::add_str(&self.watchlist, "watchlist", &mut params);
        Self::add_str(&self.watchlistexpiry, "watchlistexpiry", &mut params);
        Self::add_str(&self.md5, "md5", &mut params);
        Self::add_str(&self.prependtext, "prependtext", &mut params);
        Self::add_str(&self.appendtext, "appendtext", &mut params);
        if let Some(v) = self.undo {
            params.insert("undo".to_string(), v.to_string());
        }
        if let Some(v) = self.undoafter {
            params.insert("undoafter".to_string(), v.to_string());
        }
        Self::add_boolean(self.redirect, "redirect", &mut params);
        Self::add_str(&self.contentformat, "contentformat", &mut params);
        Self::add_str(&self.contentmodel, "contentmodel", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiEditBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiEditData,
}

impl<T> ActionApiEditBuilder<T> {
    pub fn section<S: AsRef<str>>(mut self, section: S) -> Self {
        self.data.section = Some(section.as_ref().to_string());
        self
    }

    pub fn sectiontitle<S: AsRef<str>>(mut self, sectiontitle: S) -> Self {
        self.data.sectiontitle = Some(sectiontitle.as_ref().to_string());
        self
    }

    pub fn text<S: AsRef<str>>(mut self, text: S) -> Self {
        self.data.text = Some(text.as_ref().to_string());
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

    pub fn minor(mut self, minor: bool) -> Self {
        self.data.minor = minor;
        self
    }

    pub fn notminor(mut self, notminor: bool) -> Self {
        self.data.notminor = notminor;
        self
    }

    pub fn bot(mut self, bot: bool) -> Self {
        self.data.bot = bot;
        self
    }

    pub fn baserevid(mut self, baserevid: u64) -> Self {
        self.data.baserevid = Some(baserevid);
        self
    }

    pub fn basetimestamp<S: AsRef<str>>(mut self, basetimestamp: S) -> Self {
        self.data.basetimestamp = Some(basetimestamp.as_ref().to_string());
        self
    }

    pub fn starttimestamp<S: AsRef<str>>(mut self, starttimestamp: S) -> Self {
        self.data.starttimestamp = Some(starttimestamp.as_ref().to_string());
        self
    }

    pub fn recreate(mut self, recreate: bool) -> Self {
        self.data.recreate = recreate;
        self
    }

    pub fn createonly(mut self, createonly: bool) -> Self {
        self.data.createonly = createonly;
        self
    }

    pub fn nocreate(mut self, nocreate: bool) -> Self {
        self.data.nocreate = nocreate;
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

    pub fn md5<S: AsRef<str>>(mut self, md5: S) -> Self {
        self.data.md5 = Some(md5.as_ref().to_string());
        self
    }

    pub fn prependtext<S: AsRef<str>>(mut self, prependtext: S) -> Self {
        self.data.prependtext = Some(prependtext.as_ref().to_string());
        self
    }

    pub fn appendtext<S: AsRef<str>>(mut self, appendtext: S) -> Self {
        self.data.appendtext = Some(appendtext.as_ref().to_string());
        self
    }

    pub fn undo(mut self, undo: u64) -> Self {
        self.data.undo = Some(undo);
        self
    }

    pub fn undoafter(mut self, undoafter: u64) -> Self {
        self.data.undoafter = Some(undoafter);
        self
    }

    pub fn redirect(mut self, redirect: bool) -> Self {
        self.data.redirect = redirect;
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

    pub fn token<S: AsRef<str>>(mut self, token: S) -> Self {
        self.data.token = Some(token.as_ref().to_string());
        self
    }
}

impl ActionApiEditBuilder<NoTarget> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiEditData::default(),
        }
    }

    pub fn title<S: AsRef<str>>(mut self, title: S) -> ActionApiEditBuilder<Runnable> {
        self.data.title = Some(title.as_ref().to_string());
        ActionApiEditBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    pub fn pageid(mut self, pageid: u64) -> ActionApiEditBuilder<Runnable> {
        self.data.pageid = Some(pageid);
        ActionApiEditBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiEditBuilder<Runnable> {
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

    fn new_builder() -> ActionApiEditBuilder<NoTarget> {
        ActionApiEditBuilder::new()
    }

    #[test]
    fn title_set() {
        let params = new_builder().title("Test Page").data.params();
        assert_eq!(params["title"], "Test Page");
    }

    #[test]
    fn pageid_set() {
        let params = new_builder().pageid(12345).data.params();
        assert_eq!(params["pageid"], "12345");
    }

    #[test]
    fn text_set() {
        let params = new_builder().title("Foo").text("Hello world").data.params();
        assert_eq!(params["text"], "Hello world");
    }

    #[test]
    fn minor_flag() {
        let params = new_builder().title("Foo").minor(true).data.params();
        assert_eq!(params["minor"], "");
    }

    #[test]
    fn minor_flag_false_absent() {
        let params = new_builder().title("Foo").data.params();
        assert!(!params.contains_key("minor"));
    }

    #[test]
    fn bot_flag() {
        let params = new_builder().title("Foo").bot(true).data.params();
        assert_eq!(params["bot"], "");
    }

    #[test]
    fn token_set() {
        let params = new_builder().title("Foo").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_edit() {
        let params = new_builder().title("Foo").data.params();
        assert_eq!(params["action"], "edit");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().title("Foo");
        assert_eq!(builder.http_method(), "POST");
    }
}
