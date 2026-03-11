use super::{ActionApiData, ActionApiRunnable, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub type NoValue = super::NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbformatvalueData {
    datavalue: Option<String>,
    generate: Option<String>,
    datatype: Option<String>,
    property: Option<String>,
    options: Option<String>,
}

impl ActionApiData for ActionApiWbformatvalueData {}

impl ActionApiWbformatvalueData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbformatvalue".to_string());
        Self::add_str(&self.datavalue, "datavalue", &mut params);
        Self::add_str(&self.generate, "generate", &mut params);
        Self::add_str(&self.datatype, "datatype", &mut params);
        Self::add_str(&self.property, "property", &mut params);
        Self::add_str(&self.options, "options", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiWbformatvalueBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbformatvalueData,
}

impl<T> ActionApiWbformatvalueBuilder<T> {
    pub fn generate<S: AsRef<str>>(mut self, generate: S) -> Self {
        self.data.generate = Some(generate.as_ref().to_string());
        self
    }

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
}

impl ActionApiWbformatvalueBuilder<NoValue> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbformatvalueData::default(),
        }
    }

    pub fn datavalue<S: AsRef<str>>(
        mut self,
        datavalue: S,
    ) -> ActionApiWbformatvalueBuilder<Runnable> {
        self.data.datavalue = Some(datavalue.as_ref().to_string());
        ActionApiWbformatvalueBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbformatvalueBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        self.data.params()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiWbformatvalueBuilder<NoValue> {
        ActionApiWbformatvalueBuilder::new()
    }

    #[test]
    fn datavalue_set() {
        let params = new_builder()
            .datavalue(r#"{"type":"string","value":"foo"}"#)
            .data
            .params();
        assert_eq!(params["datavalue"], r#"{"type":"string","value":"foo"}"#);
    }

    #[test]
    fn generate_set() {
        let params = new_builder()
            .datavalue("{}")
            .generate("text/html")
            .data
            .params();
        assert_eq!(params["generate"], "text/html");
    }

    #[test]
    fn datatype_set() {
        let params = new_builder()
            .datavalue("{}")
            .datatype("string")
            .data
            .params();
        assert_eq!(params["datatype"], "string");
    }

    #[test]
    fn action_is_wbformatvalue() {
        let params = new_builder().datavalue("{}").data.params();
        assert_eq!(params["action"], "wbformatvalue");
    }
}
