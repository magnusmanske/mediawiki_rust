use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ActionApiListPrefixsearchData {
    pssearch: Option<String>,
    psnamespace: Option<Vec<NamespaceID>>,
    pslimit: usize,
    psoffset: usize,
}

impl ActionApiData for ActionApiListPrefixsearchData {}

impl Default for ActionApiListPrefixsearchData {
    fn default() -> Self {
        Self {
            pssearch: None,
            psnamespace: None,
            pslimit: 10,
            psoffset: 0,
        }
    }
}

impl ActionApiListPrefixsearchData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(pssearch) = &self.pssearch {
            params.insert("pssearch".to_string(), pssearch.clone());
        }
        if let Some(ns) = &self.psnamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("psnamespace".to_string(), s.join("|"));
        }
        params.insert("pslimit".to_string(), self.pslimit.to_string());
        if self.psoffset > 0 {
            params.insert("psoffset".to_string(), self.psoffset.to_string());
        }
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiListPrefixsearchBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiListPrefixsearchData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiListPrefixsearchBuilder<T> {
    pub fn psnamespace(mut self, psnamespace: &[NamespaceID]) -> Self {
        self.data.psnamespace = Some(psnamespace.to_vec());
        self
    }

    pub fn pslimit(mut self, pslimit: usize) -> Self {
        self.data.pslimit = pslimit;
        self
    }

    pub fn psoffset(mut self, psoffset: usize) -> Self {
        self.data.psoffset = psoffset;
        self
    }
}

impl ActionApiListPrefixsearchBuilder<NoTitlesOrGenerator> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiListPrefixsearchData::default(),
            continue_params: HashMap::new(),
        }
    }

    pub fn pssearch<S: AsRef<str>>(
        mut self,
        pssearch: S,
    ) -> ActionApiListPrefixsearchBuilder<Runnable> {
        self.data.pssearch = Some(pssearch.as_ref().to_string());
        ActionApiListPrefixsearchBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiRunnable for ActionApiListPrefixsearchBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "prefixsearch".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListPrefixsearchBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiList};

    fn new_builder() -> ActionApiListPrefixsearchBuilder<NoTitlesOrGenerator> {
        ActionApiListPrefixsearchBuilder::new()
    }

    #[test]
    fn default_pslimit_is_10() {
        let params = new_builder().pssearch("Albert").data.params();
        assert_eq!(params["pslimit"], "10");
    }

    #[test]
    fn pssearch_set() {
        let params = new_builder().pssearch("Albert Einstein").data.params();
        assert_eq!(params["pssearch"], "Albert Einstein");
    }

    #[test]
    fn psnamespace_set() {
        let params = new_builder().psnamespace(&[0, 4]).pssearch("Foo").data.params();
        assert_eq!(params["psnamespace"], "0|4");
    }

    #[test]
    fn pslimit_set() {
        let params = new_builder().pslimit(20).pssearch("Foo").data.params();
        assert_eq!(params["pslimit"], "20");
    }

    #[test]
    fn psoffset_zero_absent() {
        let params = new_builder().pssearch("Foo").data.params();
        assert!(!params.contains_key("psoffset"));
    }

    #[test]
    fn psoffset_nonzero_set() {
        let params = new_builder().psoffset(10).pssearch("Foo").data.params();
        assert_eq!(params["psoffset"], "10");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let builder = new_builder().pssearch("Albert");
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "prefixsearch");
    }

    #[tokio::test]
    async fn test_prefixsearch() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiList::prefixsearch()
            .pssearch("Albert Einstein")
            .pslimit(5)
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["prefixsearch"].is_array());
    }
}
