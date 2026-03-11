use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoClaim = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbremovequalifiersData {
    claim: Option<String>,
    qualifiers: Option<Vec<String>>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
    baserevid: Option<u64>,
    bot: bool,
}

impl ActionApiData for ActionApiWbremovequalifiersData {}

impl ActionApiWbremovequalifiersData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbremovequalifiers".to_string());
        Self::add_str(&self.claim, "claim", &mut params);
        Self::add_vec(&self.qualifiers, "qualifiers", &mut params);
        Self::add_str(&self.summary, "summary", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        if let Some(v) = self.baserevid {
            params.insert("baserevid".to_string(), v.to_string());
        }
        Self::add_boolean(self.bot, "bot", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiWbremovequalifiersBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbremovequalifiersData,
}

impl<T> ActionApiWbremovequalifiersBuilder<T> {
    pub fn qualifiers<S: Into<String> + Clone>(mut self, qualifiers: &[S]) -> Self {
        self.data.qualifiers = Some(qualifiers.iter().map(|s| s.clone().into()).collect());
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

    pub fn baserevid(mut self, baserevid: u64) -> Self {
        self.data.baserevid = Some(baserevid);
        self
    }

    pub fn bot(mut self, bot: bool) -> Self {
        self.data.bot = bot;
        self
    }
}

impl ActionApiWbremovequalifiersBuilder<NoClaim> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbremovequalifiersData::default(),
        }
    }

    pub fn claim<S: AsRef<str>>(
        mut self,
        claim: S,
    ) -> ActionApiWbremovequalifiersBuilder<NoToken> {
        self.data.claim = Some(claim.as_ref().to_string());
        ActionApiWbremovequalifiersBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiWbremovequalifiersBuilder<NoToken> {
    pub fn token<S: AsRef<str>>(
        mut self,
        token: S,
    ) -> ActionApiWbremovequalifiersBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiWbremovequalifiersBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbremovequalifiersBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbremovequalifiersBuilder<NoClaim> {
        ActionApiWbremovequalifiersBuilder::new()
    }

    #[test]
    fn claim_set() {
        let params = new_builder().claim("Q42$abc-def").data.params();
        assert_eq!(params["claim"], "Q42$abc-def");
    }

    #[test]
    fn qualifiers_single() {
        let params = new_builder()
            .claim("Q42$abc-def")
            .qualifiers(&["abc123"])
            .data
            .params();
        assert_eq!(params["qualifiers"], "abc123");
    }

    #[test]
    fn qualifiers_multiple() {
        let params = new_builder()
            .claim("Q42$abc-def")
            .qualifiers(&["abc123", "def456"])
            .data
            .params();
        assert_eq!(params["qualifiers"], "abc123|def456");
    }

    #[test]
    fn token_set() {
        let params = new_builder()
            .claim("Q42$abc-def")
            .token("csrf+\\")
            .data
            .params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_wbremovequalifiers() {
        let params = new_builder().claim("Q42$abc-def").data.params();
        assert_eq!(params["action"], "wbremovequalifiers");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().claim("Q42$abc-def").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
