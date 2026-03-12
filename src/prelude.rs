/// Prelude for convenient glob import: `use mediawiki::prelude::*;`
///
/// Brings into scope:
/// - Core types: `Api`, `ApiSync`, `Page`, `Revision`, `Title`, `User`, `MediaWikiError`
/// - Page info: `PageInfo`, `PageInfoList`
/// - Action API entry points: `ActionApi`, `ActionApiQuery`, `ActionApiList`
/// - Required traits: `ActionApiRunnable`, `ActionApiQueryCommonBuilder`,
///   `ActionApiGenerator`
pub use crate::{
    Api, ApiSync, MediaWikiError, Page, Revision, Title, User,
    action_api::{
        ActionApi, ActionApiContinuable, ActionApiGenerator, ActionApiList, ActionApiQuery,
        ActionApiQueryCommonBuilder, ActionApiRunnable,
    },
    page_info::{PageInfo, PageInfoList},
    page_query::{PageContext, PageQueryResult, PageQueryResultList},
};
