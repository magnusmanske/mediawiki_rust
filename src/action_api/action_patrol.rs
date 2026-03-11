use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiPatrolData {
    rcid: Option<u64>,
    revid: Option<u64>,
    tags: Option<Vec<String>>,
    token: Option<String>,
}

impl ActionApiData for ActionApiPatrolData {}

impl ActionApiPatrolData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "patrol".to_string());
        if let Some(v) = self.rcid {
            params.insert("rcid".to_string(), v.to_string());
        }
        if let Some(v) = self.revid {
            params.insert("revid".to_string(), v.to_string());
        }
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiPatrolBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiPatrolData,
}

impl<T> ActionApiPatrolBuilder<T> {
    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

}

impl ActionApiPatrolBuilder<NoTarget> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiPatrolData::default(),
        }
    }

    pub fn rcid(mut self, rcid: u64) -> ActionApiPatrolBuilder<NoToken> {
        self.data.rcid = Some(rcid);
        ActionApiPatrolBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    pub fn revid(mut self, revid: u64) -> ActionApiPatrolBuilder<NoToken> {
        self.data.revid = Some(revid);
        ActionApiPatrolBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiPatrolBuilder<NoToken> {
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiPatrolBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiPatrolBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiPatrolBuilder<Runnable> {
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

    fn new_builder() -> ActionApiPatrolBuilder<NoTarget> {
        ActionApiPatrolBuilder::new()
    }

    #[test]
    fn rcid_set() {
        let params = new_builder().rcid(12345).data.params();
        assert_eq!(params["rcid"], "12345");
    }

    #[test]
    fn revid_set() {
        let params = new_builder().revid(67890).data.params();
        assert_eq!(params["revid"], "67890");
    }

    #[test]
    fn token_set() {
        let params = new_builder().rcid(1).token("patrol+\\").data.params();
        assert_eq!(params["token"], "patrol+\\");
    }

    #[test]
    fn action_is_patrol() {
        let params = new_builder().rcid(1).data.params();
        assert_eq!(params["action"], "patrol");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().rcid(1).token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
