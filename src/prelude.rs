/// Prelude for convenient glob import: `use mediawiki::prelude::*;`
///
/// Brings into scope:
/// - Core types: `Api`, `ApiSync`, `Page`, `Revision`, `Title`, `User`, `MediaWikiError`
/// - Core traits: `MediaWikiApi`
/// - Page info: `PageInfo`, `PageInfoList`, `ProtectionEntry`
/// - Action API entry points: `ActionApi`, `ActionApiQuery`, `ActionApiList`
/// - Required traits: `ActionApiRunnable`, `ActionApiQueryCommonBuilder`,
///   `ActionApiGenerator`
pub use crate::{
    Api, ApiSync, MediaWikiError, MediaWikiApi, Page, Revision, Title, User,
    action_api::{
        ActionApi, ActionApiContinuable, ActionApiGenerator, ActionApiList, ActionApiMeta,
        ActionApiQuery, ActionApiQueryCommonBuilder, ActionApiRunnable,
    },
    page_categories::{PageCategoryEntry, PageCategoryList},
    page_info::{PageInfo, PageInfoList, ProtectionEntry},
    page_links::{
        PageContributorEntry, PageContributorList, PageExtLinkEntry, PageExtLinkList,
        PageFileUsageEntry, PageFileUsageList, PageImageEntry, PageImageList, PageIwLinkEntry,
        PageIwLinkList, PageLangLinkEntry, PageLangLinkList, PageLinkEntry, PageLinkList,
        PageLinksHereEntry, PageLinksHereList, PageRedirectEntry, PageRedirectList,
        PageTemplateEntry, PageTemplateList, PageTranscludedInEntry, PageTranscludedInList,
    },
    page_query::{PageContext, PageQueryResult, PageQueryResultList},
    page_revisions::{PageRevisionEntry, PageRevisionList},
};
