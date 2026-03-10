use wbgetentities::{ActionApiWbGetEntitiesBuilder, NoTitles};

mod wbgetentities;

// pub trait ActionApiAction {}
// pub trait ActionApiGenerator {}
// pub trait ActionApiRunnable {}

#[derive(Debug, Clone, Copy)]
pub struct ActionApi;

impl ActionApi {
    pub fn wbgetentities() -> ActionApiWbGetEntitiesBuilder<NoTitles> {
        ActionApiWbGetEntitiesBuilder::new()
    }
}
