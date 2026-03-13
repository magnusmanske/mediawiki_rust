use super::{ActionApiContinuable, ActionApiData, ActionApiRunnable};
use std::collections::HashMap;

/// Internal data container for `list=allimages` parameters.
#[derive(Debug, Clone)]
pub struct ActionApiListAllimagesData {
    aifrom: Option<String>,
    aito: Option<String>,
    aiprefix: Option<String>,
    aistart: Option<String>,
    aiend: Option<String>,
    aidir: Option<String>,
    aiminsize: Option<u32>,
    aimaxsize: Option<u32>,
    aishalgorithm: Option<String>,
    aisha1: Option<String>,
    aisha1base36: Option<String>,
    aiprop: Option<Vec<String>>,
    aimime: Option<Vec<String>>,
    ailimit: usize,
    aifilterbots: Option<String>,
}

impl ActionApiData for ActionApiListAllimagesData {}

impl Default for ActionApiListAllimagesData {
    fn default() -> Self {
        Self {
            aifrom: None,
            aito: None,
            aiprefix: None,
            aistart: None,
            aiend: None,
            aidir: None,
            aiminsize: None,
            aimaxsize: None,
            aishalgorithm: None,
            aisha1: None,
            aisha1base36: None,
            aiprop: None,
            aimime: None,
            ailimit: 10,
            aifilterbots: None,
        }
    }
}

impl ActionApiListAllimagesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        Self::add_str(&self.aifrom, "aifrom", &mut params);
        Self::add_str(&self.aito, "aito", &mut params);
        Self::add_str(&self.aiprefix, "aiprefix", &mut params);
        Self::add_str(&self.aistart, "aistart", &mut params);
        Self::add_str(&self.aiend, "aiend", &mut params);
        Self::add_str(&self.aidir, "aidir", &mut params);
        if let Some(v) = self.aiminsize {
            params.insert("aiminsize".to_string(), v.to_string());
        }
        if let Some(v) = self.aimaxsize {
            params.insert("aimaxsize".to_string(), v.to_string());
        }
        Self::add_str(&self.aishalgorithm, "aishalgorithm", &mut params);
        Self::add_str(&self.aisha1, "aisha1", &mut params);
        Self::add_str(&self.aisha1base36, "aisha1base36", &mut params);
        Self::add_vec(&self.aiprop, "aiprop", &mut params);
        Self::add_vec(&self.aimime, "aimime", &mut params);
        params.insert("ailimit".to_string(), self.ailimit.to_string());
        Self::add_str(&self.aifilterbots, "aifilterbots", &mut params);
        params
    }
}

/// Builder for `list=allimages` — enumerates all images.
#[derive(Debug, Clone)]
pub struct ActionApiListAllimagesBuilder {
    pub(crate) data: ActionApiListAllimagesData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl ActionApiListAllimagesBuilder {
    pub(crate) fn new() -> Self {
        Self {
            data: ActionApiListAllimagesData::default(),
            continue_params: HashMap::new(),
        }
    }

    /// Start listing from this filename (`aifrom`).
    pub fn aifrom<S: AsRef<str>>(mut self, aifrom: S) -> Self {
        self.data.aifrom = Some(aifrom.as_ref().to_string());
        self
    }

    /// Stop listing at this filename (`aito`).
    pub fn aito<S: AsRef<str>>(mut self, aito: S) -> Self {
        self.data.aito = Some(aito.as_ref().to_string());
        self
    }

    /// Prefix to search for (`aiprefix`).
    pub fn aiprefix<S: AsRef<str>>(mut self, aiprefix: S) -> Self {
        self.data.aiprefix = Some(aiprefix.as_ref().to_string());
        self
    }

    /// Timestamp to start enumerating from (`aistart`).
    pub fn aistart<S: AsRef<str>>(mut self, aistart: S) -> Self {
        self.data.aistart = Some(aistart.as_ref().to_string());
        self
    }

    /// Timestamp to stop enumerating at (`aiend`).
    pub fn aiend<S: AsRef<str>>(mut self, aiend: S) -> Self {
        self.data.aiend = Some(aiend.as_ref().to_string());
        self
    }

    /// Direction to list (`aidir`).
    pub fn aidir<S: AsRef<str>>(mut self, aidir: S) -> Self {
        self.data.aidir = Some(aidir.as_ref().to_string());
        self
    }

    /// Minimum file size in bytes (`aiminsize`).
    pub fn aiminsize(mut self, aiminsize: u32) -> Self {
        self.data.aiminsize = Some(aiminsize);
        self
    }

    /// Maximum file size in bytes (`aimaxsize`).
    pub fn aimaxsize(mut self, aimaxsize: u32) -> Self {
        self.data.aimaxsize = Some(aimaxsize);
        self
    }

    /// SHA hashing algorithm (`aishalgorithm`).
    pub fn aishalgorithm<S: AsRef<str>>(mut self, aishalgorithm: S) -> Self {
        self.data.aishalgorithm = Some(aishalgorithm.as_ref().to_string());
        self
    }

    /// SHA1 hash to filter by (`aisha1`).
    pub fn aisha1<S: AsRef<str>>(mut self, aisha1: S) -> Self {
        self.data.aisha1 = Some(aisha1.as_ref().to_string());
        self
    }

    /// SHA1 hash in base 36 to filter by (`aisha1base36`).
    pub fn aisha1base36<S: AsRef<str>>(mut self, aisha1base36: S) -> Self {
        self.data.aisha1base36 = Some(aisha1base36.as_ref().to_string());
        self
    }

    /// Properties to return for each image (`aiprop`).
    pub fn aiprop<S: Into<String> + Clone>(mut self, aiprop: &[S]) -> Self {
        self.data.aiprop = Some(aiprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// MIME types to filter by (`aimime`).
    pub fn aimime<S: Into<String> + Clone>(mut self, aimime: &[S]) -> Self {
        self.data.aimime = Some(aimime.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Maximum number of images to return (`ailimit`).
    pub fn ailimit(mut self, ailimit: usize) -> Self {
        self.data.ailimit = ailimit;
        self
    }

    /// Filter bot-uploaded images (`aifilterbots`).
    pub fn aifilterbots<S: AsRef<str>>(mut self, aifilterbots: S) -> Self {
        self.data.aifilterbots = Some(aifilterbots.as_ref().to_string());
        self
    }
}

impl ActionApiRunnable for ActionApiListAllimagesBuilder {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("list".to_string(), "allimages".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiListAllimagesBuilder {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiListAllimagesBuilder {
        ActionApiListAllimagesBuilder::new()
    }

    #[test]
    fn default_ailimit_is_10() {
        let params = new_builder().data.params();
        assert_eq!(params["ailimit"], "10");
    }

    #[test]
    fn default_aifrom_absent() {
        let params = new_builder().data.params();
        assert!(!params.contains_key("aifrom"));
    }

    #[test]
    fn aiprefix_set() {
        let params = new_builder().aiprefix("Logo").data.params();
        assert_eq!(params["aiprefix"], "Logo");
    }

    #[test]
    fn ailimit_set() {
        let params = new_builder().ailimit(50).data.params();
        assert_eq!(params["ailimit"], "50");
    }

    #[test]
    fn aiminsize_set() {
        let params = new_builder().aiminsize(1000).data.params();
        assert_eq!(params["aiminsize"], "1000");
    }

    #[test]
    fn aimaxsize_set() {
        let params = new_builder().aimaxsize(5000).data.params();
        assert_eq!(params["aimaxsize"], "5000");
    }

    #[test]
    fn aiprop_set() {
        let params = new_builder().aiprop(&["timestamp", "user", "url"]).data.params();
        assert_eq!(params["aiprop"], "timestamp|user|url");
    }

    #[test]
    fn runnable_params_contain_action_list() {
        let params = ActionApiRunnable::params(&new_builder());
        assert_eq!(params["action"], "query");
        assert_eq!(params["list"], "allimages");
    }
}
