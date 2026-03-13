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
        action_sitematrix::ActionApiSitematrixBuilder,
        action_stashedit::ActionApiStasheditBuilder,
        action_tag::ActionApiTagBuilder,
        action_thank::ActionApiThankBuilder,
        action_unblock::ActionApiUnblockBuilder,
        action_undelete::ActionApiUndeleteBuilder,
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
        list_allfileusages::ActionApiListAllfileusagesBuilder,
        list_allimages::ActionApiListAllimagesBuilder,
        list_alllinks::ActionApiListAlllinksBuilder,
        list_allpages::ActionApiListAllpagesBuilder,
        list_allredirects::ActionApiListAllredirectsBuilder,
        list_alltransclusions::ActionApiListAlltranslusionsBuilder,
        list_allusers::ActionApiListAllusersBuilder,
        list_backlinks::ActionApiListBacklinksBuilder,
        list_blocks::ActionApiListBlocksBuilder,
        list_categorymembers::ActionApiListCategorymembersBuilder,
        list_embeddedin::ActionApiListEmbeddedinBuilder,
        list_exturlusage::ActionApiListExturlusageBuilder,
        list_imageusage::ActionApiListImageusageBuilder,
        list_logevents::ActionApiListLogeventsBuilder,
        list_pagepropnames::ActionApiListPagepropnamesBuilder,
        list_pageswithprop::ActionApiListPageswithpropBuilder,
        list_prefixsearch::ActionApiListPrefixsearchBuilder,
        list_protectedtitles::ActionApiListProtectedtitlesBuilder,
        list_random::ActionApiListRandomBuilder,
        list_recentchanges::ActionApiListRecentchangesBuilder,
        list_search::ActionApiListSearchBuilder,
        list_tags::ActionApiListTagsBuilder,
        list_usercontribs::ActionApiListUsercontribsBuilder,
        list_users::ActionApiListUsersBuilder,
        list_watchlist::ActionApiListWatchlistBuilder,
        list_watchlistraw::ActionApiListWatchlistrawBuilder,
        meta_allmessages::ActionApiMetaAllmessagesBuilder,
        meta_filerepoinfo::ActionApiMetaFilerepoinfoBuilder,
        meta_languageinfo::ActionApiMetaLanguageinfoBuilder,
        meta_siteinfo::ActionApiMetaSiteinfoBuilder,
        meta_tokens::ActionApiMetaTokensBuilder,
        meta_userinfo::ActionApiMetaUserinfoBuilder,
        query_categories::ActionApiQueryCategoriesBuilder,
        query_categoryinfo::ActionApiQueryCategoryinfoBuilder,
        query_contributors::ActionApiQueryContributorsBuilder,
        query_deletedrevisions::ActionApiQueryDeletedrevisionsBuilder,
        query_duplicatefiles::ActionApiQueryDuplicatefilesBuilder,
        query_extlinks::ActionApiQueryExtlinksBuilder,
        query_fileusage::ActionApiQueryFileusageBuilder,
        query_images::ActionApiQueryImagesBuilder,
        query_imageinfo::ActionApiQueryImageinfoBuilder,
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
mod list_allfileusages;
mod list_allimages;
mod list_alllinks;
mod list_allpages;
mod list_allredirects;
mod list_alltransclusions;
mod list_allusers;
mod list_backlinks;
mod list_blocks;
mod list_categorymembers;
mod list_embeddedin;
mod list_exturlusage;
mod list_imageusage;
mod list_logevents;
mod list_pagepropnames;
mod list_pageswithprop;
mod list_prefixsearch;
mod list_protectedtitles;
mod list_random;
mod list_recentchanges;
mod list_search;
mod list_tags;
mod list_usercontribs;
mod list_users;
mod list_watchlist;
mod list_watchlistraw;
mod meta_allmessages;
mod meta_filerepoinfo;
mod meta_languageinfo;
mod meta_siteinfo;
mod meta_tokens;
mod meta_userinfo;
mod query_categories;
mod query_categoryinfo;
mod query_contributors;
mod query_deletedrevisions;
mod query_duplicatefiles;
mod query_extlinks;
mod query_fileusage;
mod query_imageinfo;
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
mod action_wbgetentities;
use action_wbgetentities as wbgetentities;

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
mod action_sitematrix;
mod action_stashedit;
mod action_tag;
mod action_thank;
mod action_unblock;
mod action_undelete;
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

/// Typestate marker: no page target (title or generator) has been set yet.
#[derive(Debug, Copy, Clone)]
pub struct NoTitlesOrGenerator;

/// Typestate marker: the builder has all required fields and can be executed via [`ActionApiRunnable::run`].
#[derive(Debug, Copy, Clone)]
pub struct Runnable;

/// Typestate marker: a page target has been set but a CSRF token is still required before the
/// request can be executed. Call `.token(token)` to advance to [`Runnable`].
#[derive(Debug, Copy, Clone)]
pub struct NoToken;

/// Specifies how a query-property request identifies the pages it operates on.
#[derive(Debug, Clone, Default)]
pub enum ActionApiQueryCommonData {
    /// No target set; the builder is not yet runnable.
    #[default]
    None,
    /// Query pages by title.
    Titles(Vec<String>),
    /// Query pages by page ID.
    PageIds(Vec<u64>),
    /// Query pages by revision ID.
    RevIds(Vec<u64>),
    /// Use a generator to supply the page list.
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

/// Implemented by `Runnable` builders that support result pagination.
/// Call `continue_from(&result)` with the previous API response to build a
/// request that fetches the next page of results.
pub trait ActionApiContinuable: Sized {
    fn continue_params_mut(&mut self) -> &mut HashMap<String, String>;

    /// Returns `true` if the API response contains a `continue` object, meaning
    /// more results are available and `continue_from()` can be called.
    fn has_more(&self, result: &Value) -> bool {
        result.get("continue").is_some()
    }

    /// Returns `true` if the API response contains `batchcomplete`, signalling
    /// that all prop data for the current generator page batch is complete.
    /// Only relevant when using a generator; the next continuation will advance
    /// the generator to the next batch of pages.
    fn batch_complete(&self, result: &Value) -> bool {
        result.get("batchcomplete").is_some()
    }

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

/// Implemented by builders whose results can be used as a generator for query-property requests.
///
/// Pass a reference to a generator builder to [`ActionApiQueryCommonBuilder::generator`] to make
/// the generator supply the page list for the query.
pub trait ActionApiGenerator {
    /// Returns the generator parameters to be merged into a query request.
    fn generator_params(&self) -> HashMap<String, String>;

    /// Prefixes every key in `params` with `letter`, e.g. `"g"` for generator parameters.
    fn prefix_params(letter: char, params: HashMap<String, String>) -> HashMap<String, String> {
        params
            .into_iter()
            .map(|(k, v)| (format!("{letter}{k}"), v))
            .collect()
    }
}

/// Builder interface shared by all query-property requests.
///
/// Consuming one of these methods transitions the builder to the `Runnable` typestate so that
/// [`ActionApiRunnable::run`] becomes available.
///
/// # Example
///
/// ```rust
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use mediawiki::prelude::*;
/// let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
/// let result = ActionApiQuery::categories()
///     .titles(&["Rust (programming language)"])
///     .run(&api)
///     .await
///     .unwrap();
/// # });
/// ```
pub trait ActionApiQueryCommonBuilder: Sized {
    /// The `Runnable` builder type produced after a target is set.
    type Runnable;

    /// Returns a mutable reference to the shared page-target data.
    fn common_mut(&mut self) -> &mut ActionApiQueryCommonData;
    /// Converts `self` into the `Runnable` builder type.
    fn into_runnable(self) -> Self::Runnable;

    /// Sets the pages to query by title.
    fn titles<S: Into<String> + Clone>(mut self, titles: &[S]) -> Self::Runnable {
        *self.common_mut() =
            ActionApiQueryCommonData::Titles(titles.iter().map(|s| s.clone().into()).collect());
        self.into_runnable()
    }

    /// Sets the pages to query by page ID.
    fn pageids(mut self, pageids: &[u64]) -> Self::Runnable {
        *self.common_mut() = ActionApiQueryCommonData::PageIds(pageids.to_vec());
        self.into_runnable()
    }

    /// Sets the pages to query by revision ID.
    fn revids(mut self, revids: &[u64]) -> Self::Runnable {
        *self.common_mut() = ActionApiQueryCommonData::RevIds(revids.to_vec());
        self.into_runnable()
    }

    /// Uses a generator to supply the page list.
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

/// Implemented by all fully-configured ("Runnable") request builders.
///
/// Use [`run`](ActionApiRunnable::run) for async execution or
/// [`run_sync`](ActionApiRunnable::run_sync) for synchronous execution.
#[async_trait]
pub trait ActionApiRunnable {
    /// Returns the complete set of API parameters for this request.
    fn params(&self) -> HashMap<String, String>;

    /// HTTP method to use; defaults to `"GET"`. Write actions return `"POST"`.
    fn http_method(&self) -> &'static str {
        "GET"
    }

    /// Executes the request asynchronously and returns the raw API JSON response.
    async fn run(&self, api: &Api) -> Result<Value, MediaWikiError> {
        let params = self.params();
        let ret = api.query_api_json(&params, self.http_method()).await?;
        if let Some(_continue) = ret.get("continue") {
            // TODO use continue["continue"] and e.g. continue["lhcontinue"]
            // watch out for generator continue parameters
        }

        Ok(ret)
    }

    /// Executes the request synchronously and returns the raw API JSON response.
    fn run_sync(&self, api: &ApiSync) -> Result<Value, MediaWikiError> {
        let params = self.params();
        api.query_api_json(&params, self.http_method())
    }
}

/// Entry point for MediaWiki write actions and miscellaneous API actions.
///
/// Each method returns a builder in the [`NoTitlesOrGenerator`] (or similar initial) typestate.
/// Set the required fields to advance the builder to [`Runnable`], then call
/// [`ActionApiRunnable::run`].
///
/// # Example
///
/// ```rust
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use mediawiki::prelude::*;
/// let api = Api::new("https://www.wikidata.org/w/api.php").await.unwrap();
/// let result = ActionApi::wbgetentities()
///     .ids(&["Q42"])
///     .run(&api)
///     .await
///     .unwrap();
/// # });
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ActionApi;

impl ActionApi {
    /// Fetches Wikibase entities (`action=wbgetentities`).
    pub fn wbgetentities() -> ActionApiWbGetEntitiesBuilder<NoTitles> {
        ActionApiWbGetEntitiesBuilder::new()
    }

    /// Edits a page (`action=edit`).
    pub fn edit() -> ActionApiEditBuilder<NoTitlesOrGenerator> {
        ActionApiEditBuilder::new()
    }

    /// Deletes a page (`action=delete`).
    pub fn delete() -> ActionApiDeleteBuilder<NoTitlesOrGenerator> {
        ActionApiDeleteBuilder::new()
    }

    /// Moves (renames) a page (`action=move`).
    pub fn move_page() -> ActionApiMoveBuilder<NoTitlesOrGenerator> {
        ActionApiMoveBuilder::new()
    }

    /// Patrols a page or revision (`action=patrol`).
    pub fn patrol() -> ActionApiPatrolBuilder<NoTitlesOrGenerator> {
        ActionApiPatrolBuilder::new()
    }

    /// Sets page protection levels (`action=protect`).
    pub fn protect() -> ActionApiProtectBuilder<NoTitlesOrGenerator> {
        ActionApiProtectBuilder::new()
    }

    /// Purges the server-side cache for pages (`action=purge`).
    pub fn purge() -> ActionApiPurgeBuilder<NoTitlesOrGenerator> {
        ActionApiPurgeBuilder::new()
    }

    /// Rolls back a series of edits (`action=rollback`).
    pub fn rollback() -> ActionApiRollbackBuilder<NoTitlesOrGenerator> {
        ActionApiRollbackBuilder::new()
    }

    /// Adds or removes pages from the watchlist (`action=watch`).
    pub fn watch() -> ActionApiWatchBuilder<NoTitlesOrGenerator> {
        ActionApiWatchBuilder::new()
    }

    /// Blocks a user (`action=block`).
    pub fn block() -> ActionApiBlockBuilder<NoTitlesOrGenerator> {
        ActionApiBlockBuilder::new()
    }

    /// Unblocks a user (`action=unblock`).
    pub fn unblock() -> ActionApiUnblockBuilder<NoTitlesOrGenerator> {
        ActionApiUnblockBuilder::new()
    }

    /// Thanks a user for an edit (`action=thank`).
    pub fn thank() -> ActionApiThankBuilder<NoTitlesOrGenerator> {
        ActionApiThankBuilder::new()
    }

    /// Sends an email to a user (`action=emailuser`).
    pub fn emailuser() -> ActionApiEmailuserBuilder<NoTitlesOrGenerator> {
        ActionApiEmailuserBuilder::new()
    }

    /// Changes group memberships for a user (`action=userrights`).
    pub fn userrights() -> ActionApiUserrightsBuilder<NoTitlesOrGenerator> {
        ActionApiUserrightsBuilder::new()
    }

    /// Uploads a file (`action=upload`).
    pub fn upload() -> ActionApiUploadBuilder<NoTitlesOrGenerator> {
        ActionApiUploadBuilder::new()
    }

    /// Changes preferences for the current user (`action=options`).
    pub fn options() -> ActionApiOptionsBuilder<NoTitlesOrGenerator> {
        ActionApiOptionsBuilder::new()
    }

    /// Merges the edit history of two pages (`action=mergehistory`).
    pub fn mergehistory() -> ActionApiMergehistoryBuilder<NoTitlesOrGenerator> {
        ActionApiMergehistoryBuilder::new()
    }

    /// Fetches Wikibase claims/statements (`action=wbgetclaims`).
    pub fn wbgetclaims() -> ActionApiWbgetclaimsBuilder<NoTitlesOrGenerator> {
        ActionApiWbgetclaimsBuilder::new()
    }

    /// Searches for Wikibase entities (`action=wbsearchentities`).
    pub fn wbsearchentities() -> ActionApiWbsearchentitiesBuilder<NoSearch> {
        ActionApiWbsearchentitiesBuilder::new()
    }

    /// Formats a Wikibase data value (`action=wbformatvalue`).
    pub fn wbformatvalue() -> ActionApiWbformatvalueBuilder<NoValue> {
        ActionApiWbformatvalueBuilder::new()
    }

    /// Parses a Wikibase data value (`action=wbparsevalue`).
    pub fn wbparsevalue() -> ActionApiWbparsevalueBuilder<NoValues> {
        ActionApiWbparsevalueBuilder::new()
    }

    /// Creates or edits a Wikibase entity (`action=wbeditentity`).
    pub fn wbeditentity() -> ActionApiWbeditentityBuilder<NoTitlesOrGenerator> {
        ActionApiWbeditentityBuilder::new()
    }

    /// Sets a label on a Wikibase entity (`action=wbsetlabel`).
    pub fn wbsetlabel() -> ActionApiWbsetlabelBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetlabelBuilder::new()
    }

    /// Sets a description on a Wikibase entity (`action=wbsetdescription`).
    pub fn wbsetdescription() -> ActionApiWbsetdescriptionBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetdescriptionBuilder::new()
    }

    /// Sets aliases on a Wikibase entity (`action=wbsetaliases`).
    pub fn wbsetaliases() -> ActionApiWbsetaliasesBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetaliasesBuilder::new()
    }

    /// Merges two Wikibase items (`action=wbmergeitems`).
    pub fn wbmergeitems() -> ActionApiWbmergeitemsBuilder<NoTitlesOrGenerator> {
        ActionApiWbmergeitemsBuilder::new()
    }

    /// Creates a redirect between Wikibase entities (`action=wbcreateredirect`).
    pub fn wbcreateredirect() -> ActionApiWbcreateredirectBuilder<NoTitlesOrGenerator> {
        ActionApiWbcreateredirectBuilder::new()
    }

    /// Links titles from different sites on a Wikibase entity (`action=wblinktitles`).
    pub fn wblinktitles() -> ActionApiWblinktitlesBuilder<NoTitlesOrGenerator> {
        ActionApiWblinktitlesBuilder::new()
    }

    /// Sets a sitelink on a Wikibase entity (`action=wbsetsitelink`).
    pub fn wbsetsitelink() -> ActionApiWbsetsitelinkBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetsitelinkBuilder::new()
    }

    /// Creates a claim on a Wikibase entity (`action=wbcreateclaim`).
    pub fn wbcreateclaim() -> ActionApiWbcreateclaimBuilder<NoTitlesOrGenerator> {
        ActionApiWbcreateclaimBuilder::new()
    }

    /// Removes claims from a Wikibase entity (`action=wbremoveclaims`).
    pub fn wbremoveclaims() -> ActionApiWbremoveclaimsBuilder<NoTitlesOrGenerator> {
        ActionApiWbremoveclaimsBuilder::new()
    }

    /// Sets or updates a claim on a Wikibase entity (`action=wbsetclaim`).
    pub fn wbsetclaim() -> ActionApiWbsetclaimBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetclaimBuilder::new()
    }

    /// Sets the value of a claim on a Wikibase entity (`action=wbsetclaimvalue`).
    pub fn wbsetclaimvalue() -> ActionApiWbsetclaimvalueBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetclaimvalueBuilder::new()
    }

    /// Sets a qualifier on a Wikibase claim (`action=wbsetqualifier`).
    pub fn wbsetqualifier() -> ActionApiWbsetqualifierBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetqualifierBuilder::new()
    }

    /// Removes qualifiers from a Wikibase claim (`action=wbremovequalifiers`).
    pub fn wbremovequalifiers() -> ActionApiWbremovequalifiersBuilder<NoTitlesOrGenerator> {
        ActionApiWbremovequalifiersBuilder::new()
    }

    /// Sets a reference on a Wikibase claim (`action=wbsetreference`).
    pub fn wbsetreference() -> ActionApiWbsetreferenceBuilder<NoTitlesOrGenerator> {
        ActionApiWbsetreferenceBuilder::new()
    }

    /// Removes references from a Wikibase claim (`action=wbremovereferences`).
    pub fn wbremovereferences() -> ActionApiWbremovereferencesBuilder<NoTitlesOrGenerator> {
        ActionApiWbremovereferencesBuilder::new()
    }

    /// Logs in to the MediaWiki API (`action=login`).
    pub fn login() -> ActionApiLoginBuilder<NoTitlesOrGenerator> {
        ActionApiLoginBuilder::new()
    }

    /// Logs out of the MediaWiki API (`action=logout`).
    pub fn logout() -> ActionApiLogoutBuilder<NoTitlesOrGenerator> {
        ActionApiLogoutBuilder::new()
    }

    /// Performs an OpenSearch (search-suggest) query (`action=opensearch`).
    pub fn opensearch() -> ActionApiOpensearchBuilder<NoTitlesOrGenerator> {
        ActionApiOpensearchBuilder::new()
    }

    /// Checks the validity of a token (`action=checktoken`).
    pub fn checktoken() -> ActionApiChecktokenBuilder<NoTitlesOrGenerator> {
        ActionApiChecktokenBuilder::new()
    }

    /// Expands templates in wikitext (`action=expandtemplates`).
    pub fn expandtemplates() -> ActionApiExpandtemplatesBuilder<NoTitlesOrGenerator> {
        ActionApiExpandtemplatesBuilder::new()
    }

    /// Compares two pages or revisions (`action=compare`).
    pub fn compare() -> ActionApiCompareBuilder {
        ActionApiCompareBuilder::new()
    }

    /// Parses wikitext and returns HTML or other structured output (`action=parse`).
    pub fn parse() -> ActionApiParseBuilder {
        ActionApiParseBuilder::new()
    }

    /// Returns the Wikimedia sitematrix (`action=sitematrix`).
    pub fn sitematrix() -> ActionApiSitematrixBuilder {
        ActionApiSitematrixBuilder::new()
    }

    /// Undeletes revisions of a deleted page (`action=undelete`).
    pub fn undelete() -> ActionApiUndeleteBuilder<NoTitlesOrGenerator> {
        ActionApiUndeleteBuilder::new()
    }

    /// Adds or removes change tags (`action=tag`).
    pub fn tag() -> ActionApiTagBuilder<NoTitlesOrGenerator> {
        ActionApiTagBuilder::new()
    }

    /// Prepares an edit in shared cache (`action=stashedit`).
    pub fn stashedit() -> ActionApiStasheditBuilder<NoTitlesOrGenerator> {
        ActionApiStasheditBuilder::new()
    }
}

/// Entry point for `action=query` property modules (prop=…).
///
/// Each method returns a builder in the [`NoTitlesOrGenerator`] typestate. Set the target pages
/// via [`ActionApiQueryCommonBuilder`] methods to advance to [`Runnable`].
#[derive(Debug, Clone, Copy, Default)]
pub struct ActionApiQuery {
    _phantom: PhantomData<bool>,
}

impl ActionApiQuery {
    /// Returns the categories a page belongs to (`prop=categories`).
    pub fn categories() -> ActionApiQueryCategoriesBuilder<NoTitlesOrGenerator> {
        ActionApiQueryCategoriesBuilder::new()
    }

    /// Returns category information for category pages (`prop=categoryinfo`).
    pub fn categoryinfo() -> ActionApiQueryCategoryinfoBuilder<NoTitlesOrGenerator> {
        ActionApiQueryCategoryinfoBuilder::new()
    }

    /// Returns the list of contributors to a page (`prop=contributors`).
    pub fn contributors() -> ActionApiQueryContributorsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryContributorsBuilder::new()
    }

    /// Returns the external links on a page (`prop=extlinks`).
    pub fn extlinks() -> ActionApiQueryExtlinksBuilder<NoTitlesOrGenerator> {
        ActionApiQueryExtlinksBuilder::new()
    }

    /// Returns pages that use a given file (`prop=fileusage`).
    pub fn fileusage() -> ActionApiQueryFileusageBuilder<NoTitlesOrGenerator> {
        ActionApiQueryFileusageBuilder::new()
    }

    /// Returns the images embedded on a page (`prop=images`).
    pub fn images() -> ActionApiQueryImagesBuilder<NoTitlesOrGenerator> {
        ActionApiQueryImagesBuilder::new()
    }

    /// Returns basic page information (`prop=info`).
    pub fn info() -> ActionApiQueryInfoBuilder<NoTitlesOrGenerator> {
        ActionApiQueryInfoBuilder::new()
    }

    /// Returns interwiki links on a page (`prop=iwlinks`).
    pub fn iwlinks() -> ActionApiQueryIwlinksBuilder<NoTitlesOrGenerator> {
        ActionApiQueryIwlinksBuilder::new()
    }

    /// Returns language links (interlanguage links) on a page (`prop=langlinks`).
    pub fn langlinks() -> ActionApiQueryLanglinksBuilder<NoTitlesOrGenerator> {
        ActionApiQueryLanglinksBuilder::new()
    }

    /// Returns pages that link to a given page (`prop=linkshere`).
    pub fn linkshere() -> ActionApiQueryLinkshereBuilder<NoTitlesOrGenerator> {
        ActionApiQueryLinkshereBuilder::new()
    }

    /// Returns the internal links on a page (`prop=links`).
    pub fn links() -> ActionApiQueryLinksBuilder<NoTitlesOrGenerator> {
        ActionApiQueryLinksBuilder::new()
    }

    /// Returns page properties set by parser functions or extensions (`prop=pageprops`).
    pub fn pageprops() -> ActionApiQueryPagepropsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryPagepropsBuilder::new()
    }

    /// Returns pages that redirect to a given page (`prop=redirects`).
    pub fn redirects() -> ActionApiQueryRedirectsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryRedirectsBuilder::new()
    }

    /// Returns revision information for pages (`prop=revisions`).
    pub fn revisions() -> ActionApiQueryRevisionsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryRevisionsBuilder::new()
    }

    /// Returns the templates transcluded on a page (`prop=templates`).
    pub fn templates() -> ActionApiQueryTemplatesBuilder<NoTitlesOrGenerator> {
        ActionApiQueryTemplatesBuilder::new()
    }

    /// Returns pages that transclude a given page (`prop=transcludedin`).
    pub fn transcludedin() -> ActionApiQueryTranscludedinBuilder<NoTitlesOrGenerator> {
        ActionApiQueryTranscludedinBuilder::new()
    }

    /// Returns file information and upload history (`prop=imageinfo`).
    pub fn imageinfo() -> ActionApiQueryImageinfoBuilder<NoTitlesOrGenerator> {
        ActionApiQueryImageinfoBuilder::new()
    }

    /// Finds all files that are duplicates of the given files (`prop=duplicatefiles`).
    pub fn duplicatefiles() -> ActionApiQueryDuplicatefilesBuilder<NoTitlesOrGenerator> {
        ActionApiQueryDuplicatefilesBuilder::new()
    }

    /// Gets deleted revision information (`prop=deletedrevisions`).
    pub fn deletedrevisions() -> ActionApiQueryDeletedrevisionsBuilder<NoTitlesOrGenerator> {
        ActionApiQueryDeletedrevisionsBuilder::new()
    }
}

/// Entry point for `action=query` list modules (list=…).
///
/// Each method returns a fully-configured builder that implements [`ActionApiRunnable`] and
/// optionally [`ActionApiContinuable`] for pagination.
#[derive(Debug, Clone, Copy, Default)]
pub struct ActionApiList {
    _phantom: PhantomData<bool>,
}

impl ActionApiList {
    /// Lists all categories (`list=allcategories`).
    pub fn allcategories() -> ActionApiListAllcategoriesBuilder {
        ActionApiListAllcategoriesBuilder::new()
    }

    /// Lists all pages in a namespace (`list=allpages`).
    pub fn allpages() -> ActionApiListAllpagesBuilder {
        ActionApiListAllpagesBuilder::new()
    }

    /// Returns pages that link to a given page (`list=backlinks`).
    pub fn backlinks() -> ActionApiListBacklinksBuilder<NoTitlesOrGenerator> {
        ActionApiListBacklinksBuilder::new()
    }

    /// Lists pages in a category (`list=categorymembers`).
    pub fn categorymembers() -> ActionApiListCategorymembersBuilder<NoTitlesOrGenerator> {
        ActionApiListCategorymembersBuilder::new()
    }

    /// Returns pages that embed (transclude) a given page (`list=embeddedin`).
    pub fn embeddedin() -> ActionApiListEmbeddedinBuilder<NoTitlesOrGenerator> {
        ActionApiListEmbeddedinBuilder::new()
    }

    /// Returns pages that use a given image file (`list=imageusage`).
    pub fn imageusage() -> ActionApiListImageusageBuilder<NoTitlesOrGenerator> {
        ActionApiListImageusageBuilder::new()
    }

    /// Lists log events (`list=logevents`).
    pub fn logevents() -> ActionApiListLogeventsBuilder {
        ActionApiListLogeventsBuilder::new()
    }

    /// Performs a prefix search across page titles (`list=prefixsearch`).
    pub fn prefixsearch() -> ActionApiListPrefixsearchBuilder<NoTitlesOrGenerator> {
        ActionApiListPrefixsearchBuilder::new()
    }

    /// Lists recent changes (`list=recentchanges`).
    pub fn recentchanges() -> ActionApiListRecentchangesBuilder {
        ActionApiListRecentchangesBuilder::new()
    }

    /// Performs a full-text search (`list=search`).
    pub fn search() -> ActionApiListSearchBuilder<NoTitlesOrGenerator> {
        ActionApiListSearchBuilder::new()
    }

    /// Lists contributions made by a user (`list=usercontribs`).
    pub fn usercontribs() -> ActionApiListUsercontribsBuilder<NoTitlesOrGenerator> {
        ActionApiListUsercontribsBuilder::new()
    }

    /// Retrieves information about a list of users (`list=users`).
    pub fn users() -> ActionApiListUsersBuilder<NoTitlesOrGenerator> {
        ActionApiListUsersBuilder::new()
    }

    /// Enumerates all registered users (`list=allusers`).
    pub fn allusers() -> ActionApiListAllusersBuilder {
        ActionApiListAllusersBuilder::new()
    }

    /// Enumerates all images (`list=allimages`).
    pub fn allimages() -> ActionApiListAllimagesBuilder {
        ActionApiListAllimagesBuilder::new()
    }

    /// Enumerates all links pointing to a given namespace (`list=alllinks`).
    pub fn alllinks() -> ActionApiListAlllinksBuilder {
        ActionApiListAlllinksBuilder::new()
    }

    /// Lists all transclusions (`list=alltransclusions`).
    pub fn alltransclusions() -> ActionApiListAlltranslusionsBuilder {
        ActionApiListAlltranslusionsBuilder::new()
    }

    /// Enumerates all file usages (`list=allfileusages`).
    pub fn allfileusages() -> ActionApiListAllfileusagesBuilder {
        ActionApiListAllfileusagesBuilder::new()
    }

    /// Lists all redirects (`list=allredirects`).
    pub fn allredirects() -> ActionApiListAllredirectsBuilder {
        ActionApiListAllredirectsBuilder::new()
    }

    /// Lists all blocked users and IP addresses (`list=blocks`).
    pub fn blocks() -> ActionApiListBlocksBuilder {
        ActionApiListBlocksBuilder::new()
    }

    /// Enumerates pages containing a given URL (`list=exturlusage`).
    pub fn exturlusage() -> ActionApiListExturlusageBuilder {
        ActionApiListExturlusageBuilder::new()
    }

    /// Lists all page property names in use on the wiki (`list=pagepropnames`).
    pub fn pagepropnames() -> ActionApiListPagepropnamesBuilder {
        ActionApiListPagepropnamesBuilder::new()
    }

    /// Lists all pages using a certain page property (`list=pageswithprop`).
    pub fn pageswithprop() -> ActionApiListPageswithpropBuilder {
        ActionApiListPageswithpropBuilder::new()
    }

    /// Lists all titles protected from creation (`list=protectedtitles`).
    pub fn protectedtitles() -> ActionApiListProtectedtitlesBuilder {
        ActionApiListProtectedtitlesBuilder::new()
    }

    /// Gets a set of random pages (`list=random`).
    pub fn random() -> ActionApiListRandomBuilder {
        ActionApiListRandomBuilder::new()
    }

    /// Lists change tags (`list=tags`).
    pub fn tags() -> ActionApiListTagsBuilder {
        ActionApiListTagsBuilder::new()
    }

    /// Gets recent changes to pages in the current user's watchlist (`list=watchlist`).
    pub fn watchlist() -> ActionApiListWatchlistBuilder {
        ActionApiListWatchlistBuilder::new()
    }

    /// Gets all pages on the current user's watchlist (`list=watchlistraw`).
    pub fn watchlistraw() -> ActionApiListWatchlistrawBuilder {
        ActionApiListWatchlistrawBuilder::new()
    }
}

/// Entry point for `action=query` meta modules (meta=…).
///
/// Each method returns a fully-configured builder that implements [`ActionApiRunnable`].
#[derive(Debug, Clone, Copy, Default)]
pub struct ActionApiMeta {
    _phantom: PhantomData<bool>,
}

impl ActionApiMeta {
    /// Returns general site information (`meta=siteinfo`).
    pub fn siteinfo() -> ActionApiMetaSiteinfoBuilder {
        ActionApiMetaSiteinfoBuilder::new()
    }

    /// Returns information about the current user (`meta=userinfo`).
    pub fn userinfo() -> ActionApiMetaUserinfoBuilder {
        ActionApiMetaUserinfoBuilder::new()
    }

    /// Gets tokens for data-modifying actions (`meta=tokens`).
    pub fn tokens() -> ActionApiMetaTokensBuilder {
        ActionApiMetaTokensBuilder::new()
    }

    /// Returns messages from this site (`meta=allmessages`).
    pub fn allmessages() -> ActionApiMetaAllmessagesBuilder {
        ActionApiMetaAllmessagesBuilder::new()
    }

    /// Returns meta information about image repositories (`meta=filerepoinfo`).
    pub fn filerepoinfo() -> ActionApiMetaFilerepoinfoBuilder {
        ActionApiMetaFilerepoinfoBuilder::new()
    }

    /// Returns information about available languages (`meta=languageinfo`).
    pub fn languageinfo() -> ActionApiMetaLanguageinfoBuilder {
        ActionApiMetaLanguageinfoBuilder::new()
    }
}
