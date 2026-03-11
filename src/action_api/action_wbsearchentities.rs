use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub type NoSearch = super::NoTitlesOrGenerator;

/// Internal data container for `action=wbsearchentities` parameters.
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

/// Builder for the `action=wbsearchentities` API action; uses the typestate pattern to enforce the required search term.
#[derive(Debug, Clone)]
pub struct ActionApiWbsearchentitiesBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbsearchentitiesData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiWbsearchentitiesBuilder<T> {
    /// Sets the language of labels and descriptions to search in. `language`
    pub fn language<S: AsRef<str>>(mut self, language: S) -> Self {
        self.data.language = Some(language.as_ref().to_string());
        self
    }

    /// Restricts search results to entities whose label matches the language exactly. `strictlanguage`
    pub fn strictlanguage(mut self, strictlanguage: bool) -> Self {
        self.data.strictlanguage = strictlanguage;
        self
    }

    /// Sets the type of entity to search for (e.g. `item` or `property`). `type`
    pub fn entity_type<S: AsRef<str>>(mut self, entity_type: S) -> Self {
        self.data.entity_type = Some(entity_type.as_ref().to_string());
        self
    }

    /// Sets the maximum number of results to return. `limit`
    pub fn limit(mut self, limit: usize) -> Self {
        self.data.limit = limit;
        self
    }

    /// Sets the offset into the search results to continue from. `continue`
    pub fn search_continue(mut self, search_continue: usize) -> Self {
        self.data.search_continue = search_continue;
        self
    }

    /// Sets the properties to include in the response. `props`
    pub fn props<S: Into<String> + Clone>(mut self, props: &[S]) -> Self {
        self.data.props = Some(props.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Sets the search profile to use for ranking results. `profile`
    pub fn profile<S: AsRef<str>>(mut self, profile: S) -> Self {
        self.data.profile = Some(profile.as_ref().to_string());
        self
    }
}

impl ActionApiWbsearchentitiesBuilder<NoSearch> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbsearchentitiesData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Sets the search string to look up in entity labels and aliases. `search`
    pub fn search<S: AsRef<str>>(
        mut self,
        search: S,
    ) -> ActionApiWbsearchentitiesBuilder<Runnable> {
        self.data.search = Some(search.as_ref().to_string());
        ActionApiWbsearchentitiesBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiRunnable for ActionApiWbsearchentitiesBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiWbsearchentitiesBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
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
