use super::{ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `meta=allmessages` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiMetaAllmessagesData {
    ammessages: Option<Vec<String>>,
    amprop: Option<Vec<String>>,
    amenableparser: bool,
    amnocontent: bool,
    amincludelocal: bool,
    amargs: Option<Vec<String>>,
    amfilter: Option<String>,
    amcustomised: Option<String>,
    amlang: Option<String>,
    amfrom: Option<String>,
    amto: Option<String>,
    amtitle: Option<String>,
    amprefix: Option<String>,
}

impl ActionApiData for ActionApiMetaAllmessagesData {}

impl ActionApiMetaAllmessagesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_vec(&self.ammessages, "ammessages", &mut params);
        Self::add_vec(&self.amprop, "amprop", &mut params);
        Self::add_boolean(self.amenableparser, "amenableparser", &mut params);
        Self::add_boolean(self.amnocontent, "amnocontent", &mut params);
        Self::add_boolean(self.amincludelocal, "amincludelocal", &mut params);
        Self::add_vec(&self.amargs, "amargs", &mut params);
        Self::add_str(&self.amfilter, "amfilter", &mut params);
        Self::add_str(&self.amcustomised, "amcustomised", &mut params);
        Self::add_str(&self.amlang, "amlang", &mut params);
        Self::add_str(&self.amfrom, "amfrom", &mut params);
        Self::add_str(&self.amto, "amto", &mut params);
        Self::add_str(&self.amtitle, "amtitle", &mut params);
        Self::add_str(&self.amprefix, "amprefix", &mut params);
        params
    }
}

/// Builder for `meta=allmessages` — returns messages from this site.
#[derive(Debug, Clone)]
pub struct ActionApiMetaAllmessagesBuilder {
    pub(crate) data: ActionApiMetaAllmessagesData,
}

impl ActionApiMetaAllmessagesBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiMetaAllmessagesData::default(),
        }
    }

    /// Messages to return (`ammessages`).
    pub fn ammessages<S: Into<String> + Clone>(mut self, ammessages: &[S]) -> Self {
        self.data.ammessages = Some(ammessages.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Additional properties for each message (`amprop`).
    pub fn amprop<S: Into<String> + Clone>(mut self, amprop: &[S]) -> Self {
        self.data.amprop = Some(amprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Enable the parser for messages (`amenableparser`).
    pub fn amenableparser(mut self, amenableparser: bool) -> Self {
        self.data.amenableparser = amenableparser;
        self
    }

    /// Do not include message content in output (`amnocontent`).
    pub fn amnocontent(mut self, amnocontent: bool) -> Self {
        self.data.amnocontent = amnocontent;
        self
    }

    /// Include local messages in the result (`amincludelocal`).
    pub fn amincludelocal(mut self, amincludelocal: bool) -> Self {
        self.data.amincludelocal = amincludelocal;
        self
    }

    /// Arguments for message substitution (`amargs`).
    pub fn amargs<S: Into<String> + Clone>(mut self, amargs: &[S]) -> Self {
        self.data.amargs = Some(amargs.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Return only messages with names that contain this string (`amfilter`).
    pub fn amfilter<S: AsRef<str>>(mut self, amfilter: S) -> Self {
        self.data.amfilter = Some(amfilter.as_ref().to_string());
        self
    }

    /// Return only messages that are or are not customised (`amcustomised`).
    pub fn amcustomised<S: AsRef<str>>(mut self, amcustomised: S) -> Self {
        self.data.amcustomised = Some(amcustomised.as_ref().to_string());
        self
    }

    /// Return messages in this language (`amlang`).
    pub fn amlang<S: AsRef<str>>(mut self, amlang: S) -> Self {
        self.data.amlang = Some(amlang.as_ref().to_string());
        self
    }

    /// Return messages starting at this message (`amfrom`).
    pub fn amfrom<S: AsRef<str>>(mut self, amfrom: S) -> Self {
        self.data.amfrom = Some(amfrom.as_ref().to_string());
        self
    }

    /// Return messages ending at this message (`amto`).
    pub fn amto<S: AsRef<str>>(mut self, amto: S) -> Self {
        self.data.amto = Some(amto.as_ref().to_string());
        self
    }

    /// Title of a page to use as context when parsing messages (`amtitle`).
    pub fn amtitle<S: AsRef<str>>(mut self, amtitle: S) -> Self {
        self.data.amtitle = Some(amtitle.as_ref().to_string());
        self
    }

    /// Return messages with this prefix (`amprefix`).
    pub fn amprefix<S: AsRef<str>>(mut self, amprefix: S) -> Self {
        self.data.amprefix = Some(amprefix.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiMetaAllmessagesBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("meta".to_string(), "allmessages".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiMetaAllmessagesBuilder {
        ActionApiMetaAllmessagesBuilder::new()
    }

    #[test]
    fn default_ammessages_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("ammessages"));
    }

    #[test]
    fn ammessages_set() {
        let params = new_builder().ammessages(&["edit", "delete"]).data.params();
        assert_eq!(params["ammessages"], "edit|delete");
    }

    #[test]
    fn amenableparser_flag() {
        let params = new_builder().amenableparser(true).data.params();
        assert_eq!(params["amenableparser"], "");
    }

    #[test]
    fn amnocontent_flag() {
        let params = new_builder().amnocontent(true).data.params();
        assert_eq!(params["amnocontent"], "");
    }

    #[test]
    fn amfilter_set() {
        let params = new_builder().amfilter("edit").data.params();
        assert_eq!(params["amfilter"], "edit");
    }

    #[test]
    fn amlang_set() {
        let params = new_builder().amlang("de").data.params();
        assert_eq!(params["amlang"], "de");
    }

    #[test]
    fn amprefix_set() {
        let params = new_builder().amprefix("edit").data.params();
        assert_eq!(params["amprefix"], "edit");
    }

    #[test]
    fn runnable_params_contain_action_meta() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["meta"], "allmessages");
    }
}
