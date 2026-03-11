/// Prelude for convenient glob import: `use mediawiki::prelude::*;`
///
/// Brings into scope:
/// - Core types: `Api`, `ApiSync`, `Page`, `Revision`, `Title`, `User`, `MediaWikiError`
/// - Action API entry points: `ActionApi`, `ActionApiQuery`, `ActionApiList`
/// - Required traits: `ActionApiRunnable`, `ActionApiQueryCommonBuilder`,
///   `ActionApiGenerator`
pub use crate::{
    Api, ApiSync, MediaWikiError, Page, Revision, Title, User,
    action_api::{
        ActionApi, ActionApiContinuable, ActionApiGenerator, ActionApiList, ActionApiQuery,
        ActionApiQueryCommonBuilder, ActionApiRunnable, batch_complete, has_more,
    },
};
