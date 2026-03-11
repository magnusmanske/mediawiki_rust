use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoClaim = NoTitlesOrGenerator;

/// Internal data container for `action=wbsetclaim` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiWbsetclaimData {
    claim: Option<String>,
    index: Option<i64>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
    baserevid: Option<u64>,
    bot: bool,
    ignoreduplicatemainsnak: bool,
}

impl ActionApiData for ActionApiWbsetclaimData {}

impl ActionApiWbsetclaimData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbsetclaim".to_string());
        Self::add_str(&self.claim, "claim", &mut params);
        if let Some(v) = self.index {
            params.insert("index".to_string(), v.to_string());
        }
        Self::add_str(&self.summary, "summary", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        if let Some(v) = self.baserevid {
            params.insert("baserevid".to_string(), v.to_string());
        }
        Self::add_boolean(self.bot, "bot", &mut params);
        Self::add_boolean(
            self.ignoreduplicatemainsnak,
            "ignoreduplicatemainsnak",
            &mut params,
        );
        params
    }
}

/// Builder for the `action=wbsetclaim` API action; uses the typestate pattern to enforce required fields.
#[derive(Debug, Clone)]
pub struct ActionApiWbsetclaimBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbsetclaimData,
}

impl<T> ActionApiWbsetclaimBuilder<T> {
    /// Sets the zero-based position of the claim within the statement list. `index`
    pub fn index(mut self, index: i64) -> Self {
        self.data.index = Some(index);
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

    /// Sets the base revision ID for conflict detection. `baserevid`
    pub fn baserevid(mut self, baserevid: u64) -> Self {
        self.data.baserevid = Some(baserevid);
        self
    }

    /// Marks the edit as a bot edit. `bot`
    pub fn bot(mut self, bot: bool) -> Self {
        self.data.bot = bot;
        self
    }

    /// When set, silently ignores duplicate main snaks instead of raising an error. `ignoreduplicatemainsnak`
    pub fn ignoreduplicatemainsnak(mut self, ignoreduplicatemainsnak: bool) -> Self {
        self.data.ignoreduplicatemainsnak = ignoreduplicatemainsnak;
        self
    }
}

impl ActionApiWbsetclaimBuilder<NoClaim> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbsetclaimData::default(),
        }
    }

    /// Sets the claim data as a serialized JSON object. `claim`
    pub fn claim<S: AsRef<str>>(mut self, claim: S) -> ActionApiWbsetclaimBuilder<NoToken> {
        self.data.claim = Some(claim.as_ref().to_string());
        ActionApiWbsetclaimBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiWbsetclaimBuilder<NoToken> {
    /// Sets the CSRF token required to perform the write action. `token`
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiWbsetclaimBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiWbsetclaimBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbsetclaimBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbsetclaimBuilder<NoClaim> {
        ActionApiWbsetclaimBuilder::new()
    }

    #[test]
    fn claim_set() {
        let params = new_builder()
            .claim(r#"{"type":"statement","mainsnak":{}}"#)
            .data
            .params();
        assert_eq!(params["claim"], r#"{"type":"statement","mainsnak":{}}"#);
    }

    #[test]
    fn index_set() {
        let params = new_builder()
            .claim(r#"{"type":"statement","mainsnak":{}}"#)
            .index(2)
            .data
            .params();
        assert_eq!(params["index"], "2");
    }

    #[test]
    fn token_set() {
        let params = new_builder()
            .claim(r#"{"type":"statement","mainsnak":{}}"#)
            .token("csrf+\\")
            .data
            .params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_wbsetclaim() {
        let params = new_builder()
            .claim(r#"{"type":"statement","mainsnak":{}}"#)
            .data
            .params();
        assert_eq!(params["action"], "wbsetclaim");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().claim(r#"{"type":"statement","mainsnak":{}}"#).token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
