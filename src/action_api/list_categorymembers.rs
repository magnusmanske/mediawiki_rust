use super::{ActionApiContinuable, ActionApiData, ActionApiGenerator, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `list=categorymembers` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListCategorymembersData {
    cmtitle: Option<String>,
    cmpageid: Option<u64>,
    cmprop: Option<Vec<String>>,
    cmnamespace: Option<Vec<NamespaceID>>,
    cmtype: Option<Vec<String>>,
    cmcontinue: Option<String>,
    cmlimit: usize,
    cmsort: Option<String>,
    cmdir: Option<String>,
    cmstart: Option<String>,
    cmend: Option<String>,
    cmstarthexsortkey: Option<String>,
    cmendhexsortkey: Option<String>,
    cmstartsortkeyprefix: Option<String>,
    cmendsortkeyprefix: Option<String>,
}

impl ActionApiData for ActionApiListCategorymembersData {}

impl Default for ActionApiListCategorymembersData {
    fn default() -> Self {
        Self {
            cmtitle: None,
            cmpageid: None,
            cmprop: None,
            cmnamespace: None,
            cmtype: None,
            cmcontinue: None,
            cmlimit: 10,
            cmsort: None,
            cmdir: None,
            cmstart: None,
            cmend: None,
            cmstarthexsortkey: None,
            cmendhexsortkey: None,
            cmstartsortkeyprefix: None,
            cmendsortkeyprefix: None,
        }
    }
}

impl ActionApiListCategorymembersData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.cmtitle, "cmtitle", &mut params);
        if let Some(cmpageid) = self.cmpageid {
            params.insert("cmpageid".to_string(), cmpageid.to_string());
        }
        Self::add_vec(&self.cmprop, "cmprop", &mut params);
        if let Some(ns) = &self.cmnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("cmnamespace".to_string(), s.join("|"));
        }
        Self::add_vec(&self.cmtype, "cmtype", &mut params);
        Self::add_str(&self.cmcontinue, "cmcontinue", &mut params);
        params.insert("cmlimit".to_string(), self.cmlimit.to_string());
        Self::add_str(&self.cmsort, "cmsort", &mut params);
        Self::add_str(&self.cmdir, "cmdir", &mut params);
        Self::add_str(&self.cmstart, "cmstart", &mut params);
        Self::add_str(&self.cmend, "cmend", &mut params);
        Self::add_str(&self.cmstarthexsortkey, "cmstarthexsortkey", &mut params);
        Self::add_str(&self.cmendhexsortkey, "cmendhexsortkey", &mut params);
        Self::add_str(&self.cmstartsortkeyprefix, "cmstartsortkeyprefix", &mut params);
        Self::add_str(&self.cmendsortkeyprefix, "cmendsortkeyprefix", &mut params);
        params
    }
}

/// Builder for the `list=categorymembers` API module; supports pagination via `ActionApiContinuable`.
#[derive(Debug, Clone)]
pub struct ActionApiListCategorymembersBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiListCategorymembersData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiListCategorymembersBuilder<T> {
    /// Properties to retrieve for each member page (`cmprop`).
    pub fn cmprop<S: Into<String> + Clone>(mut self, cmprop: &[S]) -> Self {
        self.data.cmprop = Some(cmprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Filter members to these namespaces (`cmnamespace`).
    pub fn cmnamespace(mut self, cmnamespace: &[NamespaceID]) -> Self {
        self.data.cmnamespace = Some(cmnamespace.to_vec());
        self
    }

    /// Filter by member type: `page`, `subcat`, or `file` (`cmtype`).
    pub fn cmtype<S: Into<String> + Clone>(mut self, cmtype: &[S]) -> Self {
        self.data.cmtype = Some(cmtype.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Maximum number of members to return (`cmlimit`).
    pub fn cmlimit(mut self, cmlimit: usize) -> Self {
        self.data.cmlimit = cmlimit;
        self
    }

    /// Property to sort members by: `sortkey` or `timestamp` (`cmsort`).
    pub fn cmsort<S: AsRef<str>>(mut self, cmsort: S) -> Self {
        self.data.cmsort = Some(cmsort.as_ref().to_string());
        self
    }

    /// Sort direction (`asc` or `desc`) (`cmdir`).
    pub fn cmdir<S: AsRef<str>>(mut self, cmdir: S) -> Self {
        self.data.cmdir = Some(cmdir.as_ref().to_string());
        self
    }

    /// Timestamp or sortkey to start listing from (`cmstart`).
    pub fn cmstart<S: AsRef<str>>(mut self, cmstart: S) -> Self {
        self.data.cmstart = Some(cmstart.as_ref().to_string());
        self
    }

    /// Timestamp or sortkey to stop listing at (`cmend`).
    pub fn cmend<S: AsRef<str>>(mut self, cmend: S) -> Self {
        self.data.cmend = Some(cmend.as_ref().to_string());
        self
    }

    /// Hex sortkey to start listing from when sorted by `sortkey` (`cmstarthexsortkey`).
    pub fn cmstarthexsortkey<S: AsRef<str>>(mut self, cmstarthexsortkey: S) -> Self {
        self.data.cmstarthexsortkey = Some(cmstarthexsortkey.as_ref().to_string());
        self
    }

    /// Hex sortkey to stop listing at when sorted by `sortkey` (`cmendhexsortkey`).
    pub fn cmendhexsortkey<S: AsRef<str>>(mut self, cmendhexsortkey: S) -> Self {
        self.data.cmendhexsortkey = Some(cmendhexsortkey.as_ref().to_string());
        self
    }

    /// Sortkey prefix to start listing from (`cmstartsortkeyprefix`).
    pub fn cmstartsortkeyprefix<S: AsRef<str>>(mut self, cmstartsortkeyprefix: S) -> Self {
        self.data.cmstartsortkeyprefix = Some(cmstartsortkeyprefix.as_ref().to_string());
        self
    }

    /// Sortkey prefix to stop listing at (`cmendsortkeyprefix`).
    pub fn cmendsortkeyprefix<S: AsRef<str>>(mut self, cmendsortkeyprefix: S) -> Self {
        self.data.cmendsortkeyprefix = Some(cmendsortkeyprefix.as_ref().to_string());
        self
    }
}

impl ActionApiListCategorymembersBuilder<NoTitlesOrGenerator> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiListCategorymembersData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Title of the category to enumerate members of (`cmtitle`).
    pub fn cmtitle<S: AsRef<str>>(
        mut self,
        cmtitle: S,
    ) -> ActionApiListCategorymembersBuilder<Runnable> {
        self.data.cmtitle = Some(cmtitle.as_ref().to_string());
        ActionApiListCategorymembersBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }

    /// Page ID of the category to enumerate members of (`cmpageid`).
    pub fn cmpageid(mut self, cmpageid: u64) -> ActionApiListCategorymembersBuilder<Runnable> {
        self.data.cmpageid = Some(cmpageid);
        ActionApiListCategorymembersBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiGenerator for ActionApiListCategorymembersBuilder<NoTitlesOrGenerator> {
    fn generator_params(&self) -> HashMap<String, String> {
        let mut params = Self::prefix_params('g', self.data.params());
        params.insert("generator".to_string(), "categorymembers".to_string());
        params
    }
}

impl ActionApiRunnable for ActionApiListCategorymembersBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "categorymembers".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListCategorymembersBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiList};

    fn new_builder() -> ActionApiListCategorymembersBuilder<NoTitlesOrGenerator> {
        ActionApiListCategorymembersBuilder::new()
    }

    #[test]
    fn default_cmlimit_is_10() {
        let params = new_builder().cmtitle("Category:Physics").data.params();
        assert_eq!(params["cmlimit"], "10");
    }

    #[test]
    fn cmtitle_set() {
        let params = new_builder().cmtitle("Category:Physics").data.params();
        assert_eq!(params["cmtitle"], "Category:Physics");
    }

    #[test]
    fn cmpageid_set() {
        let params = new_builder().cmpageid(1234).data.params();
        assert_eq!(params["cmpageid"], "1234");
    }

    #[test]
    fn cmtitle_does_not_set_cmpageid() {
        let params = new_builder().cmtitle("Category:Physics").data.params();
        assert!(!params.contains_key("cmpageid"));
    }

    #[test]
    fn cmprop_set() {
        let params = new_builder().cmprop(&["ids", "title"]).cmtitle("Category:Foo").data.params();
        assert_eq!(params["cmprop"], "ids|title");
    }

    #[test]
    fn cmnamespace_set() {
        let params = new_builder().cmnamespace(&[0]).cmtitle("Category:Foo").data.params();
        assert_eq!(params["cmnamespace"], "0");
    }

    #[test]
    fn cmtype_set() {
        let params = new_builder()
            .cmtype(&["page", "subcat"])
            .cmtitle("Category:Foo")
            .data
            .params();
        assert_eq!(params["cmtype"], "page|subcat");
    }

    #[test]
    fn cmlimit_set() {
        let params = new_builder().cmlimit(50).cmtitle("Category:Foo").data.params();
        assert_eq!(params["cmlimit"], "50");
    }

    #[test]
    fn cmsort_timestamp() {
        let params = new_builder().cmsort("timestamp").cmtitle("Category:Foo").data.params();
        assert_eq!(params["cmsort"], "timestamp");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let builder = new_builder().cmtitle("Category:Physics");
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "categorymembers");
    }

    #[tokio::test]
    async fn test_categorymembers() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiList::categorymembers()
            .cmtitle("Category:Physics")
            .cmlimit(5)
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["categorymembers"].is_array());
    }
}
