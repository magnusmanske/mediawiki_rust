use super::{
    ActionApiContinuable, ActionApiData, ActionApiGenerator, ActionApiRunnable,
    NoTitlesOrGenerator, Runnable,
};
use crate::api::NamespaceID;
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `list=imageusage` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListImageusageData {
    iutitle: Option<String>,
    iupageid: Option<u64>,
    iucontinue: Option<String>,
    iunamespace: Option<Vec<NamespaceID>>,
    iudir: Option<String>,
    iufilterredir: Option<String>,
    iulimit: usize,
    iuredirect: bool,
}

impl ActionApiData for ActionApiListImageusageData {}

impl Default for ActionApiListImageusageData {
    fn default() -> Self {
        Self {
            iutitle: None,
            iupageid: None,
            iucontinue: None,
            iunamespace: None,
            iudir: None,
            iufilterredir: None,
            iulimit: 10,
            iuredirect: false,
        }
    }
}

impl ActionApiListImageusageData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.iutitle, "iutitle", &mut params);
        if let Some(iupageid) = self.iupageid {
            params.insert("iupageid".to_string(), iupageid.to_string());
        }
        Self::add_str(&self.iucontinue, "iucontinue", &mut params);
        if let Some(ns) = &self.iunamespace {
            let s: Vec<String> = ns.iter().map(|n| n.to_string()).collect();
            params.insert("iunamespace".to_string(), s.join("|"));
        }
        Self::add_str(&self.iudir, "iudir", &mut params);
        Self::add_str(&self.iufilterredir, "iufilterredir", &mut params);
        params.insert("iulimit".to_string(), self.iulimit.to_string());
        Self::add_boolean(self.iuredirect, "iuredirect", &mut params);
        params
    }
}

/// Builder for the `list=imageusage` API module; supports pagination via `ActionApiContinuable`.
#[derive(Debug, Clone)]
pub struct ActionApiListImageusageBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiListImageusageData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiListImageusageBuilder<T> {
    /// Filter results to pages in these namespaces (`iunamespace`).
    pub fn iunamespace(mut self, iunamespace: &[NamespaceID]) -> Self {
        self.data.iunamespace = Some(iunamespace.to_vec());
        self
    }

    /// Sort direction for listing (`iudir`).
    pub fn iudir<S: AsRef<str>>(mut self, iudir: S) -> Self {
        self.data.iudir = Some(iudir.as_ref().to_string());
        self
    }

    /// Filter by redirect status of pages using the image (`iufilterredir`).
    pub fn iufilterredir<S: AsRef<str>>(mut self, iufilterredir: S) -> Self {
        self.data.iufilterredir = Some(iufilterredir.as_ref().to_string());
        self
    }

    /// Maximum number of pages to return (`iulimit`).
    pub fn iulimit(mut self, iulimit: usize) -> Self {
        self.data.iulimit = iulimit;
        self
    }

    /// When set, also list pages that link to redirects pointing at the image (`iuredirect`).
    pub fn iuredirect(mut self, iuredirect: bool) -> Self {
        self.data.iuredirect = iuredirect;
        self
    }
}

impl ActionApiListImageusageBuilder<NoTitlesOrGenerator> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiListImageusageData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Title of the image to find usage for (`iutitle`).
    pub fn iutitle<S: AsRef<str>>(
        mut self,
        iutitle: S,
    ) -> ActionApiListImageusageBuilder<Runnable> {
        self.data.iutitle = Some(iutitle.as_ref().to_string());
        ActionApiListImageusageBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }

    /// Page ID of the image to find usage for (`iupageid`).
    pub fn iupageid(mut self, iupageid: u64) -> ActionApiListImageusageBuilder<Runnable> {
        self.data.iupageid = Some(iupageid);
        ActionApiListImageusageBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiGenerator for ActionApiListImageusageBuilder<NoTitlesOrGenerator> {
    fn generator_params(&self) -> HashMap<String, String> {
        let mut params = Self::prefix_params('g', self.data.params());
        params.insert("generator".to_string(), "imageusage".to_string());
        params
    }
}

impl ActionApiRunnable for ActionApiListImageusageBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "imageusage".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListImageusageBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiList};

    fn new_builder() -> ActionApiListImageusageBuilder<NoTitlesOrGenerator> {
        ActionApiListImageusageBuilder::new()
    }

    #[test]
    fn default_iulimit_is_10() {
        let params = new_builder().iutitle("File:Foo.jpg").data.params();
        assert_eq!(params["iulimit"], "10");
    }

    #[test]
    fn iutitle_set() {
        let params = new_builder().iutitle("File:Einstein1921.jpg").data.params();
        assert_eq!(params["iutitle"], "File:Einstein1921.jpg");
    }

    #[test]
    fn iupageid_set() {
        let params = new_builder().iupageid(12345).data.params();
        assert_eq!(params["iupageid"], "12345");
    }

    #[test]
    fn iunamespace_set() {
        let params = new_builder()
            .iunamespace(&[0])
            .iutitle("File:Foo.jpg")
            .data
            .params();
        assert_eq!(params["iunamespace"], "0");
    }

    #[test]
    fn iulimit_set() {
        let params = new_builder()
            .iulimit(50)
            .iutitle("File:Foo.jpg")
            .data
            .params();
        assert_eq!(params["iulimit"], "50");
    }

    #[test]
    fn iuredirect_true() {
        let params = new_builder()
            .iuredirect(true)
            .iutitle("File:Foo.jpg")
            .data
            .params();
        assert!(params.contains_key("iuredirect"));
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let builder = new_builder().iutitle("File:Foo.jpg");
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "imageusage");
    }

    #[tokio::test]
    async fn test_imageusage() {
        use wiremock::matchers::query_param;
        use wiremock::{Mock, ResponseTemplate};
        let server = crate::test_helpers::test_helpers_mod::start_enwiki_mock().await;
        Mock::given(query_param("list", "imageusage"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "imageusage": [
                        {"pageid": 736, "ns": 0, "title": "Albert Einstein"},
                        {"pageid": 500, "ns": 0, "title": "Theory of relativity"}
                    ]
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApiList::imageusage()
            .iutitle("File:Einstein1921 by F Schmutzer 2.jpg")
            .iulimit(5)
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["imageusage"].is_array());
    }
}
