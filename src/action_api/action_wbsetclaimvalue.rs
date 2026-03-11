use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoClaim = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbsetclaimvalueData {
    claim: Option<String>,
    snaktype: Option<String>,
    value: Option<String>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
    baserevid: Option<u64>,
    bot: bool,
}

impl ActionApiData for ActionApiWbsetclaimvalueData {}

impl ActionApiWbsetclaimvalueData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbsetclaimvalue".to_string());
        Self::add_str(&self.claim, "claim", &mut params);
        Self::add_str(&self.snaktype, "snaktype", &mut params);
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
pub struct ActionApiWbsetclaimvalueBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbsetclaimvalueData,
}

impl<T> ActionApiWbsetclaimvalueBuilder<T> {
    pub fn snaktype<S: AsRef<str>>(mut self, snaktype: S) -> Self {
        self.data.snaktype = Some(snaktype.as_ref().to_string());
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

    pub fn baserevid(mut self, baserevid: u64) -> Self {
        self.data.baserevid = Some(baserevid);
        self
    }

    pub fn bot(mut self, bot: bool) -> Self {
        self.data.bot = bot;
        self
    }
}

impl ActionApiWbsetclaimvalueBuilder<NoClaim> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbsetclaimvalueData::default(),
        }
    }

    pub fn claim<S: AsRef<str>>(
        mut self,
        claim: S,
    ) -> ActionApiWbsetclaimvalueBuilder<NoToken> {
        self.data.claim = Some(claim.as_ref().to_string());
        ActionApiWbsetclaimvalueBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiWbsetclaimvalueBuilder<NoToken> {
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiWbsetclaimvalueBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiWbsetclaimvalueBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbsetclaimvalueBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbsetclaimvalueBuilder<NoClaim> {
        ActionApiWbsetclaimvalueBuilder::new()
    }

    #[test]
    fn claim_set() {
        let params = new_builder().claim("Q42$abc-def").data.params();
        assert_eq!(params["claim"], "Q42$abc-def");
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
    fn value_set() {
        let params = new_builder()
            .claim("Q42$abc-def")
            .value(r#"{"entity-type":"item","numeric-id":5}"#)
            .data
            .params();
        assert_eq!(params["value"], r#"{"entity-type":"item","numeric-id":5}"#);
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
    fn action_is_wbsetclaimvalue() {
        let params = new_builder().claim("Q42$abc-def").data.params();
        assert_eq!(params["action"], "wbsetclaimvalue");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().claim("Q42$abc-def").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
