#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

pub use reqwest;

pub mod api;
pub mod page;
pub mod title;
pub mod user;

lazy_static! {
    static ref JUSTIFY_LAZY_STATIC_MACRO_USE: u8 = 0;
    static ref JUSTIFY_SERDE_JSON_MACRO_USE: serde_json::Value = json!("");
}
