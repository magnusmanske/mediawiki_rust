use super::{ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoSource = NoTitlesOrGenerator;

/// Internal data container for `action=mergehistory` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiMergehistoryData {
    from: Option<String>,
    fromid: Option<u64>,
    to: Option<String>,
    toid: Option<u64>,
    timestamp: Option<String>,
    reason: Option<String>,
    starttimestamp: Option<String>,
    token: Option<String>,
}

impl ActionApiData for ActionApiMergehistoryData {}

impl ActionApiMergehistoryData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "mergehistory".to_string());
        Self::add_str(&self.from, "from", &mut params);
        if let Some(v) = self.fromid {
            params.insert("fromid".to_string(), v.to_string());
        }
        Self::add_str(&self.to, "to", &mut params);
        if let Some(v) = self.toid {
            params.insert("toid".to_string(), v.to_string());
        }
        Self::add_str(&self.timestamp, "timestamp", &mut params);
        Self::add_str(&self.reason, "reason", &mut params);
        Self::add_str(&self.starttimestamp, "starttimestamp", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

/// Builder for `action=mergehistory`. Call `.from()` or `.fromid()` to set the source page, then `.token()` to make it runnable.
#[derive(Debug, Clone)]
pub struct ActionApiMergehistoryBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiMergehistoryData,
}

impl<T> ActionApiMergehistoryBuilder<T> {
    /// Title of the destination page (`to`).
    pub fn to<S: AsRef<str>>(mut self, to: S) -> Self {
        self.data.to = Some(to.as_ref().to_string());
        self
    }

    /// Page ID of the destination page (`toid`).
    pub fn toid(mut self, toid: u64) -> Self {
        self.data.toid = Some(toid);
        self
    }

    /// Timestamp up to which history will be merged (`timestamp`).
    pub fn timestamp<S: AsRef<str>>(mut self, timestamp: S) -> Self {
        self.data.timestamp = Some(timestamp.as_ref().to_string());
        self
    }

    /// Reason for the history merge (`reason`).
    pub fn reason<S: AsRef<str>>(mut self, reason: S) -> Self {
        self.data.reason = Some(reason.as_ref().to_string());
        self
    }

    /// Timestamp used to check for edit conflicts (`starttimestamp`).
    pub fn starttimestamp<S: AsRef<str>>(mut self, starttimestamp: S) -> Self {
        self.data.starttimestamp = Some(starttimestamp.as_ref().to_string());
        self
    }

}

impl ActionApiMergehistoryBuilder<NoSource> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiMergehistoryData::default(),
        }
    }

    /// Title of the source page whose history will be merged (`from`).
    pub fn from<S: AsRef<str>>(mut self, from: S) -> ActionApiMergehistoryBuilder<NoToken> {
        self.data.from = Some(from.as_ref().to_string());
        ActionApiMergehistoryBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }

    /// Page ID of the source page whose history will be merged (`fromid`).
    pub fn fromid(mut self, fromid: u64) -> ActionApiMergehistoryBuilder<NoToken> {
        self.data.fromid = Some(fromid);
        ActionApiMergehistoryBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiMergehistoryBuilder<NoToken> {
    /// CSRF token required to perform the merge (`token`).
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiMergehistoryBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiMergehistoryBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiMergehistoryBuilder<Runnable> {
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

    fn new_builder() -> ActionApiMergehistoryBuilder<NoSource> {
        ActionApiMergehistoryBuilder::new()
    }

    #[test]
    fn from_set() {
        let params = new_builder().from("Source Page").data.params();
        assert_eq!(params["from"], "Source Page");
    }

    #[test]
    fn fromid_set() {
        let params = new_builder().fromid(42).data.params();
        assert_eq!(params["fromid"], "42");
    }

    #[test]
    fn to_set() {
        let params = new_builder()
            .from("Source Page")
            .to("Dest Page")
            .data
            .params();
        assert_eq!(params["to"], "Dest Page");
    }

    #[test]
    fn timestamp_set() {
        let params = new_builder()
            .from("Source Page")
            .timestamp("2024-01-01T00:00:00Z")
            .data
            .params();
        assert_eq!(params["timestamp"], "2024-01-01T00:00:00Z");
    }

    #[test]
    fn token_set() {
        let params = new_builder()
            .from("Source Page")
            .token("csrf+\\")
            .data
            .params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_mergehistory() {
        let params = new_builder().from("Source Page").data.params();
        assert_eq!(params["action"], "mergehistory");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().from("Source Page").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
