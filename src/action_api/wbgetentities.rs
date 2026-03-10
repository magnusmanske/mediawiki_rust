use std::{collections::HashMap, marker::PhantomData};

use serde_json::Value;

use crate::{Api, ApiSync, MediaWikiError};

#[derive(Debug, Clone, Copy)]
pub struct NoTitles;

#[derive(Debug, Clone)]
pub struct ActionApiWbGetEntitiesData {
    ids: Option<WbGetEntitiesTitles>,
    redirects: bool,
    props: Vec<String>,
    languages: Vec<String>,
    languagefallback: bool,
    normalize: bool,
    sitefilter: Vec<String>,
}

impl Default for ActionApiWbGetEntitiesData {
    fn default() -> Self {
        Self {
            ids: None,
            redirects: true,
            props: Vec::new(),
            languages: Vec::new(),
            languagefallback: false,
            normalize: false,
            sitefilter: Vec::new(),
        }
    }
}

impl ActionApiWbGetEntitiesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(ids) = &self.ids {
            ids.params(&mut params);
        }
        params.insert("redirects".to_string(), self.redirects.to_string());
        params.insert("props".to_string(), self.props.join("|"));
        params.insert("languages".to_string(), self.languages.join("|"));
        params.insert("languagefallback".to_string(), String::new());
        params.insert("normalize".to_string(), String::new());
        params.insert("sitefilter".to_string(), self.sitefilter.join("|"));
        params
    }

    pub(crate) fn with_ids(mut self, ids: WbGetEntitiesTitles) -> Self {
        self.ids = Some(ids);
        self
    }
}

#[derive(Debug, Clone)]
pub enum WbGetEntitiesTitles {
    Ids(Vec<String>),
    SiteTitles { site: String, titles: Vec<String> },
    SitesTitle { sites: Vec<String>, title: String },
}

impl WbGetEntitiesTitles {
    pub(crate) fn params(&self, params: &mut HashMap<String, String>) {
        match self {
            WbGetEntitiesTitles::Ids(ids) => {
                params.insert("ids".to_string(), ids.join("|"));
            }
            WbGetEntitiesTitles::SiteTitles { site, titles } => {
                params.insert("sites".to_string(), site.clone());
                params.insert("titles".to_string(), titles.join("|"));
            }
            WbGetEntitiesTitles::SitesTitle { sites, title } => {
                params.insert("sites".to_string(), sites.join("|"));
                params.insert("titles".to_string(), title.clone());
            }
        }
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ActionApiWbGetEntitiesBuilder<T> {
    _phantom: PhantomData<T>,
    data: ActionApiWbGetEntitiesData,
}

impl<T> ActionApiWbGetEntitiesBuilder<T> {
    pub fn redirects(mut self, redirects: bool) -> Self {
        self.data.redirects = redirects;
        self
    }

    pub fn props<S: Into<String> + Clone>(mut self, props: &[S]) -> Self {
        self.data.props = props
            .iter()
            .map(|s| s.clone().into())
            .collect::<Vec<String>>();
        self
    }

    pub fn languages<S: Into<String> + Clone>(mut self, languages: &[S]) -> Self {
        self.data.languages = languages
            .iter()
            .map(|s| s.clone().into())
            .collect::<Vec<String>>();
        self
    }

    pub fn sitefilter<S: Into<String> + Clone>(mut self, sitefilter: &[S]) -> Self {
        self.data.sitefilter = sitefilter
            .iter()
            .map(|s| s.clone().into())
            .collect::<Vec<String>>();
        self
    }

    pub fn languagefallback(mut self, languagefallback: bool) -> Self {
        self.data.languagefallback = languagefallback;
        self
    }

    pub fn normalize(mut self, normalize: bool) -> Self {
        self.data.normalize = normalize;
        self
    }
}

impl<NoTitles> ActionApiWbGetEntitiesBuilder<NoTitles> {
    pub fn new() -> Self {
        Self {
            data: ActionApiWbGetEntitiesData::default(),
            _phantom: PhantomData,
        }
    }

    pub fn ids(self, ids: Vec<String>) -> ActionApiWbGetEntitiesBuilder<WbGetEntitiesTitles> {
        ActionApiWbGetEntitiesBuilder {
            data: self.data.with_ids(WbGetEntitiesTitles::Ids(ids)),
            _phantom: PhantomData,
        }
    }

    pub fn sites_title(
        self,
        sites: Vec<String>,
        title: String,
    ) -> ActionApiWbGetEntitiesBuilder<WbGetEntitiesTitles> {
        ActionApiWbGetEntitiesBuilder {
            data: self
                .data
                .with_ids(WbGetEntitiesTitles::SitesTitle { sites, title }),
            _phantom: PhantomData,
        }
    }

    pub fn site_titles(
        self,
        site: String,
        titles: Vec<String>,
    ) -> ActionApiWbGetEntitiesBuilder<WbGetEntitiesTitles> {
        ActionApiWbGetEntitiesBuilder {
            data: self
                .data
                .with_ids(WbGetEntitiesTitles::SiteTitles { site, titles }),
            _phantom: PhantomData,
        }
    }
}

impl<WbGetEntitiesRunnable> ActionApiWbGetEntitiesBuilder<WbGetEntitiesRunnable> {
    pub async fn run(&self, api: &Api) -> Result<Value, MediaWikiError> {
        let params = self.data.params();
        println!("{:#?}", params);
        api.query_api_json(&params, "wbgetentities").await
    }

    pub fn run_sync(&self, api: &ApiSync) -> Result<Value, MediaWikiError> {
        let params = self.data.params();
        println!("{:#?}", params);
        api.query_api_json(&params, "wbgetentities")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_builder() -> ActionApiWbGetEntitiesBuilder<NoTitles> {
        ActionApiWbGetEntitiesBuilder::new()
    }

    // --- Default state ---

    #[test]
    fn default_redirects_is_true() {
        let params = new_builder().ids(vec!["Q1".to_string()]).data.params();
        assert_eq!(params["redirects"], "true");
    }

    #[test]
    fn default_props_is_empty() {
        let params = new_builder().ids(vec!["Q1".to_string()]).data.params();
        assert_eq!(params["props"], "");
    }

    #[test]
    fn default_languages_is_empty() {
        let params = new_builder().ids(vec!["Q1".to_string()]).data.params();
        assert_eq!(params["languages"], "");
    }

    #[test]
    fn default_sitefilter_is_empty() {
        let params = new_builder().ids(vec!["Q1".to_string()]).data.params();
        assert_eq!(params["sitefilter"], "");
    }

    // --- ids() ---

    #[test]
    fn ids_single() {
        let params = new_builder().ids(vec!["Q42".to_string()]).data.params();
        assert_eq!(params["ids"], "Q42");
    }

    #[test]
    fn ids_multiple() {
        let params = new_builder()
            .ids(vec!["Q1".to_string(), "Q2".to_string(), "Q3".to_string()])
            .data
            .params();
        assert_eq!(params["ids"], "Q1|Q2|Q3");
    }

    #[test]
    fn ids_does_not_set_sites_or_titles() {
        let params = new_builder().ids(vec!["Q42".to_string()]).data.params();
        assert!(!params.contains_key("sites"));
        assert!(!params.contains_key("titles"));
    }

    // --- sites_title() ---

    #[test]
    fn sites_title_single_site() {
        let params = new_builder()
            .sites_title(vec!["enwiki".to_string()], "Douglas Adams".to_string())
            .data
            .params();
        assert_eq!(params["sites"], "enwiki");
        assert_eq!(params["titles"], "Douglas Adams");
    }

    #[test]
    fn sites_title_multiple_sites() {
        let params = new_builder()
            .sites_title(
                vec!["enwiki".to_string(), "dewiki".to_string()],
                "Berlin".to_string(),
            )
            .data
            .params();
        assert_eq!(params["sites"], "enwiki|dewiki");
        assert_eq!(params["titles"], "Berlin");
    }

    #[test]
    fn sites_title_does_not_set_ids() {
        let params = new_builder()
            .sites_title(vec!["enwiki".to_string()], "Foo".to_string())
            .data
            .params();
        assert!(!params.contains_key("ids"));
    }

    // --- site_titles() ---

    #[test]
    fn site_titles_single_title() {
        let params = new_builder()
            .site_titles("enwiki".to_string(), vec!["London".to_string()])
            .data
            .params();
        assert_eq!(params["sites"], "enwiki");
        assert_eq!(params["titles"], "London");
    }

    #[test]
    fn site_titles_multiple_titles() {
        let params = new_builder()
            .site_titles(
                "enwiki".to_string(),
                vec![
                    "London".to_string(),
                    "Paris".to_string(),
                    "Berlin".to_string(),
                ],
            )
            .data
            .params();
        assert_eq!(params["sites"], "enwiki");
        assert_eq!(params["titles"], "London|Paris|Berlin");
    }

    #[test]
    fn site_titles_does_not_set_ids() {
        let params = new_builder()
            .site_titles("enwiki".to_string(), vec!["Foo".to_string()])
            .data
            .params();
        assert!(!params.contains_key("ids"));
    }

    // --- redirects() ---

    #[test]
    fn redirects_false() {
        let params = new_builder()
            .redirects(false)
            .ids(vec!["Q1".to_string()])
            .data
            .params();
        assert_eq!(params["redirects"], "false");
    }

    #[test]
    fn redirects_true_explicit() {
        let params = new_builder()
            .redirects(true)
            .ids(vec!["Q1".to_string()])
            .data
            .params();
        assert_eq!(params["redirects"], "true");
    }

    // --- props() ---

    #[test]
    fn props_single() {
        let params = new_builder()
            .props(&["labels"])
            .ids(vec!["Q1".to_string()])
            .data
            .params();
        assert_eq!(params["props"], "labels");
    }

    #[test]
    fn props_multiple() {
        let params = new_builder()
            .props(&["labels", "descriptions", "aliases"])
            .ids(vec!["Q1".to_string()])
            .data
            .params();
        assert_eq!(params["props"], "labels|descriptions|aliases");
    }

    #[test]
    fn props_accepts_owned_strings() {
        let props = vec!["labels".to_string(), "claims".to_string()];
        let params = new_builder()
            .props(&props)
            .ids(vec!["Q1".to_string()])
            .data
            .params();
        assert_eq!(params["props"], "labels|claims");
    }

    // --- languages() ---

    #[test]
    fn languages_single() {
        let params = new_builder()
            .languages(&["en"])
            .ids(vec!["Q1".to_string()])
            .data
            .params();
        assert_eq!(params["languages"], "en");
    }

    #[test]
    fn languages_multiple() {
        let params = new_builder()
            .languages(&["en", "de", "fr"])
            .ids(vec!["Q1".to_string()])
            .data
            .params();
        assert_eq!(params["languages"], "en|de|fr");
    }

    // --- sitefilter() ---

    #[test]
    fn sitefilter_single() {
        let params = new_builder()
            .sitefilter(&["enwiki"])
            .ids(vec!["Q1".to_string()])
            .data
            .params();
        assert_eq!(params["sitefilter"], "enwiki");
    }

    #[test]
    fn sitefilter_multiple() {
        let params = new_builder()
            .sitefilter(&["enwiki", "dewiki", "frwiki"])
            .ids(vec!["Q1".to_string()])
            .data
            .params();
        assert_eq!(params["sitefilter"], "enwiki|dewiki|frwiki");
    }

    // --- languagefallback() ---

    #[test]
    fn languagefallback_stored() {
        let builder = new_builder().languagefallback(true);
        assert!(builder.data.languagefallback);
    }

    #[test]
    fn languagefallback_false_stored() {
        let builder = new_builder().languagefallback(false);
        assert!(!builder.data.languagefallback);
    }

    // --- normalize() ---

    #[test]
    fn normalize_stored() {
        let builder = new_builder().normalize(true);
        assert!(builder.data.normalize);
    }

    #[test]
    fn normalize_false_stored() {
        let builder = new_builder().normalize(false);
        assert!(!builder.data.normalize);
    }

    // --- WbGetEntitiesTitles::params() ---

    #[test]
    fn wbgetentitiestitle_ids_params() {
        let mut params = HashMap::new();
        WbGetEntitiesTitles::Ids(vec!["Q1".to_string(), "Q2".to_string()]).params(&mut params);
        assert_eq!(params["ids"], "Q1|Q2");
        assert!(!params.contains_key("sites"));
        assert!(!params.contains_key("titles"));
    }

    #[test]
    fn wbgetentitiestitle_site_titles_params() {
        let mut params = HashMap::new();
        WbGetEntitiesTitles::SiteTitles {
            site: "enwiki".to_string(),
            titles: vec!["Foo".to_string(), "Bar".to_string()],
        }
        .params(&mut params);
        assert_eq!(params["sites"], "enwiki");
        assert_eq!(params["titles"], "Foo|Bar");
        assert!(!params.contains_key("ids"));
    }

    #[test]
    fn wbgetentitiestitle_sites_title_params() {
        let mut params = HashMap::new();
        WbGetEntitiesTitles::SitesTitle {
            sites: vec!["enwiki".to_string(), "dewiki".to_string()],
            title: "Baz".to_string(),
        }
        .params(&mut params);
        assert_eq!(params["sites"], "enwiki|dewiki");
        assert_eq!(params["titles"], "Baz");
        assert!(!params.contains_key("ids"));
    }

    // --- Builder chaining ---

    #[test]
    fn chaining_multiple_options() {
        let params = new_builder()
            .redirects(false)
            .props(&["labels", "descriptions"])
            .languages(&["en", "de"])
            .sitefilter(&["enwiki"])
            .ids(vec!["Q42".to_string()])
            .data
            .params();
        assert_eq!(params["redirects"], "false");
        assert_eq!(params["props"], "labels|descriptions");
        assert_eq!(params["languages"], "en|de");
        assert_eq!(params["sitefilter"], "enwiki");
        assert_eq!(params["ids"], "Q42");
    }
}
