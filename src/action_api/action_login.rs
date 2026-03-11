use super::{ActionApiData, ActionApiRunnable, Runnable};
use std::{collections::HashMap, marker::PhantomData};

/// State: lgtoken not yet set
pub(crate) type NoToken = super::NoTitlesOrGenerator;

/// Internal data container for `action=login` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiLoginData {
    lgname: Option<String>,
    lgpassword: Option<String>,
    lgdomain: Option<String>,
    lgtoken: Option<String>,
}

impl ActionApiData for ActionApiLoginData {}

impl ActionApiLoginData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "login".to_string());
        Self::add_str(&self.lgname, "lgname", &mut params);
        Self::add_str(&self.lgpassword, "lgpassword", &mut params);
        Self::add_str(&self.lgdomain, "lgdomain", &mut params);
        Self::add_str(&self.lgtoken, "lgtoken", &mut params);
        params
    }
}

/// Builder for `action=login`. Call `.lgtoken()` to supply the login token and make it runnable.
#[derive(Debug, Clone)]
pub struct ActionApiLoginBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiLoginData,
}

impl<T> ActionApiLoginBuilder<T> {
    /// Username to log in with (`lgname`).
    pub fn lgname<S: AsRef<str>>(mut self, lgname: S) -> Self {
        self.data.lgname = Some(lgname.as_ref().to_string());
        self
    }

    /// Password to log in with (`lgpassword`).
    pub fn lgpassword<S: AsRef<str>>(mut self, lgpassword: S) -> Self {
        self.data.lgpassword = Some(lgpassword.as_ref().to_string());
        self
    }

    /// Domain for external authentication (`lgdomain`).
    pub fn lgdomain<S: AsRef<str>>(mut self, lgdomain: S) -> Self {
        self.data.lgdomain = Some(lgdomain.as_ref().to_string());
        self
    }
}

impl ActionApiLoginBuilder<NoToken> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiLoginData::default(),
        }
    }

    /// Login token obtained from `action=query&meta=tokens` (`lgtoken`).
    pub fn lgtoken<S: AsRef<str>>(mut self, lgtoken: S) -> ActionApiLoginBuilder<Runnable> {
        self.data.lgtoken = Some(lgtoken.as_ref().to_string());
        ActionApiLoginBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiLoginBuilder<Runnable> {
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

    fn new_builder() -> ActionApiLoginBuilder<NoToken> {
        ActionApiLoginBuilder::new()
    }

    #[test]
    fn default_lgname_absent() {
        let params = new_builder().lgtoken("tok").data.params();
        assert!(!params.contains_key("lgname"));
    }

    #[test]
    fn lgname_set() {
        let params = new_builder().lgname("TestBot").lgtoken("tok").data.params();
        assert_eq!(params["lgname"], "TestBot");
    }

    #[test]
    fn lgpassword_set() {
        let params = new_builder()
            .lgpassword("secret")
            .lgtoken("tok")
            .data
            .params();
        assert_eq!(params["lgpassword"], "secret");
    }

    #[test]
    fn lgtoken_set() {
        let params = new_builder().lgtoken("abc123").data.params();
        assert_eq!(params["lgtoken"], "abc123");
    }

    #[test]
    fn lgdomain_set() {
        let params = new_builder()
            .lgdomain("example.org")
            .lgtoken("tok")
            .data
            .params();
        assert_eq!(params["lgdomain"], "example.org");
    }

    #[test]
    fn action_is_login() {
        let params = new_builder().lgtoken("tok").data.params();
        assert_eq!(params["action"], "login");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().lgtoken("tok");
        assert_eq!(builder.http_method(), "POST");
    }

    #[test]
    fn runnable_params_via_trait() {
        let builder = new_builder()
            .lgname("Bot")
            .lgpassword("pass")
            .lgtoken("tok");
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "login");
        assert_eq!(params["lgname"], "Bot");
        assert_eq!(params["lgpassword"], "pass");
        assert_eq!(params["lgtoken"], "tok");
    }
}
