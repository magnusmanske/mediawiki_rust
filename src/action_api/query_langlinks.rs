use super::{
    ActionApiContinuable, ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData,
    ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ActionApiQueryLanglinksData {
    common: ActionApiQueryCommonData,
    llprop: Option<Vec<String>>,
    lllang: Option<String>,
    lltitle: Option<String>,
    lldir: Option<String>,
    llinlanguagecode: Option<String>,
    lllimit: usize,
    llcontinue: Option<String>,
}

impl ActionApiData for ActionApiQueryLanglinksData {}

impl Default for ActionApiQueryLanglinksData {
    fn default() -> Self {
        Self {
            common: ActionApiQueryCommonData::default(),
            llprop: None,
            lllang: None,
            lltitle: None,
            lldir: None,
            llinlanguagecode: None,
            lllimit: 10,
            llcontinue: None,
        }
    }
}

impl ActionApiQueryLanglinksData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_vec(&self.llprop, "llprop", &mut params);
        Self::add_str(&self.lllang, "lllang", &mut params);
        Self::add_str(&self.lltitle, "lltitle", &mut params);
        Self::add_str(&self.lldir, "lldir", &mut params);
        Self::add_str(&self.llinlanguagecode, "llinlanguagecode", &mut params);
        params.insert("lllimit".to_string(), self.lllimit.to_string());
        Self::add_str(&self.llcontinue, "llcontinue", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiQueryLanglinksBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryLanglinksData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryLanglinksBuilder<T> {
    pub fn llprop<S: Into<String> + Clone>(mut self, llprop: &[S]) -> Self {
        self.data.llprop = Some(llprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn lllang<S: AsRef<str>>(mut self, lllang: S) -> Self {
        self.data.lllang = Some(lllang.as_ref().to_string());
        self
    }

    pub fn lltitle<S: AsRef<str>>(mut self, lltitle: S) -> Self {
        self.data.lltitle = Some(lltitle.as_ref().to_string());
        self
    }

    pub fn lldir<S: AsRef<str>>(mut self, lldir: S) -> Self {
        self.data.lldir = Some(lldir.as_ref().to_string());
        self
    }

    pub fn llinlanguagecode<S: AsRef<str>>(mut self, llinlanguagecode: S) -> Self {
        self.data.llinlanguagecode = Some(llinlanguagecode.as_ref().to_string());
        self
    }

    pub fn lllimit(mut self, lllimit: usize) -> Self {
        self.data.lllimit = lllimit;
        self
    }
}

impl ActionApiQueryLanglinksBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryLanglinksData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryLanglinksBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryLanglinksBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryLanglinksBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryLanglinksBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "langlinks".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryLanglinksBuilder<Runnable> {
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

    fn new_builder() -> ActionApiQueryLanglinksBuilder<NoTitlesOrGenerator> {
        ActionApiQueryLanglinksBuilder::new()
    }

    #[test]
    fn default_lllimit_is_10() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert_eq!(params["lllimit"], "10");
    }

    #[test]
    fn default_llprop_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("llprop"));
    }

    #[test]
    fn default_lllang_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("lllang"));
    }

    #[test]
    fn llprop_set() {
        let params = new_builder().llprop(&["url", "langname"]).titles(&["Foo"]).data.params();
        assert_eq!(params["llprop"], "url|langname");
    }

    #[test]
    fn lllang_set() {
        let params = new_builder().lllang("de").titles(&["Foo"]).data.params();
        assert_eq!(params["lllang"], "de");
    }

    #[test]
    fn lltitle_set() {
        let params = new_builder().lltitle("Berlin").lllang("de").titles(&["Foo"]).data.params();
        assert_eq!(params["lltitle"], "Berlin");
    }

    #[test]
    fn llinlanguagecode_set() {
        let params = new_builder().llinlanguagecode("fr").titles(&["Foo"]).data.params();
        assert_eq!(params["llinlanguagecode"], "fr");
    }

    #[test]
    fn lllimit_set() {
        let params = new_builder().lllimit(50).titles(&["Foo"]).data.params();
        assert_eq!(params["lllimit"], "50");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "langlinks");
    }

    #[tokio::test]
    async fn test_langlinks() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiQuery::langlinks()
            .titles(&["Albert Einstein"])
            .lllang("de")
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
