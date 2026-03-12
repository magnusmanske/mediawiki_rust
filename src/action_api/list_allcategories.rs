use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `list=allcategories` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListAllcategoriesData {
    acfrom: Option<String>,
    accontinue: Option<String>,
    acto: Option<String>,
    acprefix: Option<String>,
    acdir: Option<String>,
    acmin: Option<u32>,
    acmax: Option<u32>,
    aclimit: usize,
    acprop: Option<Vec<String>>,
}

impl ActionApiData for ActionApiListAllcategoriesData {}

impl Default for ActionApiListAllcategoriesData {
    fn default() -> Self {
        Self {
            acfrom: None,
            accontinue: None,
            acto: None,
            acprefix: None,
            acdir: None,
            acmin: None,
            acmax: None,
            aclimit: 10,
            acprop: None,
        }
    }
}

impl ActionApiListAllcategoriesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.acfrom, "acfrom", &mut params);
        Self::add_str(&self.accontinue, "accontinue", &mut params);
        Self::add_str(&self.acto, "acto", &mut params);
        Self::add_str(&self.acprefix, "acprefix", &mut params);
        Self::add_str(&self.acdir, "acdir", &mut params);
        if let Some(acmin) = self.acmin {
            params.insert("acmin".to_string(), acmin.to_string());
        }
        if let Some(acmax) = self.acmax {
            params.insert("acmax".to_string(), acmax.to_string());
        }
        params.insert("aclimit".to_string(), self.aclimit.to_string());
        Self::add_vec(&self.acprop, "acprop", &mut params);
        params
    }
}

/// Builder for the `list=allcategories` API module; supports pagination via `ActionApiContinuable`.
#[derive(Debug, Clone)]
pub struct ActionApiListAllcategoriesBuilder {
    pub(crate) data: ActionApiListAllcategoriesData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListAllcategoriesBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            data: ActionApiListAllcategoriesData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Start listing from this category title (`acfrom`).
    pub fn acfrom<S: AsRef<str>>(mut self, acfrom: S) -> Self {
        self.data.acfrom = Some(acfrom.as_ref().to_string());
        self
    }

    /// Stop listing at this category title (`acto`).
    pub fn acto<S: AsRef<str>>(mut self, acto: S) -> Self {
        self.data.acto = Some(acto.as_ref().to_string());
        self
    }

    /// Filter categories to those starting with this prefix (`acprefix`).
    pub fn acprefix<S: AsRef<str>>(mut self, acprefix: S) -> Self {
        self.data.acprefix = Some(acprefix.as_ref().to_string());
        self
    }

    /// Sort direction for listing (`acdir`).
    pub fn acdir<S: AsRef<str>>(mut self, acdir: S) -> Self {
        self.data.acdir = Some(acdir.as_ref().to_string());
        self
    }

    /// Only return categories with at least this many members (`acmin`).
    pub fn acmin(mut self, acmin: u32) -> Self {
        self.data.acmin = Some(acmin);
        self
    }

    /// Only return categories with at most this many members (`acmax`).
    pub fn acmax(mut self, acmax: u32) -> Self {
        self.data.acmax = Some(acmax);
        self
    }

    /// Maximum number of categories to return (`aclimit`).
    pub fn aclimit(mut self, aclimit: usize) -> Self {
        self.data.aclimit = aclimit;
        self
    }

    /// Properties to retrieve for each category (`acprop`).
    pub fn acprop<S: Into<String> + Clone>(mut self, acprop: &[S]) -> Self {
        self.data.acprop = Some(acprop.iter().map(|s| s.clone().into()).collect());
        self
    }
}

impl ActionApiRunnable for ActionApiListAllcategoriesBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "allcategories".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListAllcategoriesBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiList};

    fn new_builder() -> ActionApiListAllcategoriesBuilder {
        ActionApiListAllcategoriesBuilder::new()
    }

    #[test]
    fn default_aclimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["aclimit"], "10");
    }

    #[test]
    fn default_acfrom_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("acfrom"));
    }

    #[test]
    fn acprefix_set() {
        let params = new_builder().acprefix("Phy").data.params();
        assert_eq!(params["acprefix"], "Phy");
    }

    #[test]
    fn acmin_set() {
        let params = new_builder().acmin(5).data.params();
        assert_eq!(params["acmin"], "5");
    }

    #[test]
    fn acmax_set() {
        let params = new_builder().acmax(100).data.params();
        assert_eq!(params["acmax"], "100");
    }

    #[test]
    fn aclimit_set() {
        let params = new_builder().aclimit(50).data.params();
        assert_eq!(params["aclimit"], "50");
    }

    #[test]
    fn acprop_set() {
        let params = new_builder().acprop(&["size", "hidden"]).data.params();
        assert_eq!(params["acprop"], "size|hidden");
    }

    #[test]
    fn acdir_descending() {
        let params = new_builder().acdir("descending").data.params();
        assert_eq!(params["acdir"], "descending");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "allcategories");
    }

    #[tokio::test]
    async fn test_allcategories() {
        use wiremock::matchers::query_param;
        use wiremock::{Mock, ResponseTemplate};
        let server = crate::test_helpers::test_helpers_mod::start_enwiki_mock().await;
        Mock::given(query_param("list", "allcategories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "allcategories": [
                        {"*": "Physics", "size": 100, "subcats": 20, "files": 5},
                        {"*": "Physics and society", "size": 50, "subcats": 10, "files": 2}
                    ]
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApiList::allcategories()
            .acprefix("Physics")
            .aclimit(5)
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["allcategories"].is_array());
    }
}
