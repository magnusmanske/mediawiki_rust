use super::{
    ActionApiContinuable, ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData,
    ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `prop=deletedrevisions` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiQueryDeletedrevisionsData {
    common: ActionApiQueryCommonData,
    drvprop: Option<Vec<String>>,
    drvlimit: Option<usize>,
    drvexpandtemplates: bool,
    drvgeneratexml: bool,
    drvparse: bool,
    drvsection: Option<String>,
    drvdiffto: Option<String>,
    drvdifftotext: Option<String>,
    drvdifftotextpst: bool,
    drvcontentformat: Option<String>,
    drvstart: Option<String>,
    drvend: Option<String>,
    drvdir: Option<String>,
    drvtag: Option<String>,
    drvuser: Option<String>,
    drvexcludeuser: Option<String>,
    drvcontinue: Option<String>,
}

impl ActionApiData for ActionApiQueryDeletedrevisionsData {}

impl ActionApiQueryDeletedrevisionsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_vec(&self.drvprop, "drvprop", &mut params);
        if let Some(v) = self.drvlimit {
            params.insert("drvlimit".to_string(), v.to_string());
        }
        Self::add_boolean(self.drvexpandtemplates, "drvexpandtemplates", &mut params);
        Self::add_boolean(self.drvgeneratexml, "drvgeneratexml", &mut params);
        Self::add_boolean(self.drvparse, "drvparse", &mut params);
        Self::add_str(&self.drvsection, "drvsection", &mut params);
        Self::add_str(&self.drvdiffto, "drvdiffto", &mut params);
        Self::add_str(&self.drvdifftotext, "drvdifftotext", &mut params);
        Self::add_boolean(self.drvdifftotextpst, "drvdifftotextpst", &mut params);
        Self::add_str(&self.drvcontentformat, "drvcontentformat", &mut params);
        Self::add_str(&self.drvstart, "drvstart", &mut params);
        Self::add_str(&self.drvend, "drvend", &mut params);
        Self::add_str(&self.drvdir, "drvdir", &mut params);
        Self::add_str(&self.drvtag, "drvtag", &mut params);
        Self::add_str(&self.drvuser, "drvuser", &mut params);
        Self::add_str(&self.drvexcludeuser, "drvexcludeuser", &mut params);
        Self::add_str(&self.drvcontinue, "drvcontinue", &mut params);
        params
    }
}

/// Builder for `prop=deletedrevisions` — gets deleted revision information.
#[derive(Debug, Clone)]
pub struct ActionApiQueryDeletedrevisionsBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryDeletedrevisionsData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryDeletedrevisionsBuilder<T> {
    /// Properties to return for each deleted revision (`drvprop`).
    pub fn drvprop<S: Into<String> + Clone>(mut self, drvprop: &[S]) -> Self {
        self.data.drvprop = Some(drvprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Maximum number of revisions to return (`drvlimit`).
    pub fn drvlimit(mut self, drvlimit: usize) -> Self {
        self.data.drvlimit = Some(drvlimit);
        self
    }

    /// Expand templates in revision content (`drvexpandtemplates`).
    pub fn drvexpandtemplates(mut self, drvexpandtemplates: bool) -> Self {
        self.data.drvexpandtemplates = drvexpandtemplates;
        self
    }

    /// Generate XML parse tree for revision content (`drvgeneratexml`).
    pub fn drvgeneratexml(mut self, drvgeneratexml: bool) -> Self {
        self.data.drvgeneratexml = drvgeneratexml;
        self
    }

    /// Parse revision content (`drvparse`).
    pub fn drvparse(mut self, drvparse: bool) -> Self {
        self.data.drvparse = drvparse;
        self
    }

    /// Retrieve only this section (`drvsection`).
    pub fn drvsection<S: AsRef<str>>(mut self, drvsection: S) -> Self {
        self.data.drvsection = Some(drvsection.as_ref().to_string());
        self
    }

    /// Diff revision against this (`drvdiffto`).
    pub fn drvdiffto<S: AsRef<str>>(mut self, drvdiffto: S) -> Self {
        self.data.drvdiffto = Some(drvdiffto.as_ref().to_string());
        self
    }

    /// Diff revision against this text (`drvdifftotext`).
    pub fn drvdifftotext<S: AsRef<str>>(mut self, drvdifftotext: S) -> Self {
        self.data.drvdifftotext = Some(drvdifftotext.as_ref().to_string());
        self
    }

    /// Pre-save transform the text before diffing (`drvdifftotextpst`).
    pub fn drvdifftotextpst(mut self, drvdifftotextpst: bool) -> Self {
        self.data.drvdifftotextpst = drvdifftotextpst;
        self
    }

    /// Serialization format to use for content (`drvcontentformat`).
    pub fn drvcontentformat<S: AsRef<str>>(mut self, drvcontentformat: S) -> Self {
        self.data.drvcontentformat = Some(drvcontentformat.as_ref().to_string());
        self
    }

    /// Start listing from this timestamp (`drvstart`).
    pub fn drvstart<S: AsRef<str>>(mut self, drvstart: S) -> Self {
        self.data.drvstart = Some(drvstart.as_ref().to_string());
        self
    }

    /// Stop listing at this timestamp (`drvend`).
    pub fn drvend<S: AsRef<str>>(mut self, drvend: S) -> Self {
        self.data.drvend = Some(drvend.as_ref().to_string());
        self
    }

    /// Direction to enumerate (`drvdir`).
    pub fn drvdir<S: AsRef<str>>(mut self, drvdir: S) -> Self {
        self.data.drvdir = Some(drvdir.as_ref().to_string());
        self
    }

    /// Only list revisions tagged with this tag (`drvtag`).
    pub fn drvtag<S: AsRef<str>>(mut self, drvtag: S) -> Self {
        self.data.drvtag = Some(drvtag.as_ref().to_string());
        self
    }

    /// Only list revisions by this user (`drvuser`).
    pub fn drvuser<S: AsRef<str>>(mut self, drvuser: S) -> Self {
        self.data.drvuser = Some(drvuser.as_ref().to_string());
        self
    }

    /// Exclude revisions by this user (`drvexcludeuser`).
    pub fn drvexcludeuser<S: AsRef<str>>(mut self, drvexcludeuser: S) -> Self {
        self.data.drvexcludeuser = Some(drvexcludeuser.as_ref().to_string());
        self
    }
}

impl ActionApiQueryDeletedrevisionsBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryDeletedrevisionsData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryDeletedrevisionsBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryDeletedrevisionsBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryDeletedrevisionsBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryDeletedrevisionsBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "deletedrevisions".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryDeletedrevisionsBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action_api::{ActionApiQueryCommonBuilder, NoTitlesOrGenerator};

    fn new_builder() -> ActionApiQueryDeletedrevisionsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryDeletedrevisionsBuilder::new()
    }

    #[test]
    fn default_drvprop_absent() {
        let params = new_builder().titles(&["TestPage"]).data.params();
        assert!(!params.contains_key("drvprop"));
    }

    #[test]
    fn drvprop_set() {
        let params = new_builder()
            .drvprop(&["ids", "timestamp", "content"])
            .titles(&["TestPage"])
            .data
            .params();
        assert_eq!(params["drvprop"], "ids|timestamp|content");
    }

    #[test]
    fn drvlimit_set() {
        let params = new_builder()
            .drvlimit(5)
            .titles(&["TestPage"])
            .data
            .params();
        assert_eq!(params["drvlimit"], "5");
    }

    #[test]
    fn drvexpandtemplates_flag() {
        let params = new_builder()
            .drvexpandtemplates(true)
            .titles(&["TestPage"])
            .data
            .params();
        assert_eq!(params["drvexpandtemplates"], "");
    }

    #[test]
    fn drvuser_set() {
        let params = new_builder()
            .drvuser("ExampleUser")
            .titles(&["TestPage"])
            .data
            .params();
        assert_eq!(params["drvuser"], "ExampleUser");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["TestPage"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "deletedrevisions");
    }
}
