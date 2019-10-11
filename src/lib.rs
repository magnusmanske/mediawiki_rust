#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;
//extern crate futures;

pub use reqwest;
use std::error::Error;

pub mod api;
pub mod title;
pub mod user;

type MWerror = Box<dyn Error + Send + Sync>;

lazy_static! {
    static ref JUSTIFY_LAZY_STATIC_MACRO_USE: u8 = 0;
    static ref JUSTIFY_SERDE_JSON_MACRO_USE: serde_json::Value = json!("");
}
