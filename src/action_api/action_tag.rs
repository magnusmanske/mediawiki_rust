use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub(crate) type NoTagTarget = NoTitlesOrGenerator;
pub(crate) type NoTagToken = NoToken;

/// Internal data container for `action=tag` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiTagData {
    rcids: Option<Vec<u64>>,
    revids: Option<Vec<u64>>,
    logids: Option<Vec<u64>>,
    fileid: Option<u64>,
    add: Option<Vec<String>>,
    remove: Option<Vec<String>>,
    reason: Option<String>,
    tags: Option<Vec<String>>,
    token: Option<String>,
}

impl ActionApiData for ActionApiTagData {}

impl ActionApiTagData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "tag".to_string());
        if let Some(ref ids) = self.rcids {
            let s: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
            params.insert("rcids".to_string(), s.join("|"));
        }
        if let Some(ref ids) = self.revids {
            let s: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
            params.insert("revids".to_string(), s.join("|"));
        }
        if let Some(ref ids) = self.logids {
            let s: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
            params.insert("logids".to_string(), s.join("|"));
        }
        if let Some(id) = self.fileid {
            params.insert("fileid".to_string(), id.to_string());
        }
        Self::add_vec(&self.add, "add", &mut params);
        Self::add_vec(&self.remove, "remove", &mut params);
        Self::add_str(&self.reason, "reason", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

/// Builder for `action=tag`. Call one of `rcids()`, `revids()`, `logids()`, or `fileid()` to set
/// the target, then call `.token()` to make it runnable.
#[derive(Debug, Clone)]
pub struct ActionApiTagBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiTagData,
}

impl<T> ActionApiTagBuilder<T> {
    /// Tags to add (`add`).
    pub fn add<S: Into<String> + Clone>(mut self, add: &[S]) -> Self {
        self.data.add = Some(add.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Tags to remove (`remove`).
    pub fn remove<S: Into<String> + Clone>(mut self, remove: &[S]) -> Self {
        self.data.remove = Some(remove.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Reason for the tag change (`reason`).
    pub fn reason<S: AsRef<str>>(mut self, reason: S) -> Self {
        self.data.reason = Some(reason.as_ref().to_string());
        self
    }

    /// Tags to apply to the log entry (`tags`).
    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }
}

impl ActionApiTagBuilder<NoTagTarget> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiTagData::default(),
        }
    }

    /// Recent change IDs to tag (`rcids`).
    pub fn rcids(mut self, rcids: &[u64]) -> ActionApiTagBuilder<NoTagToken> {
        self.data.rcids = Some(rcids.to_vec());
        ActionApiTagBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    /// Revision IDs to tag (`revids`).
    pub fn revids(mut self, revids: &[u64]) -> ActionApiTagBuilder<NoTagToken> {
        self.data.revids = Some(revids.to_vec());
        ActionApiTagBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    /// Log entry IDs to tag (`logids`).
    pub fn logids(mut self, logids: &[u64]) -> ActionApiTagBuilder<NoTagToken> {
        self.data.logids = Some(logids.to_vec());
        ActionApiTagBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    /// File ID to tag (`fileid`).
    pub fn fileid(mut self, fileid: u64) -> ActionApiTagBuilder<NoTagToken> {
        self.data.fileid = Some(fileid);
        ActionApiTagBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiTagBuilder<NoTagToken> {
    /// CSRF token required to perform the tag action (`token`).
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiTagBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiTagBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiTagBuilder<Runnable> {
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

    fn new_builder() -> ActionApiTagBuilder<NoTagTarget> {
        ActionApiTagBuilder::new()
    }

    #[test]
    fn rcids_set() {
        let params = new_builder().rcids(&[1, 2, 3]).data.params();
        assert_eq!(params["rcids"], "1|2|3");
    }

    #[test]
    fn revids_set() {
        let params = new_builder().revids(&[100, 200]).data.params();
        assert_eq!(params["revids"], "100|200");
    }

    #[test]
    fn logids_set() {
        let params = new_builder().logids(&[10]).data.params();
        assert_eq!(params["logids"], "10");
    }

    #[test]
    fn fileid_set() {
        let params = new_builder().fileid(42).data.params();
        assert_eq!(params["fileid"], "42");
    }

    #[test]
    fn add_set() {
        let params = new_builder().rcids(&[1]).add(&["tag1", "tag2"]).data.params();
        assert_eq!(params["add"], "tag1|tag2");
    }

    #[test]
    fn remove_set() {
        let params = new_builder().rcids(&[1]).remove(&["tag1"]).data.params();
        assert_eq!(params["remove"], "tag1");
    }

    #[test]
    fn reason_set() {
        let params = new_builder().rcids(&[1]).reason("Test reason").data.params();
        assert_eq!(params["reason"], "Test reason");
    }

    #[test]
    fn token_set() {
        let params = new_builder().rcids(&[1]).token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_tag() {
        let params = new_builder().rcids(&[1]).data.params();
        assert_eq!(params["action"], "tag");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().rcids(&[1]).token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
