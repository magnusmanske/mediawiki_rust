use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `list=blocks` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListBlocksData {
    bkstart: Option<String>,
    bkend: Option<String>,
    bkdir: Option<String>,
    bkids: Option<Vec<u64>>,
    bkusers: Option<Vec<String>>,
    bkip: Option<String>,
    bklimit: usize,
    bkprop: Option<Vec<String>>,
    bkshow: Option<Vec<String>>,
    bkcontinue: Option<String>,
}

impl ActionApiData for ActionApiListBlocksData {}

impl Default for ActionApiListBlocksData {
    fn default() -> Self {
        Self {
            bkstart: None,
            bkend: None,
            bkdir: None,
            bkids: None,
            bkusers: None,
            bkip: None,
            bklimit: 10,
            bkprop: None,
            bkshow: None,
            bkcontinue: None,
        }
    }
}

impl ActionApiListBlocksData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.bkstart, "bkstart", &mut params);
        Self::add_str(&self.bkend, "bkend", &mut params);
        Self::add_str(&self.bkdir, "bkdir", &mut params);
        if let Some(ref ids) = self.bkids {
            let s: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
            params.insert("bkids".to_string(), s.join("|"));
        }
        Self::add_vec(&self.bkusers, "bkusers", &mut params);
        Self::add_str(&self.bkip, "bkip", &mut params);
        params.insert("bklimit".to_string(), self.bklimit.to_string());
        Self::add_vec(&self.bkprop, "bkprop", &mut params);
        Self::add_vec(&self.bkshow, "bkshow", &mut params);
        Self::add_str(&self.bkcontinue, "bkcontinue", &mut params);
        params
    }
}

/// Builder for `list=blocks` — lists all blocked users and IP addresses.
#[derive(Debug, Clone)]
pub struct ActionApiListBlocksBuilder {
    pub(crate) data: ActionApiListBlocksData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListBlocksBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListBlocksData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Start listing from this timestamp (`bkstart`).
    pub fn bkstart<S: AsRef<str>>(mut self, bkstart: S) -> Self {
        self.data.bkstart = Some(bkstart.as_ref().to_string());
        self
    }

    /// Stop listing at this timestamp (`bkend`).
    pub fn bkend<S: AsRef<str>>(mut self, bkend: S) -> Self {
        self.data.bkend = Some(bkend.as_ref().to_string());
        self
    }

    /// Direction to enumerate (`bkdir`).
    pub fn bkdir<S: AsRef<str>>(mut self, bkdir: S) -> Self {
        self.data.bkdir = Some(bkdir.as_ref().to_string());
        self
    }

    /// Filter blocks by these block IDs (`bkids`).
    pub fn bkids(mut self, bkids: &[u64]) -> Self {
        self.data.bkids = Some(bkids.to_vec());
        self
    }

    /// Filter blocks to these users (`bkusers`).
    pub fn bkusers<S: Into<String> + Clone>(mut self, bkusers: &[S]) -> Self {
        self.data.bkusers = Some(bkusers.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Filter blocks that cover this IP address or range (`bkip`).
    pub fn bkip<S: AsRef<str>>(mut self, bkip: S) -> Self {
        self.data.bkip = Some(bkip.as_ref().to_string());
        self
    }

    /// Maximum number of blocks to return (`bklimit`).
    pub fn bklimit(mut self, bklimit: usize) -> Self {
        self.data.bklimit = bklimit;
        self
    }

    /// Properties to return for each block (`bkprop`).
    pub fn bkprop<S: Into<String> + Clone>(mut self, bkprop: &[S]) -> Self {
        self.data.bkprop = Some(bkprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Filter blocks by type (`bkshow`).
    pub fn bkshow<S: Into<String> + Clone>(mut self, bkshow: &[S]) -> Self {
        self.data.bkshow = Some(bkshow.iter().map(|s| s.clone().into()).collect());
        self
    }
}

impl ActionApiRunnable for ActionApiListBlocksBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "blocks".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListBlocksBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListBlocksBuilder {
        ActionApiListBlocksBuilder::new()
    }

    #[test]
    fn default_bklimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["bklimit"], "10");
    }

    #[test]
    fn default_bkstart_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("bkstart"));
    }

    #[test]
    fn bkusers_set() {
        let params = new_builder().bkusers(&["SomeUser"]).data.params();
        assert_eq!(params["bkusers"], "SomeUser");
    }

    #[test]
    fn bkids_set() {
        let params = new_builder().bkids(&[1, 2, 3]).data.params();
        assert_eq!(params["bkids"], "1|2|3");
    }

    #[test]
    fn bkip_set() {
        let params = new_builder().bkip("1.2.3.4").data.params();
        assert_eq!(params["bkip"], "1.2.3.4");
    }

    #[test]
    fn bklimit_set() {
        let params = new_builder().bklimit(50).data.params();
        assert_eq!(params["bklimit"], "50");
    }

    #[test]
    fn bkprop_set() {
        let params = new_builder().bkprop(&["id", "user", "reason"]).data.params();
        assert_eq!(params["bkprop"], "id|user|reason");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "blocks");
    }
}
