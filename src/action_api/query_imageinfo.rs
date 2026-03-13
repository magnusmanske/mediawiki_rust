use super::{
    ActionApiContinuable, ActionApiData, ActionApiQueryCommonBuilder, ActionApiQueryCommonData,
    ActionApiRunnable, NoTitlesOrGenerator, Runnable,
};
use std::{collections::HashMap, marker::PhantomData};

/// Internal data container for `prop=imageinfo` parameters.
#[derive(Debug, Clone, Default)]
pub struct ActionApiQueryImageinfoData {
    common: ActionApiQueryCommonData,
    iiprop: Option<Vec<String>>,
    iilimit: Option<usize>,
    iistart: Option<String>,
    iiend: Option<String>,
    iiurlwidth: Option<i32>,
    iiurlheight: Option<i32>,
    iiurlparam: Option<String>,
    iibadfilecontexttitle: Option<String>,
    iilocal: bool,
    iicontinue: Option<String>,
}

impl ActionApiData for ActionApiQueryImageinfoData {}

impl ActionApiQueryImageinfoData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        self.common.add_to_params(&mut params);
        Self::add_vec(&self.iiprop, "iiprop", &mut params);
        if let Some(v) = self.iilimit {
            params.insert("iilimit".to_string(), v.to_string());
        }
        Self::add_str(&self.iistart, "iistart", &mut params);
        Self::add_str(&self.iiend, "iiend", &mut params);
        if let Some(v) = self.iiurlwidth {
            params.insert("iiurlwidth".to_string(), v.to_string());
        }
        if let Some(v) = self.iiurlheight {
            params.insert("iiurlheight".to_string(), v.to_string());
        }
        Self::add_str(&self.iiurlparam, "iiurlparam", &mut params);
        Self::add_str(&self.iibadfilecontexttitle, "iibadfilecontexttitle", &mut params);
        Self::add_boolean(self.iilocal, "iilocal", &mut params);
        Self::add_str(&self.iicontinue, "iicontinue", &mut params);
        params
    }
}

/// Builder for `prop=imageinfo` — returns file information and upload history.
#[derive(Debug, Clone)]
pub struct ActionApiQueryImageinfoBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiQueryImageinfoData,
    pub(crate) continue_params: HashMap<String, String>,
}

impl<T> ActionApiQueryImageinfoBuilder<T> {
    /// Properties to return for each file version (`iiprop`).
    pub fn iiprop<S: Into<String> + Clone>(mut self, iiprop: &[S]) -> Self {
        self.data.iiprop = Some(iiprop.iter().map(|s| s.clone().into()).collect());
        self
    }

    /// Maximum number of file revisions to return (`iilimit`).
    pub fn iilimit(mut self, iilimit: usize) -> Self {
        self.data.iilimit = Some(iilimit);
        self
    }

    /// Timestamp to start enumerating from (`iistart`).
    pub fn iistart<S: AsRef<str>>(mut self, iistart: S) -> Self {
        self.data.iistart = Some(iistart.as_ref().to_string());
        self
    }

    /// Timestamp to stop enumerating at (`iiend`).
    pub fn iiend<S: AsRef<str>>(mut self, iiend: S) -> Self {
        self.data.iiend = Some(iiend.as_ref().to_string());
        self
    }

    /// Thumbnail width in pixels (`iiurlwidth`).
    pub fn iiurlwidth(mut self, iiurlwidth: i32) -> Self {
        self.data.iiurlwidth = Some(iiurlwidth);
        self
    }

    /// Thumbnail height in pixels (`iiurlheight`).
    pub fn iiurlheight(mut self, iiurlheight: i32) -> Self {
        self.data.iiurlheight = Some(iiurlheight);
        self
    }

    /// Thumbnail parameter for video and audio (`iiurlparam`).
    pub fn iiurlparam<S: AsRef<str>>(mut self, iiurlparam: S) -> Self {
        self.data.iiurlparam = Some(iiurlparam.as_ref().to_string());
        self
    }

    /// Title of the page to use as context for bad file checking (`iibadfilecontexttitle`).
    pub fn iibadfilecontexttitle<S: AsRef<str>>(mut self, iibadfilecontexttitle: S) -> Self {
        self.data.iibadfilecontexttitle = Some(iibadfilecontexttitle.as_ref().to_string());
        self
    }

    /// Also look for the file in the local repository (`iilocal`).
    pub fn iilocal(mut self, iilocal: bool) -> Self {
        self.data.iilocal = iilocal;
        self
    }
}

impl ActionApiQueryImageinfoBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiQueryImageinfoData::default(),
            continue_params: HashMap::new(),
        }
    }
}

impl ActionApiQueryCommonBuilder for ActionApiQueryImageinfoBuilder<NoTitlesOrGenerator> {
    type Runnable = ActionApiQueryImageinfoBuilder<Runnable>;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData {
        &mut self.data.common
    }

    fn into_runnable(self) -> Self::Runnable {
        ActionApiQueryImageinfoBuilder {
            _phantom: PhantomData,
            data: self.data,
            continue_params: self.continue_params,
        }
    }
}

impl ActionApiRunnable for ActionApiQueryImageinfoBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "imageinfo".to_string());
        ret.extend(self.continue_params.clone());
        ret
    }
}

impl ActionApiContinuable for ActionApiQueryImageinfoBuilder<Runnable> {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.continue_params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action_api::{ActionApiQueryCommonBuilder, NoTitlesOrGenerator};

    fn new_builder() -> ActionApiQueryImageinfoBuilder<NoTitlesOrGenerator> {
        ActionApiQueryImageinfoBuilder::new()
    }

    #[test]
    fn default_iiprop_absent() {
        let params = new_builder().titles(&["File:Test.png"]).data.params();
        assert!(!params.contains_key("iiprop"));
    }

    #[test]
    fn iiprop_set() {
        let params = new_builder()
            .iiprop(&["timestamp", "user", "url"])
            .titles(&["File:Test.png"])
            .data
            .params();
        assert_eq!(params["iiprop"], "timestamp|user|url");
    }

    #[test]
    fn iilimit_set() {
        let params = new_builder()
            .iilimit(5)
            .titles(&["File:Test.png"])
            .data
            .params();
        assert_eq!(params["iilimit"], "5");
    }

    #[test]
    fn iiurlwidth_set() {
        let params = new_builder()
            .iiurlwidth(300)
            .titles(&["File:Test.png"])
            .data
            .params();
        assert_eq!(params["iiurlwidth"], "300");
    }

    #[test]
    fn iilocal_flag() {
        let params = new_builder()
            .iilocal(true)
            .titles(&["File:Test.png"])
            .data
            .params();
        assert_eq!(params["iilocal"], "");
    }

    #[test]
    fn runnable_params_contain_action_prop() {
        let builder = new_builder().titles(&["File:Test.png"]);
        let params = ActionApiRunnable::params(&builder);
        assert_eq!(params["action"], "query");
        assert_eq!(params["prop"], "imageinfo");
    }
}
