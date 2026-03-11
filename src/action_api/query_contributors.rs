use super::{
    ActionApiContinuable, ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData,
    ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ActionApiQueryContributorsData {
    common: ActionApiQueryCommonData,
    pcgroup: Option<Vec<String>>,
    pcexcludegroup: Option<Vec<String>>,
    pcrights: Option<Vec<String>>,
    pcexcluderights: Option<Vec<String>>,
    pclimit: usize,
    pccontinue: Option<String>,
}

impl ActionApiData for ActionApiQueryContributorsData {}

impl Default for ActionApiQueryContributorsData {
    fn default() -> Self {
        Self {
            common: ActionApiQueryCommonData::default(),
            pcgroup: None,
            pcexcludegroup: None,
            pcrights: None,
            pcexcluderights: None,
            pclimit: 10,
            pccontinue: None,
        }
    }
}

impl ActionApiQueryContributorsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_vec(&self.pcgroup, "pcgroup", &mut params);
        Self::add_vec(&self.pcexcludegroup, "pcexcludegroup", &mut params);
        Self::add_vec(&self.pcrights, "pcrights", &mut params);
        Self::add_vec(&self.pcexcluderights, "pcexcluderights", &mut params);
        params.insert("pclimit".to_string(), self.pclimit.to_string());
        Self::add_str(&self.pccontinue, "pccontinue", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiQueryContributorsBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryContributorsData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryContributorsBuilder<T> {
    pub fn pcgroup<S: Into<String> + Clone>(mut self, pcgroup: &[S]) -> Self {
        self.data.pcgroup = Some(pcgroup.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn pcexcludegroup<S: Into<String> + Clone>(mut self, pcexcludegroup: &[S]) -> Self {
        self.data.pcexcludegroup = Some(pcexcludegroup.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn pcrights<S: Into<String> + Clone>(mut self, pcrights: &[S]) -> Self {
        self.data.pcrights = Some(pcrights.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn pcexcluderights<S: Into<String> + Clone>(mut self, pcexcluderights: &[S]) -> Self {
        self.data.pcexcluderights =
            Some(pcexcluderights.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn pclimit(mut self, pclimit: usize) -> Self {
        self.data.pclimit = pclimit;
        self
    }
}

impl ActionApiQueryContributorsBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryContributorsData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryContributorsBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryContributorsBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryContributorsBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryContributorsBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "contributors".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryContributorsBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Api,
        action_api::{ActionApiQuery, ActionApiQueryCommonBuilder, NoTitlesOrGenerator},
    };

    fn new_builder() -> ActionApiQueryContributorsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryContributorsBuilder::new()
    }

    #[test]
    fn default_pclimit_is_10() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert_eq!(params["pclimit"], "10");
    }

    #[test]
    fn default_pcgroup_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("pcgroup"));
    }

    #[test]
    fn pcgroup_set() {
        let params = new_builder().pcgroup(&["sysop", "bot"]).titles(&["Foo"]).data.params();
        assert_eq!(params["pcgroup"], "sysop|bot");
    }

    #[test]
    fn pcexcludegroup_set() {
        let params = new_builder().pcexcludegroup(&["bot"]).titles(&["Foo"]).data.params();
        assert_eq!(params["pcexcludegroup"], "bot");
    }

    #[test]
    fn pcrights_set() {
        let params = new_builder().pcrights(&["edit"]).titles(&["Foo"]).data.params();
        assert_eq!(params["pcrights"], "edit");
    }

    #[test]
    fn pcexcluderights_set() {
        let params = new_builder().pcexcluderights(&["delete"]).titles(&["Foo"]).data.params();
        assert_eq!(params["pcexcluderights"], "delete");
    }

    #[test]
    fn pclimit_set() {
        let params = new_builder().pclimit(25).titles(&["Foo"]).data.params();
        assert_eq!(params["pclimit"], "25");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "contributors");
    }

    #[tokio::test]
    async fn test_contributors() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiQuery::contributors()
            .titles(&["Albert Einstein"])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
