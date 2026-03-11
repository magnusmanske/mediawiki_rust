use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoFilename = NoTitlesOrGenerator;

/// Internal data container for `action=upload` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiUploadData {
    filename: Option<String>,
    comment: Option<String>,
    tags: Option<Vec<String>>,
    text: Option<String>,
    watchlist: Option<String>,
    watchlistexpiry: Option<String>,
    ignorewarnings: bool,
    url: Option<String>,
    filekey: Option<String>,
    stash: bool,
    filesize: Option<u64>,
    offset: Option<u64>,
    async_upload: bool,
    checkstatus: bool,
    token: Option<String>,
}

impl ActionApiData for ActionApiUploadData {}

impl ActionApiUploadData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "upload".to_string());
        Self::add_str(&self.filename, "filename", &mut params);
        Self::add_str(&self.comment, "comment", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_str(&self.text, "text", &mut params);
        Self::add_str(&self.watchlist, "watchlist", &mut params);
        Self::add_str(&self.watchlistexpiry, "watchlistexpiry", &mut params);
        Self::add_boolean(self.ignorewarnings, "ignorewarnings", &mut params);
        Self::add_str(&self.url, "url", &mut params);
        Self::add_str(&self.filekey, "filekey", &mut params);
        Self::add_boolean(self.stash, "stash", &mut params);
        if let Some(v) = self.filesize {
            params.insert("filesize".to_string(), v.to_string());
        }
        if let Some(v) = self.offset {
            params.insert("offset".to_string(), v.to_string());
        }
        Self::add_boolean(self.async_upload, "async", &mut params);
        Self::add_boolean(self.checkstatus, "checkstatus", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

/// Builder for the `action=upload` API call, using a typestate pattern to enforce required fields before execution.
#[derive(Debug, Clone)]
pub struct ActionApiUploadBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiUploadData,
}

impl<T> ActionApiUploadBuilder<T> {
    /// Sets the upload comment and initial page text description (`comment`).
    pub fn comment<S: AsRef<str>>(mut self, comment: S) -> Self {
        self.data.comment = Some(comment.as_ref().to_string());
        self
    }

    /// Sets the change tags to apply to the upload log entry (`tags`).
    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Sets the initial wikitext for the file description page (`text`).
    pub fn text<S: AsRef<str>>(mut self, text: S) -> Self {
        self.data.text = Some(text.as_ref().to_string());
        self
    }

    /// Sets the watchlist update mode for the uploaded file (`watchlist`).
    pub fn watchlist<S: AsRef<str>>(mut self, watchlist: S) -> Self {
        self.data.watchlist = Some(watchlist.as_ref().to_string());
        self
    }

    /// Sets the expiry timestamp for the watchlist entry (`watchlistexpiry`).
    pub fn watchlistexpiry<S: AsRef<str>>(mut self, watchlistexpiry: S) -> Self {
        self.data.watchlistexpiry = Some(watchlistexpiry.as_ref().to_string());
        self
    }

    /// Sets whether to ignore any upload warnings (`ignorewarnings`).
    pub fn ignorewarnings(mut self, ignorewarnings: bool) -> Self {
        self.data.ignorewarnings = ignorewarnings;
        self
    }

    /// Sets the URL to fetch the file from for URL-based uploads (`url`).
    pub fn url<S: AsRef<str>>(mut self, url: S) -> Self {
        self.data.url = Some(url.as_ref().to_string());
        self
    }

    /// Sets the key identifying a previous stashed upload to complete or check (`filekey`).
    pub fn filekey<S: AsRef<str>>(mut self, filekey: S) -> Self {
        self.data.filekey = Some(filekey.as_ref().to_string());
        self
    }

    /// Sets whether to stash the file temporarily instead of committing it (`stash`).
    pub fn stash(mut self, stash: bool) -> Self {
        self.data.stash = stash;
        self
    }

    /// Sets the total size of the file for chunked uploads (`filesize`).
    pub fn filesize(mut self, filesize: u64) -> Self {
        self.data.filesize = Some(filesize);
        self
    }

    /// Sets the byte offset of this chunk within the file for chunked uploads (`offset`).
    pub fn offset(mut self, offset: u64) -> Self {
        self.data.offset = Some(offset);
        self
    }

    /// Sets whether to process the upload asynchronously when possible (`async`).
    pub fn async_upload(mut self, async_upload: bool) -> Self {
        self.data.async_upload = async_upload;
        self
    }

    /// Sets whether to only fetch the upload status for an asynchronous upload (`checkstatus`).
    pub fn checkstatus(mut self, checkstatus: bool) -> Self {
        self.data.checkstatus = checkstatus;
        self
    }

}

impl ActionApiUploadBuilder<NoFilename> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiUploadData::default(),
        }
    }

    /// Sets the target filename on the wiki (`filename`).
    pub fn filename<S: AsRef<str>>(mut self, filename: S) -> ActionApiUploadBuilder<NoToken> {
        self.data.filename = Some(filename.as_ref().to_string());
        ActionApiUploadBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiUploadBuilder<NoToken> {
    /// Sets the CSRF token (`token`).
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiUploadBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiUploadBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiUploadBuilder<Runnable> {
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

    fn new_builder() -> ActionApiUploadBuilder<NoFilename> {
        ActionApiUploadBuilder::new()
    }

    #[test]
    fn filename_set() {
        let params = new_builder().filename("Example.png").data.params();
        assert_eq!(params["filename"], "Example.png");
    }

    #[test]
    fn url_set() {
        let params = new_builder()
            .filename("Example.png")
            .url("https://example.com/image.png")
            .data
            .params();
        assert_eq!(params["url"], "https://example.com/image.png");
    }

    #[test]
    fn comment_set() {
        let params = new_builder()
            .filename("Example.png")
            .comment("Uploaded via API")
            .data
            .params();
        assert_eq!(params["comment"], "Uploaded via API");
    }

    #[test]
    fn ignorewarnings_flag() {
        let params = new_builder()
            .filename("Example.png")
            .ignorewarnings(true)
            .data
            .params();
        assert_eq!(params["ignorewarnings"], "");
    }

    #[test]
    fn ignorewarnings_false_absent() {
        let params = new_builder().filename("Example.png").data.params();
        assert!(!params.contains_key("ignorewarnings"));
    }

    #[test]
    fn token_set() {
        let params = new_builder()
            .filename("Example.png")
            .token("csrf+\\")
            .data
            .params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_upload() {
        let params = new_builder().filename("Example.png").data.params();
        assert_eq!(params["action"], "upload");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().filename("Example.png").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
