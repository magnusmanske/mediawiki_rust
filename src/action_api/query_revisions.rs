use super::{
    ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData, ActionApiRunnable,
    NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone, Default)]
pub struct ActionApiQueryRevisionsData {
    common: ActionApiQueryCommonData,
    rvprop: Option<Vec<String>>,
    rvslots: Option<Vec<String>>,
    rvlimit: Option<usize>,
    rvsection: Option<String>,
    rvstartid: Option<u64>,
    rvendid: Option<u64>,
    rvstart: Option<String>,
    rvend: Option<String>,
    rvdir: Option<String>,
    rvuser: Option<String>,
    rvexcludeuser: Option<String>,
    rvtag: Option<String>,
    rvcontinue: Option<String>,
}

impl ActionApiData for ActionApiQueryRevisionsData {}

impl ActionApiQueryRevisionsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_vec(&self.rvprop, "rvprop", &mut params);
        Self::add_vec(&self.rvslots, "rvslots", &mut params);
        if let Some(rvlimit) = self.rvlimit {
            params.insert("rvlimit".to_string(), rvlimit.to_string());
        }
        Self::add_str(&self.rvsection, "rvsection", &mut params);
        if let Some(rvstartid) = self.rvstartid {
            params.insert("rvstartid".to_string(), rvstartid.to_string());
        }
        if let Some(rvendid) = self.rvendid {
            params.insert("rvendid".to_string(), rvendid.to_string());
        }
        Self::add_str(&self.rvstart, "rvstart", &mut params);
        Self::add_str(&self.rvend, "rvend", &mut params);
        Self::add_str(&self.rvdir, "rvdir", &mut params);
        Self::add_str(&self.rvuser, "rvuser", &mut params);
        Self::add_str(&self.rvexcludeuser, "rvexcludeuser", &mut params);
        Self::add_str(&self.rvtag, "rvtag", &mut params);
        Self::add_str(&self.rvcontinue, "rvcontinue", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ActionApiQueryRevisionsBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryRevisionsData,
}

impl<T> ActionApiQueryRevisionsBuilder<T> {
    pub fn rvprop<S: Into<String> + Clone>(mut self, rvprop: &[S]) -> Self {
        self.data.rvprop = Some(rvprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn rvslots<S: Into<String> + Clone>(mut self, rvslots: &[S]) -> Self {
        self.data.rvslots = Some(rvslots.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn rvlimit(mut self, rvlimit: usize) -> Self {
        self.data.rvlimit = Some(rvlimit);
        self
    }

    pub fn rvsection<S: AsRef<str>>(mut self, rvsection: S) -> Self {
        self.data.rvsection = Some(rvsection.as_ref().to_string());
        self
    }

    pub fn rvstartid(mut self, rvstartid: u64) -> Self {
        self.data.rvstartid = Some(rvstartid);
        self
    }

    pub fn rvendid(mut self, rvendid: u64) -> Self {
        self.data.rvendid = Some(rvendid);
        self
    }

    pub fn rvstart<S: AsRef<str>>(mut self, rvstart: S) -> Self {
        self.data.rvstart = Some(rvstart.as_ref().to_string());
        self
    }

    pub fn rvend<S: AsRef<str>>(mut self, rvend: S) -> Self {
        self.data.rvend = Some(rvend.as_ref().to_string());
        self
    }

    pub fn rvdir<S: AsRef<str>>(mut self, rvdir: S) -> Self {
        self.data.rvdir = Some(rvdir.as_ref().to_string());
        self
    }

    pub fn rvuser<S: AsRef<str>>(mut self, rvuser: S) -> Self {
        self.data.rvuser = Some(rvuser.as_ref().to_string());
        self
    }

    pub fn rvexcludeuser<S: AsRef<str>>(mut self, rvexcludeuser: S) -> Self {
        self.data.rvexcludeuser = Some(rvexcludeuser.as_ref().to_string());
        self
    }

    pub fn rvtag<S: AsRef<str>>(mut self, rvtag: S) -> Self {
        self.data.rvtag = Some(rvtag.as_ref().to_string());
        self
    }
}

impl ActionApiQueryRevisionsBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryRevisionsData::default(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryRevisionsBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryRevisionsBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryRevisionsBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryRevisionsBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "revisions".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Api,
        action_api::{ActionApiQuery, ActionApiQueryCommonBuilder, NoTitlesOrGenerator},
    };

    fn new_builder() -> ActionApiQueryRevisionsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryRevisionsBuilder::new()
    }

    #[test]
    fn default_rvprop_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("rvprop"));
    }

    #[test]
    fn default_rvlimit_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("rvlimit"));
    }

    #[test]
    fn rvprop_set() {
        let params = new_builder()
            .rvprop(&["ids", "timestamp", "user", "comment"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["rvprop"], "ids|timestamp|user|comment");
    }

    #[test]
    fn rvslots_set() {
        let params = new_builder()
            .rvslots(&["main"])
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["rvslots"], "main");
    }

    #[test]
    fn rvlimit_set() {
        let params = new_builder().rvlimit(10).titles(&["Foo"]).data.params();
        assert_eq!(params["rvlimit"], "10");
    }

    #[test]
    fn rvdir_newer() {
        let params = new_builder().rvdir("newer").titles(&["Foo"]).data.params();
        assert_eq!(params["rvdir"], "newer");
    }

    #[test]
    fn rvuser_set() {
        let params = new_builder()
            .rvuser("ExampleUser")
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["rvuser"], "ExampleUser");
    }

    #[test]
    fn rvexcludeuser_set() {
        let params = new_builder()
            .rvexcludeuser("Bot")
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["rvexcludeuser"], "Bot");
    }

    #[test]
    fn rvstartid_set() {
        let params = new_builder()
            .rvstartid(12345)
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["rvstartid"], "12345");
    }

    #[test]
    fn rvendid_set() {
        let params = new_builder().rvendid(99999).titles(&["Foo"]).data.params();
        assert_eq!(params["rvendid"], "99999");
    }

    #[test]
    fn rvtag_set() {
        let params = new_builder()
            .rvtag("mobile edit")
            .titles(&["Foo"])
            .data
            .params();
        assert_eq!(params["rvtag"], "mobile edit");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "revisions");
    }

    #[tokio::test]
    async fn test_revisions() {
        let api = Api::new("https://en.wikipedia.org/w/api.php")
            .await
            .unwrap();
        let result = ActionApiQuery::revisions()
            .titles(&["Albert Einstein"])
            .rvprop(&["ids", "timestamp", "user"])
            .rvlimit(5)
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
        let page = pages.values().next().unwrap();
        assert!(page["revisions"].is_array());
    }
}
