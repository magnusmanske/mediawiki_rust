use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable, NoTitlesOrGenerator, Runnable};
use std::{collections::HashMap, marker::PhantomData};

type NoTarget = NoTitlesOrGenerator;

/// Internal data container for `action=wbgetclaims` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiWbgetclaimsData {
    entity: Option<String>,
    claim: Option<String>,
    property: Option<String>,
    rank: Option<String>,
    props: Option<Vec<String>>,
}

impl ActionApiData for ActionApiWbgetclaimsData {}

impl ActionApiWbgetclaimsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbgetclaims".to_string());
        Self::add_str(&self.entity, "entity", &mut params);
        Self::add_str(&self.claim, "claim", &mut params);
        Self::add_str(&self.property, "property", &mut params);
        Self::add_str(&self.rank, "rank", &mut params);
        Self::add_vec(&self.props, "props", &mut params);
        params
    }
}

/// Builder for the `action=wbgetclaims` API action; uses the typestate pattern to enforce required parameters before execution.
#[derive(Debug, Clone)]
pub struct ActionApiWbgetclaimsBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbgetclaimsData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiWbgetclaimsBuilder<T> {
    /// Filters the returned claims to those using this property ID. `property`
    pub fn property<S: AsRef<str>>(mut self, property: S) -> Self {
        self.data.property = Some(property.as_ref().to_string());
        self
    }

    /// Filters the returned claims by rank (e.g., `normal`, `preferred`, `deprecated`). `rank`
    pub fn rank<S: AsRef<str>>(mut self, rank: S) -> Self {
        self.data.rank = Some(rank.as_ref().to_string());
        self
    }

    /// Sets which additional properties to include in the response (e.g., `references`, `qualifiers`). `props`
    pub fn props<S: Into<String> + Clone>(mut self, props: &[S]) -> Self {
        self.data.props = Some(props.iter().map(|s| s.clone().into()).collect());
        self
    }
}

impl ActionApiWbgetclaimsBuilder<NoTarget> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbgetclaimsData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Sets the entity ID whose claims to retrieve, advancing the builder to the runnable state. `entity`
    pub fn entity<S: AsRef<str>>(mut self, entity: S) -> ActionApiWbgetclaimsBuilder<Runnable> {
        self.data.entity = Some(entity.as_ref().to_string());
        ActionApiWbgetclaimsBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }

    /// Sets a specific claim GUID to retrieve, advancing the builder to the runnable state. `claim`
    pub fn claim<S: AsRef<str>>(mut self, claim: S) -> ActionApiWbgetclaimsBuilder<Runnable> {
        self.data.claim = Some(claim.as_ref().to_string());
        ActionApiWbgetclaimsBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiRunnable for ActionApiWbgetclaimsBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiWbgetclaimsBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiWbgetclaimsBuilder<NoTarget> {
        ActionApiWbgetclaimsBuilder::new()
    }

    #[test]
    fn entity_set() {
        let params = new_builder().entity("Q42").data.params();
        assert_eq!(params["entity"], "Q42");
    }

    #[test]
    fn claim_set() {
        let params = new_builder().claim("Q42$abc").data.params();
        assert_eq!(params["claim"], "Q42$abc");
    }

    #[test]
    fn property_set() {
        let params = new_builder().entity("Q42").property("P31").data.params();
        assert_eq!(params["property"], "P31");
    }

    #[test]
    fn rank_set() {
        let params = new_builder().entity("Q42").rank("normal").data.params();
        assert_eq!(params["rank"], "normal");
    }

    #[test]
    fn action_is_wbgetclaims() {
        let params = new_builder().entity("Q42").data.params();
        assert_eq!(params["action"], "wbgetclaims");
    }
}
