use crate::action_api::{ActionApiData, ActionApiRunnable};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ActionApiWbGetEntitiesData {
    ids: Option<WbGetEntitiesTitles>,
    redirects: bool,
    props: Option<Vec<String>>,
    languages: Option<Vec<String>>,
    languagefallback: bool,
    normalize: bool,
    sitefilter: Option<Vec<String>>,
}

impl Default for ActionApiWbGetEntitiesData {
    fn default() -> Self {
        Self {
            ids: None,
            redirects: true,
            props: None,
            languages: None,
            languagefallback: false,
            normalize: false,
            sitefilter: None,
        }
    }
}

impl ActionApiData for ActionApiWbGetEntitiesData {}

impl ActionApiWbGetEntitiesData {
    pub(crate) fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(ids) = &self.ids {
            ids.params(&mut params);
        }
        if !self.redirects {
            // Default: true=yes
            params.insert("redirects".to_string(), "no".to_string());
        }
        Self::add_vec(&self.props, "props", &mut params);
        Self::add_vec(&self.languages, "languages", &mut params);
        Self::add_vec(&self.sitefilter, "sitefilter", &mut params);
        Self::add_boolean(self.languagefallback, "languagefallback", &mut params);
        Self::add_boolean(self.normalize, "normalize", &mut params);
        params
    }

    pub(crate) fn with_ids(mut self, ids: WbGetEntitiesTitles) -> Self {
        self.ids = Some(ids);
        self
    }
}

/// Typestate marker: no entity identifier has been set yet on `wbgetentities`.
#[derive(Debug, Clone, Copy)]
pub struct NoTitles;

/// Specifies how entities are identified in a `wbgetentities` request.
#[derive(Debug, Clone)]
pub enum WbGetEntitiesTitles {
    /// Identify entities by their Wikibase IDs (e.g. `["Q42", "P31"]`).
    Ids(Vec<String>),
    /// Identify entities by a single sitelink site and multiple titles.
    SiteTitles { site: String, titles: Vec<String> },
    /// Identify entities by multiple sitelink sites and a single title.
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

/// Builder for `action=wbgetentities` — fetches Wikibase entities.
///
/// # Typestate
/// - Start: [`NoTitles`] — call `.ids()`, `.site_titles()`, or `.sites_title()` to set the target.
/// - After target: [`WbGetEntitiesTitles`] — builder is [`Runnable`](crate::action_api::Runnable).
///
/// # Example
/// ```rust
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use mediawiki::prelude::*;
/// let api = Api::new("https://www.wikidata.org/w/api.php").await.unwrap();
/// let result = ActionApi::wbgetentities()
///     .ids(&["Q42"])
///     .languages(&["en"])
///     .run(&api)
///     .await
///     .unwrap();
/// # });
/// ```
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ActionApiWbGetEntitiesBuilder<T> {
    _phantom: PhantomData<T>,
    data: ActionApiWbGetEntitiesData,
}

impl<T> ActionApiWbGetEntitiesBuilder<T> {
    /// Whether to follow entity redirects (default `true`) (`redirects`).
    pub fn redirects(mut self, redirects: bool) -> Self {
        self.data.redirects = redirects;
        self
    }

    /// Entity properties to include, e.g. `["labels", "descriptions", "claims"]` (`props`).
    pub fn props<S: Into<String> + Clone>(mut self, props: &[S]) -> Self {
        self.data.props = Some(
            props
                .iter()
                .map(|s| s.clone().into())
                .collect::<Vec<String>>(),
        );
        self
    }

    /// Language codes to filter labels/descriptions/aliases to, e.g. `["en", "de"]` (`languages`).
    pub fn languages<S: Into<String> + Clone>(mut self, languages: &[S]) -> Self {
        self.data.languages = Some(
            languages
                .iter()
                .map(|s| s.clone().into())
                .collect::<Vec<String>>(),
        );
        self
    }

    /// Sitelink sites to filter sitelinks to, e.g. `["enwiki", "dewiki"]` (`sitefilter`).
    pub fn sitefilter<S: Into<String> + Clone>(mut self, sitefilter: &[S]) -> Self {
        self.data.sitefilter = Some(
            sitefilter
                .iter()
                .map(|s| s.clone().into())
                .collect::<Vec<String>>(),
        );
        self
    }

    /// Apply language fallback chains when languages are filtered (`languagefallback`).
    pub fn languagefallback(mut self, languagefallback: bool) -> Self {
        self.data.languagefallback = languagefallback;
        self
    }

    /// Normalise sitelink titles to their canonical form (`normalize`).
    pub fn normalize(mut self, normalize: bool) -> Self {
        self.data.normalize = normalize;
        self
    }
}

impl ActionApiWbGetEntitiesBuilder<NoTitles> {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self {
            data: ActionApiWbGetEntitiesData::default(),
            _phantom: PhantomData,
        }
    }

    /// Fetch entities by their Wikibase IDs (e.g. `["Q42", "P31"]`), advancing to [`Runnable`](crate::action_api::Runnable).
    pub fn ids<S: Into<String> + Clone>(
        self,
        ids: &[S],
    ) -> ActionApiWbGetEntitiesBuilder<WbGetEntitiesTitles> {
        let ids = ids.iter().map(|s| s.clone().into()).collect();
        ActionApiWbGetEntitiesBuilder {
            data: self.data.with_ids(WbGetEntitiesTitles::Ids(ids)),
            _phantom: PhantomData,
        }
    }

    /// Fetch entities by multiple sitelink sites and a single title, advancing to [`Runnable`](crate::action_api::Runnable).
    pub fn sites_title<S, T>(
        self,
        sites: &[S],
        title: T,
    ) -> ActionApiWbGetEntitiesBuilder<WbGetEntitiesTitles>
    where
        S: Into<String> + Clone,
        T: AsRef<str>,
    {
        let sites = sites.iter().map(|s| s.clone().into()).collect();
        let title = title.as_ref().to_string();
        ActionApiWbGetEntitiesBuilder {
            data: self
                .data
                .with_ids(WbGetEntitiesTitles::SitesTitle { sites, title }),
            _phantom: PhantomData,
        }
    }

    /// Fetch entities by a single sitelink site and multiple titles, advancing to [`Runnable`](crate::action_api::Runnable).
    pub fn site_titles<S, T>(
        self,
        site: S,
        titles: &[T],
    ) -> ActionApiWbGetEntitiesBuilder<WbGetEntitiesTitles>
    where
        S: AsRef<str>,
        T: Into<String> + Clone,
    {
        let site = site.as_ref().into();
        let titles = titles.iter().map(|s| s.clone().into()).collect();
        ActionApiWbGetEntitiesBuilder {
            data: self
                .data
                .with_ids(WbGetEntitiesTitles::SiteTitles { site, titles }),
            _phantom: PhantomData,
        }
    }
}

impl ActionApiRunnable for ActionApiWbGetEntitiesBuilder<WbGetEntitiesTitles> {
    fn params(&self) -> HashMap<String, String> {
        let mut ret = self.data.params();
        ret.insert("action".to_string(), "wbgetentities".to_string());
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::{Api, action_api::ActionApi};

    use super::*;

    fn new_builder() -> ActionApiWbGetEntitiesBuilder<NoTitles> {
        ActionApiWbGetEntitiesBuilder::new()
    }

    // --- Default state ---

    #[test]
    fn default_redirects_is_true() {
        let params = new_builder().ids(&["Q1"]).data.params();
        assert!(!params.contains_key("redirects"));
    }

    #[test]
    fn default_props_is_empty() {
        let params = new_builder().ids(&["Q1"]).data.params();
        assert!(!params.contains_key("props"));
    }

    #[test]
    fn default_languages_is_empty() {
        let params = new_builder().ids(&["Q1"]).data.params();
        assert!(!params.contains_key("languages"));
    }

    #[test]
    fn default_sitefilter_is_empty() {
        let params = new_builder().ids(&["Q1"]).data.params();
        assert!(!params.contains_key("sitefilter"));
    }

    // --- ids() ---

    #[test]
    fn ids_single() {
        let params = new_builder().ids(&["Q42"]).data.params();
        assert_eq!(params["ids"], "Q42");
    }

    #[test]
    fn ids_multiple() {
        let params = new_builder().ids(&["Q1", "Q2", "Q3"]).data.params();
        assert_eq!(params["ids"], "Q1|Q2|Q3");
    }

    #[test]
    fn ids_does_not_set_sites_or_titles() {
        let params = new_builder().ids(&["Q42"]).data.params();
        assert!(!params.contains_key("sites"));
        assert!(!params.contains_key("titles"));
    }

    // --- sites_title() ---

    #[test]
    fn sites_title_single_site() {
        let params = new_builder()
            .sites_title(&["enwiki"], "Douglas Adams")
            .data
            .params();
        assert_eq!(params["sites"], "enwiki");
        assert_eq!(params["titles"], "Douglas Adams");
    }

    #[test]
    fn sites_title_multiple_sites() {
        let params = new_builder()
            .sites_title(&["enwiki", "dewiki"], "Berlin")
            .data
            .params();
        assert_eq!(params["sites"], "enwiki|dewiki");
        assert_eq!(params["titles"], "Berlin");
    }

    #[test]
    fn sites_title_does_not_set_ids() {
        let params = new_builder().sites_title(&["enwiki"], "Foo").data.params();
        assert!(!params.contains_key("ids"));
    }

    // --- site_titles() ---

    #[test]
    fn site_titles_single_title() {
        let params = new_builder()
            .site_titles("enwiki", &["London"])
            .data
            .params();
        assert_eq!(params["sites"], "enwiki");
        assert_eq!(params["titles"], "London");
    }

    #[test]
    fn site_titles_multiple_titles() {
        let params = new_builder()
            .site_titles("enwiki", &["London", "Paris", "Berlin"])
            .data
            .params();
        assert_eq!(params["sites"], "enwiki");
        assert_eq!(params["titles"], "London|Paris|Berlin");
    }

    #[test]
    fn site_titles_does_not_set_ids() {
        let params = new_builder().site_titles("enwiki", &["Foo"]).data.params();
        assert!(!params.contains_key("ids"));
    }

    // --- redirects() ---

    #[test]
    fn redirects_false() {
        let params = new_builder().redirects(false).ids(&["Q1"]).data.params();
        assert_eq!(params["redirects"], "no");
    }

    #[test]
    fn redirects_true_explicit() {
        let params = new_builder().redirects(true).ids(&["Q1"]).data.params();
        assert!(!params.contains_key("redirects"));
    }

    // --- props() ---

    #[test]
    fn props_single() {
        let params = new_builder().props(&["labels"]).ids(&["Q1"]).data.params();
        assert_eq!(params["props"], "labels");
    }

    #[test]
    fn props_multiple() {
        let params = new_builder()
            .props(&["labels", "descriptions", "aliases"])
            .ids(&["Q1"])
            .data
            .params();
        assert_eq!(params["props"], "labels|descriptions|aliases");
    }

    #[test]
    fn props_accepts_owned_strings() {
        let props = vec!["labels".to_string(), "claims".to_string()];
        let params = new_builder().props(&props).ids(&["Q1"]).data.params();
        assert_eq!(params["props"], "labels|claims");
    }

    // --- languages() ---

    #[test]
    fn languages_single() {
        let params = new_builder().languages(&["en"]).ids(&["Q1"]).data.params();
        assert_eq!(params["languages"], "en");
    }

    #[test]
    fn languages_multiple() {
        let params = new_builder()
            .languages(&["en", "de", "fr"])
            .ids(&["Q1"])
            .data
            .params();
        assert_eq!(params["languages"], "en|de|fr");
    }

    // --- sitefilter() ---

    #[test]
    fn sitefilter_single() {
        let params = new_builder()
            .sitefilter(&["enwiki"])
            .ids(&["Q1"])
            .data
            .params();
        assert_eq!(params["sitefilter"], "enwiki");
    }

    #[test]
    fn sitefilter_multiple() {
        let params = new_builder()
            .sitefilter(&["enwiki", "dewiki", "frwiki"])
            .ids(&["Q1"])
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
            .ids(&["Q42"])
            .data
            .params();
        assert_eq!(params["redirects"], "no");
        assert_eq!(params["props"], "labels|descriptions");
        assert_eq!(params["languages"], "en|de");
        assert_eq!(params["sitefilter"], "enwiki");
        assert_eq!(params["ids"], "Q42");
    }

    #[tokio::test]
    async fn test_wbgetentities() {
        let api = Api::new("https://www.wikidata.org/w/api.php")
            .await
            .unwrap();
        let result = ActionApi::wbgetentities()
            .ids(&["Q5"])
            .run(&api)
            .await
            .unwrap();
        assert!(result["entities"]["Q5"].is_object())
    }
}
