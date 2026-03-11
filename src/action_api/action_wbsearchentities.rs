use super::{ActionApiData, ActionApiRunnable, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub type NoSearch = super::NoTitlesOrGenerator;

#[derive(Debug, Clone)]
pub struct ActionApiWbsearchentitiesData {
    search: Option<String>,
    language: Option<String>,
    strictlanguage: bool,
    entity_type: Option<String>,
    limit: usize,
    search_continue: usize,
    props: Option<Vec<String>>,
    profile: Option<String>,
}

impl Default for ActionApiWbsearchentitiesData {
    fn default() -> Self {
        Self {
            search: None,
            language: None,
            strictlanguage: false,
            entity_type: None,
            limit: 7,
            search_continue: 0,
            props: None,
            profile: None,
        }
    }
}

impl ActionApiData for ActionApiWbsearchentitiesData {}

impl ActionApiWbsearchentitiesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbsearchentities".to_string());
        Self::add_str(&self.search, "search", &mut params);
        Self::add_str(&self.language, "language", &mut params);
        Self::add_boolean(self.strictlanguage, "strictlanguage", &mut params);
        Self::add_str(&self.entity_type, "type", &mut params);
        params.insert("limit".to_string(), self.limit.to_string());
        if self.search_continue > 0 {
            params.insert("continue".to_string(), self.search_continue.to_string());
        }
        Self::add_vec(&self.props, "props", &mut params);
        Self::add_str(&self.profile, "profile", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiWbsearchentitiesBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbsearchentitiesData,
}

impl<T> ActionApiWbsearchentitiesBuilder<T> {
    pub fn language<S: AsRef<str>>(mut self, language: S) -> Self {
        self.data.language = Some(language.as_ref().to_string());
        self
    }

    pub fn strictlanguage(mut self, strictlanguage: bool) -> Self {
        self.data.strictlanguage = strictlanguage;
        self
    }

    pub fn entity_type<S: AsRef<str>>(mut self, entity_type: S) -> Self {
        self.data.entity_type = Some(entity_type.as_ref().to_string());
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.data.limit = limit;
        self
    }

    pub fn search_continue(mut self, search_continue: usize) -> Self {
        self.data.search_continue = search_continue;
        self
    }

    pub fn props<S: Into<String> + Clone>(mut self, props: &[S]) -> Self {
        self.data.props = Some(props.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn profile<S: AsRef<str>>(mut self, profile: S) -> Self {
        self.data.profile = Some(profile.as_ref().to_string());
        self
    }
}

impl ActionApiWbsearchentitiesBuilder<NoSearch> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbsearchentitiesData::default(),
        }
    }

    pub fn search<S: AsRef<str>>(
        mut self,
        search: S,
    ) -> ActionApiWbsearchentitiesBuilder<Runnable> {
        self.data.search = Some(search.as_ref().to_string());
        ActionApiWbsearchentitiesBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbsearchentitiesBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        self.data.params()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiWbsearchentitiesBuilder<NoSearch> {
        ActionApiWbsearchentitiesBuilder::new()
    }

    #[test]
    fn search_set() {
        let params = new_builder().search("Douglas Adams").data.params();
        assert_eq!(params["search"], "Douglas Adams");
    }

    #[test]
    fn language_set() {
        let params = new_builder().search("foo").language("en").data.params();
        assert_eq!(params["language"], "en");
    }

    #[test]
    fn entity_type_set() {
        let params = new_builder().search("foo").entity_type("property").data.params();
        assert_eq!(params["type"], "property");
    }

    #[test]
    fn limit_set() {
        let params = new_builder().search("foo").limit(20).data.params();
        assert_eq!(params["limit"], "20");
    }

    #[test]
    fn action_is_wbsearchentities() {
        let params = new_builder().search("foo").data.params();
        assert_eq!(params["action"], "wbsearchentities");
    }
}
