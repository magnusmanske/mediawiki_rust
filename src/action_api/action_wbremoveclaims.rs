use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoClaims = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbremoveclaimsData {
    claim: Option<Vec<String>>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
    baserevid: Option<u64>,
    bot: bool,
}

impl ActionApiData for ActionApiWbremoveclaimsData {}

impl ActionApiWbremoveclaimsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbremoveclaims".to_string());
        Self::add_vec(&self.claim, "claim", &mut params);
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
pub struct ActionApiWbremoveclaimsBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbremoveclaimsData,
}

impl<T> ActionApiWbremoveclaimsBuilder<T> {
    pub fn summary<S: AsRef<str>>(mut self, summary: S) -> Self {
        self.data.summary = Some(summary.as_ref().to_string());
        self
    }

    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn token<S: AsRef<str>>(mut self, token: S) -> Self {
        self.data.token = Some(token.as_ref().to_string());
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

impl ActionApiWbremoveclaimsBuilder<NoClaims> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbremoveclaimsData::default(),
        }
    }

    pub fn claim<S: Into<String> + Clone>(
        mut self,
        claim: &[S],
    ) -> ActionApiWbremoveclaimsBuilder<Runnable> {
        self.data.claim = Some(claim.iter().map(|s| s.clone().into()).collect());
        ActionApiWbremoveclaimsBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbremoveclaimsBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbremoveclaimsBuilder<NoClaims> {
        ActionApiWbremoveclaimsBuilder::new()
    }

    #[test]
    fn claim_single() {
        let params = new_builder().claim(&["Q42$abc-def"]).data.params();
        assert_eq!(params["claim"], "Q42$abc-def");
    }

    #[test]
    fn claim_multiple() {
        let params = new_builder()
            .claim(&["Q42$abc-def", "Q42$ghi-jkl"])
            .data
            .params();
        assert_eq!(params["claim"], "Q42$abc-def|Q42$ghi-jkl");
    }

    #[test]
    fn token_set() {
        let params = new_builder()
            .claim(&["Q42$abc-def"])
            .token("csrf+\\")
            .data
            .params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_wbremoveclaims() {
        let params = new_builder().claim(&["Q42$abc-def"]).data.params();
        assert_eq!(params["action"], "wbremoveclaims");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().claim(&["Q42$abc-def"]);
        assert_eq!(builder.http_method(), "POST");
    }
}
