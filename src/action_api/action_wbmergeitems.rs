use super::{ActionApiData, ActionApiRunnable, NoToken, Runnable};
use std::{collections::HashMap, marker::PhantomData};

pub type NoSource = super::NoTitlesOrGenerator;

#[derive(Debug, Clone, Default)]
pub struct ActionApiWbmergeitemsData {
    fromid: Option<String>,
    toid: Option<String>,
    ignoreconflicts: Option<Vec<String>>,
    summary: Option<String>,
    tags: Option<Vec<String>>,
    bot: bool,
    token: Option<String>,
}

impl ActionApiData for ActionApiWbmergeitemsData {}

impl ActionApiWbmergeitemsData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("action".to_string(), "wbmergeitems".to_string());
        Self::add_str(&self.fromid, "fromid", &mut params);
        Self::add_str(&self.toid, "toid", &mut params);
        Self::add_vec(&self.ignoreconflicts, "ignoreconflicts", &mut params);
        Self::add_str(&self.summary, "summary", &mut params);
        Self::add_vec(&self.tags, "tags", &mut params);
        Self::add_boolean(self.bot, "bot", &mut params);
        Self::add_str(&self.token, "token", &mut params);
        params
    }
}

#[derive(Debug, Clone)]
pub struct ActionApiWbmergeitemsBuilder<T> {
    _phantom: PhantomData<T>,
    pub(crate) data: ActionApiWbmergeitemsData,
}

impl<T> ActionApiWbmergeitemsBuilder<T> {
    pub fn toid<S: AsRef<str>>(mut self, toid: S) -> Self {
        self.data.toid = Some(toid.as_ref().to_string());
        self
    }

    pub fn ignoreconflicts<S: Into<String> + Clone>(mut self, ignoreconflicts: &[S]) -> Self {
        self.data.ignoreconflicts =
            Some(ignoreconflicts.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn summary<S: AsRef<str>>(mut self, summary: S) -> Self {
        self.data.summary = Some(summary.as_ref().to_string());
        self
    }

    pub fn tags<S: Into<String> + Clone>(mut self, tags: &[S]) -> Self {
        self.data.tags = Some(tags.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn bot(mut self, bot: bool) -> Self {
        self.data.bot = bot;
        self
    }

}

impl ActionApiWbmergeitemsBuilder<NoSource> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            data: ActionApiWbmergeitemsData::default(),
        }
    }

    pub fn fromid<S: AsRef<str>>(mut self, fromid: S) -> ActionApiWbmergeitemsBuilder<NoToken> {
        self.data.fromid = Some(fromid.as_ref().to_string());
        ActionApiWbmergeitemsBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiWbmergeitemsBuilder<NoToken> {
    pub fn token<S: AsRef<str>>(mut self, token: S) -> ActionApiWbmergeitemsBuilder<Runnable> {
        self.data.token = Some(token.as_ref().to_string());
        ActionApiWbmergeitemsBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl ActionApiRunnable for ActionApiWbmergeitemsBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        self.data.params()
    }

    fn http_method(&self) -> &'static str {
        "POST"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiWbmergeitemsBuilder<NoSource> {
        ActionApiWbmergeitemsBuilder::new()
    }

    #[test]
    fn fromid_set() {
        let params = new_builder().fromid("Q1").data.params();
        assert_eq!(params["fromid"], "Q1");
    }

    #[test]
    fn toid_set() {
        let params = new_builder().fromid("Q1").toid("Q2").data.params();
        assert_eq!(params["toid"], "Q2");
    }

    #[test]
    fn ignoreconflicts_set() {
        let params = new_builder()
            .fromid("Q1")
            .ignoreconflicts(&["label", "description"])
            .data
            .params();
        assert_eq!(params["ignoreconflicts"], "label|description");
    }

    #[test]
    fn bot_flag() {
        let params = new_builder().fromid("Q1").bot(true).data.params();
        assert_eq!(params["bot"], "");
    }

    #[test]
    fn token_set() {
        let params = new_builder().fromid("Q1").token("csrf+\\").data.params();
        assert_eq!(params["token"], "csrf+\\");
    }

    #[test]
    fn action_is_wbmergeitems() {
        let params = new_builder().fromid("Q1").data.params();
        assert_eq!(params["action"], "wbmergeitems");
    }

    #[test]
    fn http_method_is_post() {
        let builder = new_builder().fromid("Q1").token("csrf");
        assert_eq!(builder.http_method(), "POST");
    }
}
