use super::{ActionApiData, ActionApiRunnable, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub(crate) type NoType = super::NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiChecktokenData {
    token_type: Option<String>,
    token: Option<String>,
    maxtokenage: Option<u64>,
}

impl ActionApiData for ActionApiChecktokenData {}

impl ActionApiChecktokenData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "checktoken".to_string());
        if let Some(t) = &self.token_type {
            params.insert("type".to_string(), t.clone());
        }
        Self::add_str(&self.token, "token", &mut params);
        if let Some(age) = self.maxtokenage {
            params.insert("maxtokenage".to_string(), age.to_string());
        }
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiChecktokenBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiChecktokenData,
}

impl<T> ActionApiChecktokenBuilder<T> {
    pub fn token<S: AsRef<str>>(mut self, token: S) -> Self {
        self.data.token = Some(token.as_ref().to_string());
        self
    }

    pub fn maxtokenage(mut self, maxtokenage: u64) -> Self {
        self.data.maxtokenage = Some(maxtokenage);
        self
    }
}

impl ActionApiChecktokenBuilder<NoType> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiChecktokenData::default(),
        }
    }

    pub fn token_type<S: AsRef<str>>(
        mut self,
        token_type: S,
    ) -> ActionApiChecktokenBuilder<Runnable> {
        self.data.token_type = Some(token_type.as_ref().to_string());
        ActionApiChecktokenBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiChecktokenBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        self.data.params()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiChecktokenBuilder<NoType> {
        ActionApiChecktokenBuilder::new()
    }

    #[test]
    fn token_type_set() {
        let params = new_builder().token_type("csrf").data.params();
        assert_eq!(params["type"], "csrf");
    }

    #[test]
    fn token_set() {
        let params = new_builder()
            .token("abc+\\")
            .token_type("csrf")
            .data
            .params();
        assert_eq!(params["token"], "abc+\\");
    }

    #[test]
    fn maxtokenage_set() {
        let params = new_builder()
            .maxtokenage(3600)
            .token_type("csrf")
            .data
            .params();
        assert_eq!(params["maxtokenage"], "3600");
    }

    #[test]
    fn default_maxtokenage_absent() {
        let params = new_builder().token_type("csrf").data.params();
        assert!(!params.contains_key("maxtokenage"));
    }

    #[test]
    fn action_is_checktoken() {
        let params = new_builder().token_type("csrf").data.params();
        assert_eq!(params["action"], "checktoken");
    }
}
