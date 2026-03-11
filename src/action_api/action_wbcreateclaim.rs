use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbcreateclaimData {
    entity: Option<String>,
    snaktype: Option<String>,
    property: Option<String>,
    value: Option<String>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
    baserevid: Option<u64>,
    bot: bool,
}

impl ActionApiData for ActionApiWbcreateclaimData {}

impl ActionApiWbcreateclaimData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbcreateclaim".to_string());
        Self::add_str(&self.entity, "entity", &mut params);
        Self::add_str(&self.snaktype, "snaktype", &mut params);
        Self::add_str(&self.property, "property", &mut params);
        Self::add_str(&self.value, "value", &mut params);
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
pub struct ActionApiWbcreateclaimBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbcreateclaimData,
}

impl<T> ActionApiWbcreateclaimBuilder<T> {
    pub fn snaktype<S: AsRef<str>>(mut self, snaktype: S) -> Self {
        self.data.snaktype = Some(snaktype.as_ref().to_string());
        self
    }

    pub fn property<S: AsRef<str>>(mut self, property: S) -> Self {
        self.data.property = Some(property.as_ref().to_string());
        self
    }

    pub fn value<S: AsRef<str>>(mut self, value: S) -> Self {
        self.data.value = Some(value.as_ref().to_string());
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

impl ActionApiWbcreateclaimBuilder<NoTarget> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbcreateclaimData::default(),
        }
    }

    pub fn entity<S: AsRef<str>>(mut self, entity: S) -> ActionApiWbcreateclaimBuilder<Runnable> {
        self.data.entity = Some(entity.as_ref().to_string());
        ActionApiWbcreateclaimBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbcreateclaimBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbcreateclaimBuilder<NoTarget> {
        ActionApiWbcreateclaimBuilder::new()
    }

    #[test]
    fn entity_set() {
        let params = new_builder().entity("Q42").data.params();
        assert_eq!(params["entity"], "Q42");
    }

    #[test]
    fn snaktype_set() {
        let params = new_builder().entity("Q42").snaktype("value").data.params();
        assert_eq!(params["snaktype"], "value");
    }

    #[test]
    fn property_set() {
        let params = new_builder().entity("Q42").property("P31").data.params();
        assert_eq!(params["property"], "P31");
    }

    #[test]
    fn value_set() {
        let params = new_builder()
            .entity("Q42")
            .value(r#"{"entity-type":"item","numeric-id":5}"#)
            .data
            .params();
        assert_eq!(params["value"], r#"{"entity-type":"item","numeric-id":5}"#);
    }

    #[test]
    fn token_set() {
        let params = new_builder().entity("Q42").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_wbcreateclaim() {
        let params = new_builder().entity("Q42").data.params();
        assert_eq!(params["action"], "wbcreateclaim");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().entity("Q42");
        assert_eq!(builder.http_method(), "POST");
    }
}
