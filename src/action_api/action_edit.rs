use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

/// Internal data container for `action=edit` parameters.
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

/// Builder for `action=edit` — creates or modifies a page.
///
/// # Typestate
/// - Start: [`NoTitlesOrGenerator`] — call `.title()` or `.pageid()` to set the target page.
/// - After target: [`NoToken`] — call `.token(csrf_token)` to advance to [`Runnable`].
///
/// # Example
/// ```rust
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use mediawiki::prelude::*;
/// let mut api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
/// let token = api.get_edit_token().await.unwrap();
/// let _result = ActionApi::edit()
///     .title("Sandbox")
///     .text("Hello world")
///     .summary("test edit")
///     .token(&token)
///     .run(&api)
///     .await;
/// # });
/// ```
#[derive(Debug, Clone)]
pub struct ActionApiEditBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiEditData,
}

impl<T> ActionApiEditBuilder<T> {
    /// Sets the section number to edit, or `"new"` to add a new section.
    pub fn section<S: AsRef<str>>(mut self, section: S) -> Self {
        self.data.section = Some(section.as_ref().to_string());
        self
    }

    /// Title of the new section when adding a new section (`sectiontitle`).
    pub fn sectiontitle<S: AsRef<str>>(mut self, sectiontitle: S) -> Self {
        self.data.sectiontitle = Some(sectiontitle.as_ref().to_string());
        self
    }

    /// Full page content to replace the current wikitext (`text`).
    pub fn text<S: AsRef<str>>(mut self, text: S) -> Self {
        self.data.text = Some(text.as_ref().to_string());
        self
    }

    /// Edit summary (`summary`).
    pub fn summary<S: AsRef<str>>(mut self, summary: S) -> Self {
        self.data.summary = Some(summary.as_ref().to_string());
        self
    }

    /// Change tags to apply to the edit (`tags`).
    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Mark the edit as minor (`minor`).
    pub fn minor(mut self, minor: bool) -> Self {
        self.data.minor = minor;
        self
    }

    /// Mark the edit as non-minor, overriding user preferences (`notminor`).
    pub fn notminor(mut self, notminor: bool) -> Self {
        self.data.notminor = notminor;
        self
    }

    /// Mark the edit as a bot edit (`bot`).
    pub fn bot(mut self, bot: bool) -> Self {
        self.data.bot = bot;
        self
    }

    /// Revision ID of the base revision to detect edit conflicts (`baserevid`).
    pub fn baserevid(mut self, baserevid: u64) -> Self {
        self.data.baserevid = Some(baserevid);
        self
    }

    /// Timestamp of the base revision to detect edit conflicts (`basetimestamp`).
    pub fn basetimestamp<S: AsRef<str>>(mut self, basetimestamp: S) -> Self {
        self.data.basetimestamp = Some(basetimestamp.as_ref().to_string());
        self
    }

    /// Timestamp when the editing session began, used for lost-edit detection (`starttimestamp`).
    pub fn starttimestamp<S: AsRef<str>>(mut self, starttimestamp: S) -> Self {
        self.data.starttimestamp = Some(starttimestamp.as_ref().to_string());
        self
    }

    /// Override any errors about the article having been deleted (`recreate`).
    pub fn recreate(mut self, recreate: bool) -> Self {
        self.data.recreate = recreate;
        self
    }

    /// Only create the page; return an error if it already exists (`createonly`).
    pub fn createonly(mut self, createonly: bool) -> Self {
        self.data.createonly = createonly;
        self
    }

    /// Return an error if the page doesn't exist (`nocreate`).
    pub fn nocreate(mut self, nocreate: bool) -> Self {
        self.data.nocreate = nocreate;
        self
    }

    /// Watchlist handling: `"watch"`, `"unwatch"`, `"preferences"`, or `"nochange"` (`watchlist`).
    pub fn watchlist<S: AsRef<str>>(mut self, watchlist: S) -> Self {
        self.data.watchlist = Some(watchlist.as_ref().to_string());
        self
    }

    /// Watchlist expiry timestamp (`watchlistexpiry`).
    pub fn watchlistexpiry<S: AsRef<str>>(mut self, watchlistexpiry: S) -> Self {
        self.data.watchlistexpiry = Some(watchlistexpiry.as_ref().to_string());
        self
    }

    /// MD5 hash of the `text` parameter, used for integrity verification (`md5`).
    pub fn md5<S: AsRef<str>>(mut self, md5: S) -> Self {
        self.data.md5 = Some(md5.as_ref().to_string());
        self
    }

    /// Text to prepend to the page content (`prependtext`).
    pub fn prependtext<S: AsRef<str>>(mut self, prependtext: S) -> Self {
        self.data.prependtext = Some(prependtext.as_ref().to_string());
        self
    }

    /// Text to append to the page content (`appendtext`).
    pub fn appendtext<S: AsRef<str>>(mut self, appendtext: S) -> Self {
        self.data.appendtext = Some(appendtext.as_ref().to_string());
        self
    }

    /// Revision ID to undo (`undo`).
    pub fn undo(mut self, undo: u64) -> Self {
        self.data.undo = Some(undo);
        self
    }

    /// Revision ID to stop undoing at (`undoafter`).
    pub fn undoafter(mut self, undoafter: u64) -> Self {
        self.data.undoafter = Some(undoafter);
        self
    }

    /// Whether to follow redirects (`redirect`).
    pub fn redirect(mut self, redirect: bool) -> Self {
        self.data.redirect = redirect;
        self
    }

    /// Content serialisation format, e.g. `"text/x-wiki"` (`contentformat`).
    pub fn contentformat<S: AsRef<str>>(mut self, contentformat: S) -> Self {
        self.data.contentformat = Some(contentformat.as_ref().to_string());
        self
    }

    /// Content model, e.g. `"wikitext"` or `"json"` (`contentmodel`).
    pub fn contentmodel<S: AsRef<str>>(mut self, contentmodel: S) -> Self {
        self.data.contentmodel = Some(contentmodel.as_ref().to_string());
        self
    }

}

impl ActionApiEditBuilder<NoTarget> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiEditData::default(),
        }
    }

    /// Sets the title of the page to edit, advancing to the [`NoToken`] state.
    pub fn title<S: AsRef<str>>(mut self, title: S) -> ActionApiEditBuilder<NoToken> {
        self.data.title = Some(title.as_ref().to_string());
        ActionApiEditBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    /// Sets the page ID of the page to edit, advancing to the [`NoToken`] state.
    pub fn pageid(mut self, pageid: u64) -> ActionApiEditBuilder<NoToken> {
        self.data.pageid = Some(pageid);
        ActionApiEditBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiEditBuilder<NoToken> {
    /// Sets the CSRF token, advancing to the [`Runnable`] state.
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiEditBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
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
        let builder = new_builder().title("Foo").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
