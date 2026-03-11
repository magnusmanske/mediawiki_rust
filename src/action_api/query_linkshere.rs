use std::{collections::HashMap, marker::PhantomData};

use crate::{action_api::ActionApiRunnable, api::NamespaceID};

#[derive(Debug, Clone)]
pub struct ActionApiQueryLinkshereData {
    titles: Vec<String>,
    lhprop: Option<Vec<String>>,
    lhnamespace: Option<Vec<NamespaceID>>,
    lhshow: Option<Vec<String>>,
    lhlimit: usize,
    lhcontinue: Option<String>,
}

impl Default for ActionApiQueryLinkshereData {
    fn default() -> Self {
        Self {
            titles: Vec::new(),
            lhprop: None,
            lhnamespace: None,
            lhshow: None,
            lhlimit: 10,
            lhcontinue: None,
        }
    }
}

impl ActionApiQueryLinkshereData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(lhnamespace) = &self.lhnamespace {
            let lhnamespace: Vec<String> = lhnamespace.iter().map(|n| n.to_string()).collect();
            params.insert("lhnamespace".to_string(), lhnamespace.join("|"));
        }
        params.insert("titles".to_string(), self.titles.join("|"));
        if let Some(lhprop) = &self.lhprop {
            params.insert("lhprop".to_string(), lhprop.join("|"));
        }
        if let Some(lhshow) = &self.lhshow {
            params.insert("lhshow".to_string(), lhshow.join("|"));
        }
        params.insert("lhlimit".to_string(), self.lhlimit.to_string());
        if let Some(lhcontinue) = &self.lhcontinue {
            params.insert("lhcontinue".to_string(), lhcontinue.clone());
        }
        params
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct ActionApiQueryLinkshereBuilder<T> {
    _phantom: PhantomData<T>,
    data: ActionApiQueryLinkshereData,
}

impl<T> ActionApiQueryLinkshereBuilder<T> {
    pub fn lhprop<S: Into<String> + Clone>(mut self, lhprop: &[S]) -> Self {
        self.data.lhprop = Some(
            lhprop
                .iter()
                .map(|s| s.clone().into())
                .collect::<Vec<String>>(),
        );
        self
    }

    pub fn lhnamespace(mut self, lhnamespace: &[NamespaceID]) -> Self {
        self.data.lhnamespace = Some(lhnamespace.to_vec());
        self
    }

    pub fn lhshow<S: Into<String> + Clone>(mut self, lhshow: &[S]) -> Self {
        self.data.lhshow = Some(
            lhshow
                .iter()
                .map(|s| s.clone().into())
                .collect::<Vec<String>>(),
        );
        self
    }

    pub fn lhlimit(mut self, lhlimit: usize) -> Self {
        self.data.lhlimit = lhlimit;
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NoTitlesOrGenerator;

#[derive(Debug, Copy, Clone)]
pub struct Runnable;

impl<NoTitlesOrGenerator> ActionApiQueryLinkshereBuilder<NoTitlesOrGenerator> {
    pub(crate) fn new() -> ActionApiQueryLinkshereBuilder<NoTitlesOrGenerator> {
        ActionApiQueryLinkshereBuilder {
            _phantom: PhantomData,
            data: ActionApiQueryLinkshereData::default(),
        }
    }

    pub fn titles<S: Into<String> + Clone>(
        mut self,
        titles: &[S],
    ) -> ActionApiQueryLinkshereBuilder<Runnable> {
        self.data.titles = titles.iter().map(|s| s.clone().into()).collect();
        ActionApiQueryLinkshereBuilder {
            _phantom: PhantomData,
            data: self.data,
        }
    }
}

impl<Runnable> ActionApiRunnable for ActionApiQueryLinkshereBuilder<Runnable> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "query".to_string());
        ret.insert("prop".to_string(), "linkshere".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Api, action_api::ActionApiQuery};

    #[tokio::test]
    async fn test_linkshere() {
        let api = Api::new("https://en.wikipedia.org/w/api.php")
            .await
            .unwrap();
        let result = ActionApiQuery::linkshere()
            .titles(&["Magnus Manske"])
            .lhnamespace(&[0])
            .run(&api)
            .await
            .unwrap();
        assert!(result["query"]["pages"]["3361346"]["linkshere"].is_array())
    }
}
