use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoStatement = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbsetreferenceData {
    statement: Option<String>,
    snaks: Option<String>,
    snaks_order: Option<String>,
    reference: Option<String>,
    index: Option<i64>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
    baserevid: Option<u64>,
    bot: bool,
}

impl ActionApiData for ActionApiWbsetreferenceData {}

impl ActionApiWbsetreferenceData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbsetreference".to_string());
        Self::add_str(&self.statement, "statement", &mut params);
        Self::add_str(&self.snaks, "snaks", &mut params);
        if let Some(ref v) = self.snaks_order {
            params.insert("snaks-order".to_string(), v.clone());
        }
        Self::add_str(&self.reference, "reference", &mut params);
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
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiWbsetreferenceBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbsetreferenceData,
}

impl<T> ActionApiWbsetreferenceBuilder<T> {
    pub fn snaks<S: AsRef<str>>(mut self, snaks: S) -> Self {
        self.data.snaks = Some(snaks.as_ref().to_string());
        self
    }

    pub fn snaks_order<S: AsRef<str>>(mut self, snaks_order: S) -> Self {
        self.data.snaks_order = Some(snaks_order.as_ref().to_string());
        self
    }

    pub fn reference<S: AsRef<str>>(mut self, reference: S) -> Self {
        self.data.reference = Some(reference.as_ref().to_string());
        self
    }

    pub fn index(mut self, index: i64) -> Self {
        self.data.index = Some(index);
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

impl ActionApiWbsetreferenceBuilder<NoStatement> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbsetreferenceData::default(),
        }
    }

    pub fn statement<S: AsRef<str>>(
        mut self,
        statement: S,
    ) -> ActionApiWbsetreferenceBuilder<Runnable> {
        self.data.statement = Some(statement.as_ref().to_string());
        ActionApiWbsetreferenceBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbsetreferenceBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbsetreferenceBuilder<NoStatement> {
        ActionApiWbsetreferenceBuilder::new()
    }

    #[test]
    fn statement_set() {
        let params = new_builder().statement("Q42$abc-def").data.params();
        assert_eq!(params["statement"], "Q42$abc-def");
    }

    #[test]
    fn snaks_set() {
        let params = new_builder()
            .statement("Q42$abc-def")
            .snaks(r#"{"P31":[{"snaktype":"value"}]}"#)
            .data
            .params();
        assert_eq!(params["snaks"], r#"{"P31":[{"snaktype":"value"}]}"#);
    }

    #[test]
    fn reference_set() {
        let params = new_builder()
            .statement("Q42$abc-def")
            .reference("abc123hash")
            .data
            .params();
        assert_eq!(params["reference"], "abc123hash");
    }

    #[test]
    fn snaks_order_param_key() {
        let params = new_builder()
            .statement("Q42$abc-def")
            .snaks_order(r#"["P31"]"#)
            .data
            .params();
        assert_eq!(params["snaks-order"], r#"["P31"]"#);
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
    fn action_is_wbsetreference() {
        let params = new_builder().statement("Q42$abc-def").data.params();
        assert_eq!(params["action"], "wbsetreference");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().statement("Q42$abc-def");
        assert_eq!(builder.http_method(), "POST");
    }
}
