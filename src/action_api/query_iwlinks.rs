use super::{
    ActionApiContinuable, ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData,
    ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ActionApiQueryIwlinksData {
    common: ActionApiQueryCommonData,
    iwprop: Option<Vec<String>>,
    iwprefix: Option<String>,
    iwtitle: Option<String>,
    iwdir: Option<String>,
    iwlimit: usize,
    iwcontinue: Option<String>,
}

impl ActionApiData for ActionApiQueryIwlinksData {}

impl Default for ActionApiQueryIwlinksData {
    fn default() -> Self {
        Self {
            common: ActionApiQueryCommonData::default(),
            iwprop: None,
            iwprefix: None,
            iwtitle: None,
            iwdir: None,
            iwlimit: 10,
            iwcontinue: None,
        }
    }
}

impl ActionApiQueryIwlinksData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_vec(&self.iwprop, "iwprop", &mut params);
        Self::add_str(&self.iwprefix, "iwprefix", &mut params);
        Self::add_str(&self.iwtitle, "iwtitle", &mut params);
        Self::add_str(&self.iwdir, "iwdir", &mut params);
        params.insert("iwlimit".to_string(), self.iwlimit.to_string());
        Self::add_str(&self.iwcontinue, "iwcontinue", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiQueryIwlinksBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryIwlinksData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryIwlinksBuilder<T> {
    pub fn iwprop<S: Into<String> + Clone>(mut self, iwprop: &[S]) -> Self {
        self.data.iwprop = Some(iwprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn iwprefix<S: AsRef<str>>(mut self, iwprefix: S) -> Self {
        self.data.iwprefix = Some(iwprefix.as_ref().to_string());
        self
    }

    pub fn iwtitle<S: AsRef<str>>(mut self, iwtitle: S) -> Self {
        self.data.iwtitle = Some(iwtitle.as_ref().to_string());
        self
    }

    pub fn iwdir<S: AsRef<str>>(mut self, iwdir: S) -> Self {
        self.data.iwdir = Some(iwdir.as_ref().to_string());
        self
    }

    pub fn iwlimit(mut self, iwlimit: usize) -> Self {
        self.data.iwlimit = iwlimit;
        self
    }
}

impl ActionApiQueryIwlinksBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryIwlinksData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryIwlinksBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryIwlinksBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryIwlinksBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryIwlinksBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "iwlinks".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryIwlinksBuilder<Runnable> {
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

    fn new_builder() -> ActionApiQueryIwlinksBuilder<NoTitlesOrGenerator> {
        ActionApiQueryIwlinksBuilder::new()
    }

    #[test]
    fn default_iwlimit_is_10() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert_eq!(params["iwlimit"], "10");
    }

    #[test]
    fn default_iwprop_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("iwprop"));
    }

    #[test]
    fn iwprop_url() {
        let params = new_builder().iwprop(&["url"]).titles(&["Foo"]).data.params();
        assert_eq!(params["iwprop"], "url");
    }

    #[test]
    fn iwprefix_set() {
        let params = new_builder().iwprefix("de").titles(&["Foo"]).data.params();
        assert_eq!(params["iwprefix"], "de");
    }

    #[test]
    fn iwtitle_set() {
        let params = new_builder().iwtitle("Berlin").iwprefix("de").titles(&["Foo"]).data.params();
        assert_eq!(params["iwtitle"], "Berlin");
    }

    #[test]
    fn iwdir_descending() {
        let params = new_builder().iwdir("descending").titles(&["Foo"]).data.params();
        assert_eq!(params["iwdir"], "descending");
    }

    #[test]
    fn iwlimit_set() {
        let params = new_builder().iwlimit(25).titles(&["Foo"]).data.params();
        assert_eq!(params["iwlimit"], "25");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "iwlinks");
    }

    #[tokio::test]
    async fn test_iwlinks() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiQuery::iwlinks()
            .titles(&["Albert Einstein"])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
