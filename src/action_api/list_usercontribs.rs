use super::{
    ActionApiContinuable, ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `list=usercontribs` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListUsercontribsData {
    ucuser: Option<Vec<String>>,
    ucuserids: Option<Vec<u64>>,
    ucuserprefix: Option<String>,
    uciprange: Option<String>,
    uclimit: usize,
    ucstart: Option<String>,
    ucend: Option<String>,
    uccontinue: Option<String>,
    ucdir: Option<String>,
    ucnamespace: Option<Vec<NamespaceID>>,
    ucprop: Option<Vec<String>>,
    ucshow: Option<Vec<String>>,
    uctag: Option<String>,
}

impl ActionApiData for ActionApiListUsercontribsData {}

impl Default for ActionApiListUsercontribsData {
    fn default() -> Self {
        Self {
            ucuser: None,
            ucuserids: None,
            ucuserprefix: None,
            uciprange: None,
            uclimit: 10,
            ucstart: None,
            ucend: None,
            uccontinue: None,
            ucdir: None,
            ucnamespace: None,
            ucprop: None,
            ucshow: None,
            uctag: None,
        }
    }
}

impl ActionApiListUsercontribsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_vec(&self.ucuser, "ucuser", &mut params);
        if let Some(ucuserids) = &self.ucuserids {
            let s: Vec<String> = ucuserids.iter().map(|id| id.to_string()).collect();
            params.insert("ucuserids".to_string(), s.join("|"));
        }
        Self::add_str(&self.ucuserprefix, "ucuserprefix", &mut params);
        Self::add_str(&self.uciprange, "uciprange", &mut params);
        params.insert("uclimit".to_string(), self.uclimit.to_string());
        Self::add_str(&self.ucstart, "ucstart", &mut params);
        Self::add_str(&self.ucend, "ucend", &mut params);
        Self::add_str(&self.uccontinue, "uccontinue", &mut params);
        Self::add_str(&self.ucdir, "ucdir", &mut params);
        if let Some(ns) = &self.ucnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("ucnamespace".to_string(), s.join("|"));
        }
        Self::add_vec(&self.ucprop, "ucprop", &mut params);
        Self::add_vec(&self.ucshow, "ucshow", &mut params);
        Self::add_str(&self.uctag, "uctag", &mut params);
        params
    }
}

/// Builder for the `list=usercontribs` API module; supports pagination via `ActionApiContinuable`.
#[derive(Debug, Clone)]
pub struct ActionApiListUsercontribsBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiListUsercontribsData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiListUsercontribsBuilder<T> {
    /// Maximum number of contributions to return (`uclimit`).
    pub fn uclimit(mut self, uclimit: usize) -> Self {
        self.data.uclimit = uclimit;
        self
    }

    /// Timestamp to start enumerating contributions from (`ucstart`).
    pub fn ucstart<S: AsRef<str>>(mut self, ucstart: S) -> Self {
        self.data.ucstart = Some(ucstart.as_ref().to_string());
        self
    }

    /// Timestamp to stop enumerating contributions at (`ucend`).
    pub fn ucend<S: AsRef<str>>(mut self, ucend: S) -> Self {
        self.data.ucend = Some(ucend.as_ref().to_string());
        self
    }

    /// Enumeration direction (`newer` or `older`) (`ucdir`).
    pub fn ucdir<S: AsRef<str>>(mut self, ucdir: S) -> Self {
        self.data.ucdir = Some(ucdir.as_ref().to_string());
        self
    }

    /// Filter contributions to these namespaces (`ucnamespace`).
    pub fn ucnamespace(mut self, ucnamespace: &[NamespaceID]) -> Self {
        self.data.ucnamespace = Some(ucnamespace.to_vec());
        self
    }

    /// Properties to retrieve for each contribution (`ucprop`).
    pub fn ucprop<S: Into<String> + Clone>(mut self, ucprop: &[S]) -> Self {
        self.data.ucprop = Some(ucprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Show only contributions matching these criteria (e.g. `minor`, `top`) (`ucshow`).
    pub fn ucshow<S: Into<String> + Clone>(mut self, ucshow: &[S]) -> Self {
        self.data.ucshow = Some(ucshow.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Filter contributions to those tagged with this tag (`uctag`).
    pub fn uctag<S: AsRef<str>>(mut self, uctag: S) -> Self {
        self.data.uctag = Some(uctag.as_ref().to_string());
        self
    }
}

impl ActionApiListUsercontribsBuilder<NoTitlesOrGenerator> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiListUsercontribsData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// One or more usernames whose contributions to retrieve (`ucuser`).
    pub fn ucuser<S: Into<String> + Clone>(
        mut self,
        ucuser: &[S],
    ) -> ActionApiListUsercontribsBuilder<Runnable> {
        self.data.ucuser = Some(ucuser.iter().map(|s| s.clone().into()).collect());
        ActionApiListUsercontribsBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }

    /// One or more user IDs whose contributions to retrieve (`ucuserids`).
    pub fn ucuserids(mut self, ucuserids: &[u64]) -> ActionApiListUsercontribsBuilder<Runnable> {
        self.data.ucuserids = Some(ucuserids.to_vec());
        ActionApiListUsercontribsBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }

    /// Retrieve contributions for all users whose name starts with this prefix (`ucuserprefix`).
    pub fn ucuserprefix<S: AsRef<str>>(
        mut self,
        ucuserprefix: S,
    ) -> ActionApiListUsercontribsBuilder<Runnable> {
        self.data.ucuserprefix = Some(ucuserprefix.as_ref().to_string());
        ActionApiListUsercontribsBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }

    /// Retrieve contributions from all IP addresses in this CIDR range (`uciprange`).
    pub fn uciprange<S: AsRef<str>>(
        mut self,
        uciprange: S,
    ) -> ActionApiListUsercontribsBuilder<Runnable> {
        self.data.uciprange = Some(uciprange.as_ref().to_string());
        ActionApiListUsercontribsBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiRunnable for ActionApiListUsercontribsBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "usercontribs".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListUsercontribsBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiList};

    fn new_builder() -> ActionApiListUsercontribsBuilder<NoTitlesOrGenerator> {
        ActionApiListUsercontribsBuilder::new()
    }

    #[test]
    fn default_uclimit_is_10() {
        let params = new_builder().ucuser(&["ExampleUser"]).data.params();
        assert_eq!(params["uclimit"], "10");
    }

    #[test]
    fn ucuser_single() {
        let params = new_builder().ucuser(&["ExampleUser"]).data.params();
        assert_eq!(params["ucuser"], "ExampleUser");
    }

    #[test]
    fn ucuser_multiple() {
        let params = new_builder().ucuser(&["Alice", "Bob"]).data.params();
        assert_eq!(params["ucuser"], "Alice|Bob");
    }

    #[test]
    fn ucuserids_set() {
        let params = new_builder().ucuserids(&[123, 456]).data.params();
        assert_eq!(params["ucuserids"], "123|456");
    }

    #[test]
    fn ucuserprefix_set() {
        let params = new_builder().ucuserprefix("Albert").data.params();
        assert_eq!(params["ucuserprefix"], "Albert");
    }

    #[test]
    fn uciprange_set() {
        let params = new_builder().uciprange("192.0.2.0/24").data.params();
        assert_eq!(params["uciprange"], "192.0.2.0/24");
    }

    #[test]
    fn uclimit_set() {
        let params = new_builder().uclimit(50).ucuser(&["Foo"]).data.params();
        assert_eq!(params["uclimit"], "50");
    }

    #[test]
    fn ucnamespace_set() {
        let params = new_builder()
            .ucnamespace(&[0])
            .ucuser(&["Foo"])
            .data
            .params();
        assert_eq!(params["ucnamespace"], "0");
    }

    #[test]
    fn ucprop_set() {
        let params = new_builder()
            .ucprop(&["ids", "title", "timestamp"])
            .ucuser(&["Foo"])
            .data
            .params();
        assert_eq!(params["ucprop"], "ids|title|timestamp");
    }

    #[test]
    fn ucdir_newer() {
        let params = new_builder().ucdir("newer").ucuser(&["Foo"]).data.params();
        assert_eq!(params["ucdir"], "newer");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let builder = new_builder().ucuser(&["ExampleUser"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "usercontribs");
    }

    #[tokio::test]
    async fn test_usercontribs() {
        use wiremock::matchers::query_param;
        use wiremock::{Mock, ResponseTemplate};
        let server = crate::test_helpers::test_helpers_mod::start_enwiki_mock().await;
        Mock::given(query_param("list", "usercontribs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "usercontribs": [
                        {"userid": 1, "user": "Magnus Manske", "pageid": 736,
                         "revid": 1001, "ns": 0, "title": "Albert Einstein",
                         "timestamp": "2024-01-01T00:00:00Z", "sizediff": 100}
                    ]
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApiList::usercontribs()
            .ucuser(&["Magnus Manske"])
            .uclimit(5)
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["usercontribs"].is_array());
    }
}
