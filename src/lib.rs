#[macro_use]
extern crate lazy_static;

pub mod api;
pub mod title;
pub mod user;

lazy_static! {
    static ref JUSTIFY_LAZY_STATIC_MACRO_USE: u8 = 0;
}
