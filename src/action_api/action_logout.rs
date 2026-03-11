use super::{ActionApiData, ActionApiRunnable, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub(crate) type NoToken = super::NoTitlesOrGenerator;

/// Internal data container for `action=logout` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiLogoutData {
    token: Option<String>,
}

impl ActionApiData for ActionApiLogoutData {}

impl ActionApiLogoutData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "logout".to_string());
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

/// Builder for `action=logout`. Call `.token()` to supply the CSRF token and make it runnable.
#[derive(Debug, Clone)]
pub struct ActionApiLogoutBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiLogoutData,
}

impl ActionApiLogoutBuilder<NoToken> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiLogoutData::default(),
        }
    }

    /// CSRF token required to perform the logout (`token`).
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiLogoutBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiLogoutBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiLogoutBuilder<Runnable> {
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

    #[test]
    fn token_set() {
        let params = ActionApiLogoutBuilder::new().token("csrf123").data.params();
        assert_eq!(params["token"], "csrf123");
    }

    #[test]
    fn action_is_logout() {
        let params = ActionApiLogoutBuilder::new().token("tok").data.params();
        assert_eq!(params["action"], "logout");
    }

    #[test]
    fn http_method_is_post() {
        let builder = ActionApiLogoutBuilder::new().token("tok");
        assert_eq!(builder.http_method(), "POST");
    }
}
