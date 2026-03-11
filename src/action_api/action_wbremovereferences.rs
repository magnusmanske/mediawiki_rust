use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoStatement = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbremovereferencesData {
    statement: Option<String>,
    references: Option<Vec<String>>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
    baserevid: Option<u64>,
    bot: bool,
}

impl ActionApiData for ActionApiWbremovereferencesData {}

impl ActionApiWbremovereferencesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbremovereferences".to_string());
        Self::add_str(&self.statement, "statement", &mut params);
        Self::add_vec(&self.references, "references", &mut params);
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
pub struct ActionApiWbremovereferencesBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbremovereferencesData,
}

impl<T> ActionApiWbremovereferencesBuilder<T> {
    pub fn references<S: Into<String> + Clone>(mut self, references: &[S]) -> Self {
        self.data.references = Some(references.iter().map(|s| s.clone().into()).collect());
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

impl ActionApiWbremovereferencesBuilder<NoStatement> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbremovereferencesData::default(),
        }
    }

    pub fn statement<S: AsRef<str>>(
        mut self,
        statement: S,
    ) -> ActionApiWbremovereferencesBuilder<NoToken> {
        self.data.statement = Some(statement.as_ref().to_string());
        ActionApiWbremovereferencesBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiWbremovereferencesBuilder<NoToken> {
    pub fn token<S: AsRef<str>>(
        mut self,
        token: S,
    ) -> ActionApiWbremovereferencesBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiWbremovereferencesBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbremovereferencesBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbremovereferencesBuilder<NoStatement> {
        ActionApiWbremovereferencesBuilder::new()
    }

    #[test]
    fn statement_set() {
        let params = new_builder().statement("Q42$abc-def").data.params();
        assert_eq!(params["statement"], "Q42$abc-def");
    }

    #[test]
    fn references_set() {
        let params = new_builder()
            .statement("Q42$abc-def")
            .references(&["hash1", "hash2"])
            .data
            .params();
        assert_eq!(params["references"], "hash1|hash2");
    }

    #[test]
    fn token_set() {
        let params = new_builder()
            .statement("Q42$abc-def")
            .token("csrf+\\")
            .data
            .params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_wbremovereferences() {
        let params = new_builder().statement("Q42$abc-def").data.params();
        assert_eq!(params["action"], "wbremovereferences");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().statement("Q42$abc-def").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
