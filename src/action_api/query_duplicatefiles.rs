use super::{
    ActionApiContinuable, ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData,
    ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `prop=duplicatefiles` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiQueryDuplicatefilesData {
    common: ActionApiQueryCommonData,
    dflimit: Option<usize>,
    dfcontinue: Option<String>,
    dfdir: Option<String>,
    dflocalonly: bool,
}

impl ActionApiData for ActionApiQueryDuplicatefilesData {}

impl ActionApiQueryDuplicatefilesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        if let Some(v) = self.dflimit {
            params.insert("dflimit".to_string(), v.to_string());
        }
        Self::add_str(&self.dfcontinue, "dfcontinue", &mut params);
        Self::add_str(&self.dfdir, "dfdir", &mut params);
        Self::add_boolean(self.dflocalonly, "dflocalonly", &mut params);
        params
    }
}

/// Builder for `prop=duplicatefiles` — finds all files that are duplicates of the given files.
#[derive(Debug, Clone)]
pub struct ActionApiQueryDuplicatefilesBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryDuplicatefilesData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryDuplicatefilesBuilder<T> {
    /// Maximum number of duplicate files to return (`dflimit`).
    pub fn dflimit(mut self, dflimit: usize) -> Self {
        self.data.dflimit = Some(dflimit);
        self
    }

    /// Direction to list: `"ascending"` or `"descending"` (`dfdir`).
    pub fn dfdir<S: AsRef<str>>(mut self, dfdir: S) -> Self {
        self.data.dfdir = Some(dfdir.as_ref().to_string());
        self
    }

    /// Only look for files in the local repository (`dflocalonly`).
    pub fn dflocalonly(mut self, dflocalonly: bool) -> Self {
        self.data.dflocalonly = dflocalonly;
        self
    }
}

impl ActionApiQueryDuplicatefilesBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryDuplicatefilesData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryDuplicatefilesBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryDuplicatefilesBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryDuplicatefilesBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryDuplicatefilesBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "duplicatefiles".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryDuplicatefilesBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action_api::{ActionApiQueryCommonBuilder, NoTitlesOrGenerator};

    fn new_builder() -> ActionApiQueryDuplicatefilesBuilder<NoTitlesOrGenerator> {
        ActionApiQueryDuplicatefilesBuilder::new()
    }

    #[test]
    fn default_dflimit_absent() {
        let params = new_builder().titles(&["File:Test.png"]).data.params();
        assert!(!params.contains_key("dflimit"));
    }

    #[test]
    fn dflimit_set() {
        let params = new_builder()
            .dflimit(5)
            .titles(&["File:Test.png"])
            .data
            .params();
        assert_eq!(params["dflimit"], "5");
    }

    #[test]
    fn dfdir_set() {
        let params = new_builder()
            .dfdir("descending")
            .titles(&["File:Test.png"])
            .data
            .params();
        assert_eq!(params["dfdir"], "descending");
    }

    #[test]
    fn dflocalonly_flag() {
        let params = new_builder()
            .dflocalonly(true)
            .titles(&["File:Test.png"])
            .data
            .params();
        assert_eq!(params["dflocalonly"], "");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["File:Test.png"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "duplicatefiles");
    }
}
