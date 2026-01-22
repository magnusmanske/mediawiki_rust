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
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate serde_json;
extern crate hmac;
extern crate nanoid;

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

pub mod api;
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
