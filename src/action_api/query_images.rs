use super::{
    ActionApiContinuable, ActionApiData, ActionApiGenerator, ActionApiQueryCommonBuilder,
    ActionApiQueryCommonData, ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ActionApiQueryImagesData {
    common: ActionApiQueryCommonData,
    imlimit: usize,
    imcontinue: Option<String>,
    imimages: Option<Vec<String>>,
    imdir: Option<String>,
}

impl ActionApiData for ActionApiQueryImagesData {}

impl Default for ActionApiQueryImagesData {
    fn default() -> Self {
        Self {
            common: ActionApiQueryCommonData::default(),
            imlimit: 10,
            imcontinue: None,
            imimages: None,
            imdir: None,
        }
    }
}

impl ActionApiQueryImagesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        params.insert("imlimit".to_string(), self.imlimit.to_string());
        Self::add_str(&self.imcontinue, "imcontinue", &mut params);
        Self::add_vec(&self.imimages, "imimages", &mut params);
        Self::add_str(&self.imdir, "imdir", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiQueryImagesBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryImagesData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryImagesBuilder<T> {
    pub fn imlimit(mut self, imlimit: usize) -> Self {
        self.data.imlimit = imlimit;
        self
    }

    pub fn imimages<S: Into<String> + Clone>(mut self, imimages: &[S]) -> Self {
        self.data.imimages = Some(imimages.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn imdir<S: AsRef<str>>(mut self, imdir: S) -> Self {
        self.data.imdir = Some(imdir.as_ref().to_string());
        self
    }
}

impl ActionApiQueryImagesBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryImagesData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiGenerator for ActionApiQueryImagesBuilder<NoTitlesOrGenerator> {
    fn generator_params(&self) -> HashMap<String, String> {
        let mut params = Self::prefix_params('g', self.data.params());
        params.insert("generator".to_string(), "images".to_string());
        params
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryImagesBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryImagesBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryImagesBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryImagesBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "images".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryImagesBuilder<Runnable> {
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

    fn new_builder() -> ActionApiQueryImagesBuilder<NoTitlesOrGenerator> {
        ActionApiQueryImagesBuilder::new()
    }

    #[test]
    fn default_imlimit_is_10() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert_eq!(params["imlimit"], "10");
    }

    #[test]
    fn default_imimages_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("imimages"));
    }

    #[test]
    fn default_imdir_absent() {
        let params = new_builder().titles(&["Foo"]).data.params();
        assert!(!params.contains_key("imdir"));
    }

    #[test]
    fn imlimit_set() {
        let params = new_builder().imlimit(50).titles(&["Foo"]).data.params();
        assert_eq!(params["imlimit"], "50");
    }

    #[test]
    fn imimages_filter() {
        let params = new_builder()
            .imimages(&["File:Foo.jpg", "File:Bar.png"])
            .titles(&["Baz"])
            .data
            .params();
        assert_eq!(params["imimages"], "File:Foo.jpg|File:Bar.png");
    }

    #[test]
    fn imdir_descending() {
        let params = new_builder().imdir("descending").titles(&["Foo"]).data.params();
        assert_eq!(params["imdir"], "descending");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["Foo"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "images");
    }

    #[tokio::test]
    async fn test_images() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiQuery::images()
            .titles(&["Albert Einstein"])
            .run(&api)
            .await
            .unwrap();
        let pages = result["query"]["pages"].as_object().unwrap();
        assert!(!pages.is_empty());
    }
}
