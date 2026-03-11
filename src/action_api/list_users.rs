use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone, Default)]
pub struct ActionApiListUsersData {
    usprop: Option<Vec<String>>,
    usattachedwiki: Option<String>,
    ususers: Option<Vec<String>>,
    ususerids: Option<Vec<u64>>,
}

impl ActionApiData for ActionApiListUsersData {}

impl ActionApiListUsersData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_vec(&self.usprop, "usprop", &mut params);
        Self::add_str(&self.usattachedwiki, "usattachedwiki", &mut params);
        Self::add_vec(&self.ususers, "ususers", &mut params);
        if let Some(ususerids) = &self.ususerids {
            let s: Vec<String> = ususerids.iter().map(|id| id.to_string()).collect();
            params.insert("ususerids".to_string(), s.join("|"));
        }
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiListUsersBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiListUsersData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiListUsersBuilder<T> {
    pub fn usprop<S: Into<String> + Clone>(mut self, usprop: &[S]) -> Self {
        self.data.usprop = Some(usprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn usattachedwiki<S: AsRef<str>>(mut self, usattachedwiki: S) -> Self {
        self.data.usattachedwiki = Some(usattachedwiki.as_ref().to_string());
        self
    }
}

impl ActionApiListUsersBuilder<NoTitlesOrGenerator> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiListUsersData::default(),
            continue_params: HashMap::new(),
        }
    }

    pub fn ususers<S: Into<String> + Clone>(
        mut self,
        ususers: &[S],
    ) -> ActionApiListUsersBuilder<Runnable> {
        self.data.ususers = Some(ususers.iter().map(|s| s.clone().into()).collect());
        ActionApiListUsersBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }

    pub fn ususerids(mut self, ususerids: &[u64]) -> ActionApiListUsersBuilder<Runnable> {
        self.data.ususerids = Some(ususerids.to_vec());
        ActionApiListUsersBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiRunnable for ActionApiListUsersBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "users".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListUsersBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiList};

    fn new_builder() -> ActionApiListUsersBuilder<NoTitlesOrGenerator> {
        ActionApiListUsersBuilder::new()
    }

    #[test]
    fn default_usprop_absent() {
        let params = new_builder().ususers(&["ExampleUser"]).data.params();
        assert!(!params.contains_key("usprop"));
    }

    #[test]
    fn ususers_single() {
        let params = new_builder().ususers(&["ExampleUser"]).data.params();
        assert_eq!(params["ususers"], "ExampleUser");
    }

    #[test]
    fn ususers_multiple() {
        let params = new_builder().ususers(&["Alice", "Bob"]).data.params();
        assert_eq!(params["ususers"], "Alice|Bob");
    }

    #[test]
    fn ususerids_set() {
        let params = new_builder().ususerids(&[123, 456]).data.params();
        assert_eq!(params["ususerids"], "123|456");
    }

    #[test]
    fn ususers_does_not_set_ususerids() {
        let params = new_builder().ususers(&["Foo"]).data.params();
        assert!(!params.contains_key("ususerids"));
    }

    #[test]
    fn usprop_set() {
        let params = new_builder()
            .usprop(&["groups", "editcount", "registration"])
            .ususers(&["Foo"])
            .data
            .params();
        assert_eq!(params["usprop"], "groups|editcount|registration");
    }

    #[test]
    fn usattachedwiki_set() {
        let params = new_builder()
            .usprop(&["centralids"])
            .usattachedwiki("enwiki")
            .ususers(&["Foo"])
            .data
            .params();
        assert_eq!(params["usattachedwiki"], "enwiki");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let builder = new_builder().ususers(&["ExampleUser"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "users");
    }

    #[tokio::test]
    async fn test_users() {
        let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
        let result = ActionApiList::users()
            .ususers(&["Magnus Manske"])
            .usprop(&["groups", "editcount"])
            .run(&api)
            .await
            .unwrap();
        let users = result["query"]["users"].as_array().unwrap();
        assert!(!users.is_empty());
    }
}
