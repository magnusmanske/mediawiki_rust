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

pub mod media_wiki_error;
pub mod api;
pub mod api_sync;
pub mod page;
pub mod title;
pub mod user;
