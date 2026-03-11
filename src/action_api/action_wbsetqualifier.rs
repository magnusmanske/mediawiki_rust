use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoClaim = NoTitlesOrGenerator;

/// Internal data container for `action=wbsetqualifier` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiWbsetqualifierData {
    claim: Option<String>,
    property: Option<String>,
    snaktype: Option<String>,
    value: Option<String>,
    snakhash: Option<String>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
    baserevid: Option<u64>,
    bot: bool,
}

impl ActionApiData for ActionApiWbsetqualifierData {}

impl ActionApiWbsetqualifierData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbsetqualifier".to_string());
        Self::add_str(&self.claim, "claim", &mut params);
        Self::add_str(&self.property, "property", &mut params);
        Self::add_str(&self.snaktype, "snaktype", &mut params);
        Self::add_str(&self.value, "value", &mut params);
        Self::add_str(&self.snakhash, "snakhash", &mut params);
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

/// Builder for the `action=wbsetqualifier` API action; uses the typestate pattern to enforce required fields.
#[derive(Debug, Clone)]
pub struct ActionApiWbsetqualifierBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbsetqualifierData,
}

impl<T> ActionApiWbsetqualifierBuilder<T> {
    /// Sets the property ID for the qualifier snak. `property`
    pub fn property<S: AsRef<str>>(mut self, property: S) -> Self {
        self.data.property = Some(property.as_ref().to_string());
        self
    }

    /// Sets the type of the qualifier snak (e.g. `value`, `novalue`, `somevalue`). `snaktype`
    pub fn snaktype<S: AsRef<str>>(mut self, snaktype: S) -> Self {
        self.data.snaktype = Some(snaktype.as_ref().to_string());
        self
    }

    /// Sets the serialized data value for the qualifier snak. `value`
    pub fn value<S: AsRef<str>>(mut self, value: S) -> Self {
        self.data.value = Some(value.as_ref().to_string());
        self
    }

    /// Sets the hash of an existing qualifier snak to update. `snakhash`
    pub fn snakhash<S: AsRef<str>>(mut self, snakhash: S) -> Self {
        self.data.snakhash = Some(snakhash.as_ref().to_string());
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
}

impl ActionApiWbsetqualifierBuilder<NoClaim> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbsetqualifierData::default(),
        }
    }

    /// Sets the GUID of the claim to which the qualifier belongs. `claim`
    pub fn claim<S: AsRef<str>>(
        mut self,
        claim: S,
    ) -> ActionApiWbsetqualifierBuilder<NoToken> {
        self.data.claim = Some(claim.as_ref().to_string());
        ActionApiWbsetqualifierBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiWbsetqualifierBuilder<NoToken> {
    /// Sets the CSRF token required to perform the write action. `token`
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiWbsetqualifierBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiWbsetqualifierBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbsetqualifierBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbsetqualifierBuilder<NoClaim> {
        ActionApiWbsetqualifierBuilder::new()
    }

    #[test]
    fn claim_set() {
        let params = new_builder().claim("Q42$abc-def").data.params();
        assert_eq!(params["claim"], "Q42$abc-def");
    }

    #[test]
    fn property_set() {
        let params = new_builder()
            .claim("Q42$abc-def")
            .property("P31")
            .data
            .params();
        assert_eq!(params["property"], "P31");
    }

    #[test]
    fn snaktype_set() {
        let params = new_builder()
            .claim("Q42$abc-def")
            .snaktype("value")
            .data
            .params();
        assert_eq!(params["snaktype"], "value");
    }

    #[test]
    fn snakhash_set() {
        let params = new_builder()
            .claim("Q42$abc-def")
            .snakhash("abc123")
            .data
            .params();
        assert_eq!(params["snakhash"], "abc123");
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
    fn action_is_wbsetqualifier() {
        let params = new_builder().claim("Q42$abc-def").data.params();
        assert_eq!(params["action"], "wbsetqualifier");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().claim("Q42$abc-def").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
