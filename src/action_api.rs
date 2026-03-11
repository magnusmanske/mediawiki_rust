use async_trait::*;
use serde_json::Value;
use std::{collections::HashMap, marker::PhantomData};
use wbgetentities::{ActionApiWbGetEntitiesBuilder, NoTitles};

use crate::{
    Api, ApiSync, MediaWikiError,
    action_api::{
        action_block::ActionApiBlockBuilder,
        action_checktoken::ActionApiChecktokenBuilder,
        action_compare::ActionApiCompareBuilder,
        action_delete::ActionApiDeleteBuilder,
        action_edit::ActionApiEditBuilder,
        action_emailuser::ActionApiEmailuserBuilder,
        action_expandtemplates::ActionApiExpandtemplatesBuilder,
        action_login::ActionApiLoginBuilder,
        action_logout::ActionApiLogoutBuilder,
        action_mergehistory::ActionApiMergehistoryBuilder,
        action_move::ActionApiMoveBuilder,
        action_opensearch::ActionApiOpensearchBuilder,
        action_options::ActionApiOptionsBuilder,
        action_parse::ActionApiParseBuilder,
        action_patrol::ActionApiPatrolBuilder,
        action_protect::ActionApiProtectBuilder,
        action_purge::ActionApiPurgeBuilder,
        action_rollback::ActionApiRollbackBuilder,
        action_thank::ActionApiThankBuilder,
        action_unblock::ActionApiUnblockBuilder,
        action_upload::ActionApiUploadBuilder,
        action_userrights::ActionApiUserrightsBuilder,
        action_watch::ActionApiWatchBuilder,
        action_wbcreateclaim::ActionApiWbcreateclaimBuilder,
        action_wbcreateredirect::ActionApiWbcreateredirectBuilder,
        action_wbeditentity::ActionApiWbeditentityBuilder,
        action_wbformatvalue::{ActionApiWbformatvalueBuilder, NoValue},
        action_wbgetclaims::ActionApiWbgetclaimsBuilder,
        action_wblinktitles::ActionApiWblinktitlesBuilder,
        action_wbmergeitems::ActionApiWbmergeitemsBuilder,
        action_wbparsevalue::{ActionApiWbparsevalueBuilder, NoValues},
        action_wbremoveclaims::ActionApiWbremoveclaimsBuilder,
        action_wbremovequalifiers::ActionApiWbremovequalifiersBuilder,
        action_wbremovereferences::ActionApiWbremovereferencesBuilder,
        action_wbsearchentities::{ActionApiWbsearchentitiesBuilder, NoSearch},
        action_wbsetaliases::ActionApiWbsetaliasesBuilder,
        action_wbsetclaim::ActionApiWbsetclaimBuilder,
        action_wbsetclaimvalue::ActionApiWbsetclaimvalueBuilder,
        action_wbsetdescription::ActionApiWbsetdescriptionBuilder,
        action_wbsetlabel::ActionApiWbsetlabelBuilder,
        action_wbsetqualifier::ActionApiWbsetqualifierBuilder,
        action_wbsetreference::ActionApiWbsetreferenceBuilder,
        action_wbsetsitelink::ActionApiWbsetsitelinkBuilder,
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
        query_links::ActionApiQueryLinksBuilder,
        query_linkshere::ActionApiQueryLinkshereBuilder,
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
mod query_links;
mod query_linkshere;
mod query_pageprops;
mod query_redirects;
mod query_revisions;
mod query_templates;
mod query_transcludedin;
mod wbgetentities;

mod action_block;
mod action_checktoken;
mod action_compare;
mod action_delete;
mod action_edit;
mod action_emailuser;
mod action_expandtemplates;
mod action_login;
mod action_logout;
mod action_mergehistory;
mod action_move;
mod action_opensearch;
mod action_options;
mod action_parse;
mod action_patrol;
mod action_protect;
mod action_purge;
mod action_rollback;
mod action_thank;
mod action_unblock;
mod action_upload;
mod action_userrights;
mod action_watch;
mod action_wbcreateclaim;
mod action_wbcreateredirect;
mod action_wbeditentity;
mod action_wbformatvalue;
mod action_wbgetclaims;
mod action_wblinktitles;
mod action_wbmergeitems;
mod action_wbparsevalue;
mod action_wbremoveclaims;
mod action_wbremovequalifiers;
mod action_wbremovereferences;
mod action_wbsearchentities;
mod action_wbsetaliases;
mod action_wbsetclaim;
mod action_wbsetclaimvalue;
mod action_wbsetdescription;
mod action_wbsetlabel;
mod action_wbsetqualifier;
mod action_wbsetreference;
mod action_wbsetsitelink;

#[derive(Debug, Copy, Clone)]
pub struct NoTitlesOrGenerator;

#[derive(Debug, Copy, Clone)]
pub struct Runnable;

#[derive(Debug, Copy, Clone)]
pub struct NoToken;

#[derive(Debug, Clone, Default)]
pub enum ActionApiQueryCommonData {
    #[default]
    None,
    Titles(Vec<String>),
    PageIds(Vec<u64>),
    RevIds(Vec<u64>),
    Generator(HashMap<String, String>),
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

/// Returns `true` if the API response contains a `continue` object, meaning
/// more results are available and `continue_from()` can be called.
pub fn has_more(result: &Value) -> bool {
    result.get("continue").is_some()
}

/// Returns `true` if the API response contains `batchcomplete`, signalling
/// that all prop data for the current generator page batch is complete.
/// Only relevant when using a generator; the next continuation will advance
/// the generator to the next batch of pages.
pub fn batch_complete(result: &Value) -> bool {
    result.get("batchcomplete").is_some()
}

/// Implemented by `Runnable` builders that support result pagination.
/// Call `continue_from(&result)` with the previous API response to build a
/// request that fetches the next page of results.
pub trait ActionApiContinuable: Sized {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String>;

    /// Replace the current continuation state with the `continue` object from
    /// `result`. All key-value pairs in `result["continue"]` are stored and
    /// will be merged into the params of the next `run()` call.
    fn continue_from(mut self, result: &Value) -> Self {
        let m = self.continue_params_mut();
        m.clear();
        if let Some(obj) = result["continue"].as_object() {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    m.insert(k.clone(), s.to_string());
                }
            }
        }
        self
    }
}

pub trait ActionApiGenerator {
    fn generator_params(&self) -> HashMap<String, String>;

    fn prefix_params(letter: char, params: HashMap<String, String>) -> HashMap<String, String> {
        params
            .into_iter()
            .map(|(k, v)| (format!("{letter}{k}"), v))
            .collect()
    }
}

pub trait ActionApiQueryCommonBuilder: Sized {
    type Runnable;

    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData;
    fn into_runnable(self) -> Self::Runnable;

    fn titles<S: Into<String> + Clone>(mut self, titles: &[S]) -> Self::Runnable {
        *self.common_mut() =
            ActionApiQueryCommonData::Titles(titles.iter().map(|s| s.clone().into()).collect());
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

    fn http_method(&self) -> &'static str {
        "GET"
    }

    async fn run(&self, api: &Api) -> Result<Value, MediaWikiError> {
        let params = self.params();
        println!("{:#?}", params);
        let ret = api.query_api_json(&params, self.http_method()).await?;
        if let Some(_continue) = ret.get("continue") {
            // TODO use continue["continue"] and e.g. continue["lhcontinue"]
            // watch out for generator continue parameters
        }

        Ok(ret)
    }

    fn run_sync(&self, api: &ApiSync) -> Result<Value, MediaWikiError> {
        let params = self.params();
        println!("{:#?}", params);
        api.query_api_json(&params, self.http_method())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ActionApi;

impl ActionApi {
    pub fn wbgetentities() -> ActionApiWbGetEntitiesBuilder<NoTitles> {
        ActionApiWbGetEntitiesBuilder::new()
    }

    pub fn edit() -> ActionApiEditBuilder<NoTitlesOrGenerator> {
        ActionApiEditBuilder::new()
    }

    pub fn delete() -> ActionApiDeleteBuilder<NoTitlesOrGenerator> {
        ActionApiDeleteBuilder::new()
    }

    pub fn move_page() -> ActionApiMoveBuilder<NoTitlesOrGenerator> {
        ActionApiMoveBuilder::new()
    }

    pub fn patrol() -> ActionApiPatrolBuilder<NoTitlesOrGenerator> {
        ActionApiPatrolBuilder::new()
    }

    pub fn protect() -> ActionApiProtectBuilder<NoTitlesOrGenerator> {
        ActionApiProtectBuilder::new()
    }

    pub fn purge() -> ActionApiPurgeBuilder<NoTitlesOrGenerator> {
        ActionApiPurgeBuilder::new()
    }

    pub fn rollback() -> ActionApiRollbackBuilder<NoTitlesOrGenerator> {
        ActionApiRollbackBuilder::new()
    }

    pub fn watch() -> ActionApiWatchBuilder<NoTitlesOrGenerator> {
        ActionApiWatchBuilder::new()
    }

    pub fn block() -> ActionApiBlockBuilder<NoTitlesOrGenerator> {
        ActionApiBlockBuilder::new()
    }

    pub fn unblock() -> ActionApiUnblockBuilder<NoTitlesOrGenerator> {
        ActionApiUnblockBuilder::new()
    }

    pub fn thank() -> ActionApiThankBuilder<NoTitlesOrGenerator> {
        ActionApiThankBuilder::new()
    }

    pub fn emailuser() -> ActionApiEmailuserBuilder<NoTitlesOrGenerator> {
        ActionApiEmailuserBuilder::new()
    }

    pub fn userrights() -> ActionApiUserrightsBuilder<NoTitlesOrGenerator> {
        ActionApiUserrightsBuilder::new()
    }

    pub fn upload() -> ActionApiUploadBuilder<NoTitlesOrGenerator> {
        ActionApiUploadBuilder::new()
    }

    pub fn options() -> ActionApiOptionsBuilder<NoTitlesOrGenerator> {
        ActionApiOptionsBuilder::new()
    }

    pub fn mergehistory() -> ActionApiMergehistoryBuilder<NoTitlesOrGenerator> {
        ActionApiMergehistoryBuilder::new()
    }

    pub fn wbgetclaims() -> ActionApiWbgetclaimsBuilder<NoTitlesOrGenerator> {
        ActionApiWbgetclaimsBuilder::new()
    }

    pub fn wbsearchentities() -> ActionApiWbsearchentitiesBuilder<NoSearch> {
        ActionApiWbsearchentitiesBuilder::new()
    }

    pub fn wbformatvalue() -> ActionApiWbformatvalueBuilder<NoValue> {
        ActionApiWbformatvalueBuilder::new()
    }

    pub fn wbparsevalue() -> ActionApiWbparsevalueBuilder<NoValues> {
        ActionApiWbparsevalueBuilder::new()
    }

    pub fn wbeditentity() -> ActionApiWbeditentityBuilder<NoTitlesOrGenerator> {
        ActionApiWbeditentityBuilder::new()
    }

    pub fn wbsetlabel() -> ActionApiWbsetlabelBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetlabelBuilder::new()
    }

    pub fn wbsetdescription() -> ActionApiWbsetdescriptionBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetdescriptionBuilder::new()
    }

    pub fn wbsetaliases() -> ActionApiWbsetaliasesBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetaliasesBuilder::new()
    }

    pub fn wbmergeitems() -> ActionApiWbmergeitemsBuilder<NoTitlesOrGenerator> {
        ActionApiWbmergeitemsBuilder::new()
    }

    pub fn wbcreateredirect() -> ActionApiWbcreateredirectBuilder<NoTitlesOrGenerator> {
        ActionApiWbcreateredirectBuilder::new()
    }

    pub fn wblinktitles() -> ActionApiWblinktitlesBuilder<NoTitlesOrGenerator> {
        ActionApiWblinktitlesBuilder::new()
    }

    pub fn wbsetsitelink() -> ActionApiWbsetsitelinkBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetsitelinkBuilder::new()
    }

    pub fn wbcreateclaim() -> ActionApiWbcreateclaimBuilder<NoTitlesOrGenerator> {
        ActionApiWbcreateclaimBuilder::new()
    }

    pub fn wbremoveclaims() -> ActionApiWbremoveclaimsBuilder<NoTitlesOrGenerator> {
        ActionApiWbremoveclaimsBuilder::new()
    }

    pub fn wbsetclaim() -> ActionApiWbsetclaimBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetclaimBuilder::new()
    }

    pub fn wbsetclaimvalue() -> ActionApiWbsetclaimvalueBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetclaimvalueBuilder::new()
    }

    pub fn wbsetqualifier() -> ActionApiWbsetqualifierBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetqualifierBuilder::new()
    }

    pub fn wbremovequalifiers() -> ActionApiWbremovequalifiersBuilder<NoTitlesOrGenerator> {
        ActionApiWbremovequalifiersBuilder::new()
    }

    pub fn wbsetreference() -> ActionApiWbsetreferenceBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetreferenceBuilder::new()
    }

    pub fn wbremovereferences() -> ActionApiWbremovereferencesBuilder<NoTitlesOrGenerator> {
        ActionApiWbremovereferencesBuilder::new()
    }

    pub fn login() -> ActionApiLoginBuilder<NoTitlesOrGenerator> {
        ActionApiLoginBuilder::new()
    }

    pub fn logout() -> ActionApiLogoutBuilder<NoTitlesOrGenerator> {
        ActionApiLogoutBuilder::new()
    }

    pub fn opensearch() -> ActionApiOpensearchBuilder<NoTitlesOrGenerator> {
        ActionApiOpensearchBuilder::new()
    }

    pub fn checktoken() -> ActionApiChecktokenBuilder<NoTitlesOrGenerator> {
        ActionApiChecktokenBuilder::new()
    }

    pub fn expandtemplates() -> ActionApiExpandtemplatesBuilder<NoTitlesOrGenerator> {
        ActionApiExpandtemplatesBuilder::new()
    }

    pub fn compare() -> ActionApiCompareBuilder {
        ActionApiCompareBuilder::new()
    }

    pub fn parse() -> ActionApiParseBuilder {
        ActionApiParseBuilder::new()
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
