use super::{ActionApiData, ActionApiRunnable, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub type NoValues = super::NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbparsevalueData {
    values: Option<Vec<String>>,
    datatype: Option<String>,
    property: Option<String>,
    options: Option<String>,
    validate: bool,
}

impl ActionApiData for ActionApiWbparsevalueData {}

impl ActionApiWbparsevalueData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbparsevalue".to_string());
        Self::add_vec(&self.values, "values", &mut params);
        Self::add_str(&self.datatype, "datatype", &mut params);
        Self::add_str(&self.property, "property", &mut params);
        Self::add_str(&self.options, "options", &mut params);
        Self::add_boolean(self.validate, "validate", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiWbparsevalueBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbparsevalueData,
}

impl<T> ActionApiWbparsevalueBuilder<T> {
    pub fn datatype<S: AsRef<str>>(mut self, datatype: S) -> Self {
        self.data.datatype = Some(datatype.as_ref().to_string());
        self
    }

    pub fn property<S: AsRef<str>>(mut self, property: S) -> Self {
        self.data.property = Some(property.as_ref().to_string());
        self
    }

    pub fn options<S: AsRef<str>>(mut self, options: S) -> Self {
        self.data.options = Some(options.as_ref().to_string());
        self
    }

    pub fn validate(mut self, validate: bool) -> Self {
        self.data.validate = validate;
        self
    }
}

impl ActionApiWbparsevalueBuilder<NoValues> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbparsevalueData::default(),
        }
    }

    pub fn values<S: Into<String> + Clone>(
        mut self,
        values: &[S],
    ) -> ActionApiWbparsevalueBuilder<Runnable> {
        self.data.values = Some(values.iter().map(|s| s.clone().into()).collect());
        ActionApiWbparsevalueBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbparsevalueBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        self.data.params()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiWbparsevalueBuilder<NoValues> {
        ActionApiWbparsevalueBuilder::new()
    }

    #[test]
    fn values_single_set() {
        let params = new_builder().values(&["foo"]).data.params();
        assert_eq!(params["values"], "foo");
    }

    #[test]
    fn values_multiple_set() {
        let params = new_builder().values(&["foo", "bar"]).data.params();
        assert_eq!(params["values"], "foo|bar");
    }

    #[test]
    fn datatype_set() {
        let params = new_builder().values(&["foo"]).datatype("string").data.params();
        assert_eq!(params["datatype"], "string");
    }

    #[test]
    fn action_is_wbparsevalue() {
        let params = new_builder().values(&["foo"]).data.params();
        assert_eq!(params["action"], "wbparsevalue");
    }
}
