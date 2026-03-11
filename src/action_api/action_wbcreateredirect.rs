use super::{ActionApiData, ActionApiRunnable, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub type NoSource = super::NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbcreateredirectData {
    from: Option<String>,
    to: Option<String>,
    token: Option<String>,
    bot: bool,
}

impl ActionApiData for ActionApiWbcreateredirectData {}

impl ActionApiWbcreateredirectData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbcreateredirect".to_string());
        Self::add_str(&self.from, "from", &mut params);
        Self::add_str(&self.to, "to", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        Self::add_boolean(self.bot, "bot", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiWbcreateredirectBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbcreateredirectData,
}

impl<T> ActionApiWbcreateredirectBuilder<T> {
    pub fn to<S: AsRef<str>>(mut self, to: S) -> Self {
        self.data.to = Some(to.as_ref().to_string());
        self
    }

    pub fn token<S: AsRef<str>>(mut self, token: S) -> Self {
        self.data.token = Some(token.as_ref().to_string());
        self
    }

    pub fn bot(mut self, bot: bool) -> Self {
        self.data.bot = bot;
        self
    }
}

impl ActionApiWbcreateredirectBuilder<NoSource> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbcreateredirectData::default(),
        }
    }

    pub fn from<S: AsRef<str>>(
        mut self,
        from: S,
    ) -> ActionApiWbcreateredirectBuilder<Runnable> {
        self.data.from = Some(from.as_ref().to_string());
        ActionApiWbcreateredirectBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbcreateredirectBuilder<Runnable> {
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

    fn new_builder() -> ActionApiWbcreateredirectBuilder<NoSource> {
        ActionApiWbcreateredirectBuilder::new()
    }

    #[test]
    fn from_set() {
        let params = new_builder().from("Q1").data.params();
        assert_eq!(params["from"], "Q1");
    }

    #[test]
    fn to_set() {
        let params = new_builder().from("Q1").to("Q2").data.params();
        assert_eq!(params["to"], "Q2");
    }

    #[test]
    fn bot_flag() {
        let params = new_builder().from("Q1").bot(true).data.params();
        assert_eq!(params["bot"], "");
    }

    #[test]
    fn token_set() {
        let params = new_builder().from("Q1").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_wbcreateredirect() {
        let params = new_builder().from("Q1").data.params();
        assert_eq!(params["action"], "wbcreateredirect");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().from("Q1");
        assert_eq!(builder.http_method(), "POST");
    }
}
