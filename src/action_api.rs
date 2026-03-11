use async_trait::*;
use serde_json::Value;
use std::{collections::HashMap, marker::PhantomData};
use wbgetentities::{ActionApiWbGetEntitiesBuilder, NoTitles};

use crate::{
    Api, ApiSync, MediaWikiError,
    action_api::{
        list_allcategories::ActionApiListAllcategoriesBuilder,
        list_allpages::ActionApiListAllpagesBuilder,
        list_backlinks::ActionApiListBacklinksBuilder,
        list_categorymembers::ActionApiListCategorymembersBuilder,
        list_embeddedin::ActionApiListEmbeddedinBuilder,
        list_imageusage::ActionApiListImageusageBuilder,
        list_logevents::ActionApiListLogeventsBuilder,
        list_prefixsearch::ActionApiListPrefixsearchBuilder,
        list_recentchanges::ActionApiListRecentchangesBuilder,
        list_search::ActionApiListSearchBuilder,
        list_usercontribs::ActionApiListUsercontribsBuilder,
        list_users::ActionApiListUsersBuilder,
        query_categories::ActionApiQueryCategoriesBuilder,
        query_categoryinfo::ActionApiQueryCategoryinfoBuilder,
        query_contributors::ActionApiQueryContributorsBuilder,
        query_extlinks::ActionApiQueryExtlinksBuilder,
        query_fileusage::ActionApiQueryFileusageBuilder,
        query_images::ActionApiQueryImagesBuilder,
        query_info::ActionApiQueryInfoBuilder,
        query_iwlinks::ActionApiQueryIwlinksBuilder,
        query_langlinks::ActionApiQueryLanglinksBuilder,
        query_linkshere::ActionApiQueryLinkshereBuilder,
        query_links::ActionApiQueryLinksBuilder,
        query_pageprops::ActionApiQueryPagepropsBuilder,
        query_redirects::ActionApiQueryRedirectsBuilder,
        query_revisions::ActionApiQueryRevisionsBuilder,
        query_templates::ActionApiQueryTemplatesBuilder,
        query_transcludedin::ActionApiQueryTranscludedinBuilder,
    },
};

mod list_allcategories;
mod list_allpages;
mod list_backlinks;
mod list_categorymembers;
mod list_embeddedin;
mod list_imageusage;
mod list_logevents;
mod list_prefixsearch;
mod list_recentchanges;
mod list_search;
mod list_usercontribs;
mod list_users;
mod query_categories;
mod query_categoryinfo;
mod query_contributors;
mod query_extlinks;
mod query_fileusage;
mod query_images;
mod query_info;
mod query_iwlinks;
mod query_langlinks;
mod query_linkshere;
mod query_links;
mod query_pageprops;
mod query_redirects;
mod query_revisions;
mod query_templates;
mod query_transcludedin;
mod wbgetentities;

#[derive(Debug, Copy, Clone)]
pub struct NoTitlesOrGenerator;

#[derive(Debug, Copy, Clone)]
pub struct Runnable;

#[derive(Debug, Clone)]
pub(crate) enum ActionApiQueryCommonData {
    None,
    Titles(Vec<String>),
    PageIds(Vec<u64>),
    RevIds(Vec<u64>),
    Generator(HashMap<String, String>),
}

impl Default for ActionApiQueryCommonData {
    fn default() -> Self {
        Self::None
    }
}

impl ActionApiQueryCommonData {
    pub(crate) fn add_to_params(&self, params: &mut HashMap<String, String>) {
        match self {
            Self::None => {}
            Self::Titles(titles) => {
                params.insert("titles".to_string(), titles.join("|"));
            }
            Self::PageIds(pageids) => {
                let s: Vec<String> = pageids.iter().map(|id| id.to_string()).collect();
                params.insert("pageids".to_string(), s.join("|"));
            }
            Self::RevIds(revids) => {
                let s: Vec<String> = revids.iter().map(|id| id.to_string()).collect();
                params.insert("revids".to_string(), s.join("|"));
            }
            Self::Generator(generator) => {
                params.extend(generator.clone());
            }
        }
    }
}

pub(crate) trait ActionApiGenerator {
    fn generator_params(&self) -> HashMap<String, String>;

    fn prefix_params(letter: char, params: HashMap<String, String>) -> HashMap<String, String> {
        params
            .into_iter()
            .map(|(k, v)| (format!("{letter}{k}"), v))
            .collect()
    }
}

pub(crate) trait ActionApiQueryCommonBuilder: Sized {
    type Runnable;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData;
    fn into_runnable(self) -> Self::Runnable;

    fn titles<S: Into<String> + Clone>(mut self, titles: &[S]) -> Self::Runnable {
        *self.common_mut() = ActionApiQueryCommonData::Titles(
            titles.iter().map(|s| s.clone().into()).collect(),
        );
        self.into_runnable()
    }

    fn pageids(mut self, pageids: &[u64]) -> Self::Runnable {
        *self.common_mut() = ActionApiQueryCommonData::PageIds(pageids.to_vec());
        self.into_runnable()
    }

    fn revids(mut self, revids: &[u64]) -> Self::Runnable {
        *self.common_mut() = ActionApiQueryCommonData::RevIds(revids.to_vec());
        self.into_runnable()
    }

    fn generator<G: ActionApiGenerator>(mut self, generator: &G) -> Self::Runnable {
        *self.common_mut() = ActionApiQueryCommonData::Generator(generator.generator_params());
        self.into_runnable()
    }
}

pub(crate) trait ActionApiData {
    fn add_boolean(value: bool, key: &str, params: &mut HashMap<String, String>) {
        if value {
            params.insert(key.to_string(), String::new());
        }
    }

    fn add_vec(value: &Option<Vec<String>>, key: &str, params: &mut HashMap<String, String>) {
        if let Some(v) = value {
            params.insert(key.to_string(), v.join("|"));
        }
    }

    fn add_str(value: &Option<String>, key: &str, params: &mut HashMap<String, String>) {
        if let Some(v) = value {
            params.insert(key.to_string(), v.to_owned());
        }
    }
}

#[async_trait]
pub trait ActionApiRunnable {
    fn params(&self) -> HashMap<String, String>;

    async fn run(&self, api: &Api) -> Result<Value, MediaWikiError> {
        let params = self.params();
        println!("{:#?}", params);
        let ret = api.query_api_json(&params, "GET").await?;
        if let Some(_continue) = ret.get("continue") {
            // TODO use continue["continue"] and e.g. continue["lhcontinue"]
            // watch out for generator continue parameters
        }

        Ok(ret)
    }

    fn run_sync(&self, api: &ApiSync) -> Result<Value, MediaWikiError> {
        let params = self.params();
        println!("{:#?}", params);
        api.query_api_json(&params, "GET")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ActionApi;

impl ActionApi {
    pub fn wbgetentities() -> ActionApiWbGetEntitiesBuilder<NoTitles> {
        ActionApiWbGetEntitiesBuilder::new()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ActionApiQuery {
    _phantom: PhantomData<bool>,
}

impl ActionApiQuery {
    pub fn categories() -> ActionApiQueryCategoriesBuilder<NoTitlesOrGenerator> {
        ActionApiQueryCategoriesBuilder::new()
    }

    pub fn categoryinfo() -> ActionApiQueryCategoryinfoBuilder<NoTitlesOrGenerator> {
        ActionApiQueryCategoryinfoBuilder::new()
    }

    pub fn contributors() -> ActionApiQueryContributorsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryContributorsBuilder::new()
    }

    pub fn extlinks() -> ActionApiQueryExtlinksBuilder<NoTitlesOrGenerator> {
        ActionApiQueryExtlinksBuilder::new()
    }

    pub fn fileusage() -> ActionApiQueryFileusageBuilder<NoTitlesOrGenerator> {
        ActionApiQueryFileusageBuilder::new()
    }

    pub fn images() -> ActionApiQueryImagesBuilder<NoTitlesOrGenerator> {
        ActionApiQueryImagesBuilder::new()
    }

    pub fn info() -> ActionApiQueryInfoBuilder<NoTitlesOrGenerator> {
        ActionApiQueryInfoBuilder::new()
    }

    pub fn iwlinks() -> ActionApiQueryIwlinksBuilder<NoTitlesOrGenerator> {
        ActionApiQueryIwlinksBuilder::new()
    }

    pub fn langlinks() -> ActionApiQueryLanglinksBuilder<NoTitlesOrGenerator> {
        ActionApiQueryLanglinksBuilder::new()
    }

    pub fn linkshere() -> ActionApiQueryLinkshereBuilder<NoTitlesOrGenerator> {
        ActionApiQueryLinkshereBuilder::new()
    }

    pub fn links() -> ActionApiQueryLinksBuilder<NoTitlesOrGenerator> {
        ActionApiQueryLinksBuilder::new()
    }

    pub fn pageprops() -> ActionApiQueryPagepropsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryPagepropsBuilder::new()
    }

    pub fn redirects() -> ActionApiQueryRedirectsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryRedirectsBuilder::new()
    }

    pub fn revisions() -> ActionApiQueryRevisionsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryRevisionsBuilder::new()
    }

    pub fn templates() -> ActionApiQueryTemplatesBuilder<NoTitlesOrGenerator> {
        ActionApiQueryTemplatesBuilder::new()
    }

    pub fn transcludedin() -> ActionApiQueryTranscludedinBuilder<NoTitlesOrGenerator> {
        ActionApiQueryTranscludedinBuilder::new()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ActionApiList {
    _phantom: PhantomData<bool>,
}

impl ActionApiList {
    pub fn allcategories() -> ActionApiListAllcategoriesBuilder {
        ActionApiListAllcategoriesBuilder::new()
    }

    pub fn allpages() -> ActionApiListAllpagesBuilder {
        ActionApiListAllpagesBuilder::new()
    }

    pub fn backlinks() -> ActionApiListBacklinksBuilder<NoTitlesOrGenerator> {
        ActionApiListBacklinksBuilder::new()
    }

    pub fn categorymembers() -> ActionApiListCategorymembersBuilder<NoTitlesOrGenerator> {
        ActionApiListCategorymembersBuilder::new()
    }

    pub fn embeddedin() -> ActionApiListEmbeddedinBuilder<NoTitlesOrGenerator> {
        ActionApiListEmbeddedinBuilder::new()
    }

    pub fn imageusage() -> ActionApiListImageusageBuilder<NoTitlesOrGenerator> {
        ActionApiListImageusageBuilder::new()
    }

    pub fn logevents() -> ActionApiListLogeventsBuilder {
        ActionApiListLogeventsBuilder::new()
    }

    pub fn prefixsearch() -> ActionApiListPrefixsearchBuilder<NoTitlesOrGenerator> {
        ActionApiListPrefixsearchBuilder::new()
    }

    pub fn recentchanges() -> ActionApiListRecentchangesBuilder {
        ActionApiListRecentchangesBuilder::new()
    }

    pub fn search() -> ActionApiListSearchBuilder<NoTitlesOrGenerator> {
        ActionApiListSearchBuilder::new()
    }

    pub fn usercontribs() -> ActionApiListUsercontribsBuilder<NoTitlesOrGenerator> {
        ActionApiListUsercontribsBuilder::new()
    }

    pub fn users() -> ActionApiListUsersBuilder<NoTitlesOrGenerator> {
        ActionApiListUsersBuilder::new()
    }
}
