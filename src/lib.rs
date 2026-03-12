//! Rust client library for the [MediaWiki Action API](https://www.mediawiki.org/wiki/API:Main_page).
//!
//! # Quick start
//!
//! ```rust
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use mediawiki::prelude::*;
//!
//! let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
//!
//! // Fetch the first five pages starting with "Albert"
//! let result = ActionApiList::allpages()
//!     .apprefix("Albert")
//!     .aplimit(5)
//!     .run(&api)
//!     .await
//!     .unwrap();
//! # });
//! ```
//!
//! See [`prelude`] for the full set of re-exported types and traits.
#![allow(clippy::collapsible_if)]
#![deny(
    // missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

#[cfg(test)]
#[macro_use]
extern crate serde_json;

#[macro_export]
/// To quickly create a hashmap.
/// Example: `hashmap!["action"=>"query","meta"=>"siteinfo","siprop"=>"general|namespaces|namespacealiases|libraries|extensions|statistics"]`
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub use reqwest;

pub mod action_api;
pub mod api;
pub mod prelude;
pub mod api_sync;
pub mod media_wiki_error;
pub mod page;
pub mod revision;
pub mod title;
pub mod user;

pub use crate::api::Api;
pub use crate::api_sync::ApiSync;
pub use crate::media_wiki_error::MediaWikiError;
pub use crate::page::Page;
pub use crate::revision::Revision;
pub use crate::title::Title;
pub use crate::user::User;
