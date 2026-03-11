use async_trait::*;
use serde_json::Value;
use std::{collections::HashMap, marker::PhantomData};
use wbgetentities::{ActionApiWbGetEntitiesBuilder, NoTitles};

use crate::{
    Api, ApiSync, MediaWikiError,
    action_api::query_info::{ActionApiQueryInfoBuilder, NoTitlesOrPageids},
    action_api::query_linkshere::{ActionApiQueryLinkshereBuilder, NoTitlesOrGenerator},
};

mod query_info;
mod query_linkshere;
mod wbgetentities;

// pub trait ActionApiGenerator {}

pub(crate) trait ActionApiData {
    fn add_boolean(value: bool, key: &str, params: &mut HashMap<String, String>) {
        if value {
            params.insert(key.to_string(), String::new());
        }
    }

    fn add_vec(value: &Option<Vec<String>>, key: &str, params: &mut HashMap<String, String>) {
        if let Some(v) = value {
            params.insert(key.to_string(), v.join("|"));
        }
    }
}

#[async_trait]
pub trait ActionApiRunnable {
    fn params(&self) -> HashMap<String, String>;

    async fn run(&self, api: &Api) -> Result<Value, MediaWikiError> {
        let params = self.params();
        println!("{:#?}", params);
        let ret = api.query_api_json(&params, "GET").await?;
        if let Some(_continue) = ret.get("continue") {
            // TODO use continue["continue"] and e.g. continue["lhcontinue"]
            // watch out for generator continue parameters
        }

        Ok(ret)
    }

    fn run_sync(&self, api: &ApiSync) -> Result<Value, MediaWikiError> {
        let params = self.params();
        println!("{:#?}", params);
        api.query_api_json(&params, "GET")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ActionApi;

impl ActionApi {
    pub fn wbgetentities() -> ActionApiWbGetEntitiesBuilder<NoTitles> {
        ActionApiWbGetEntitiesBuilder::new()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ActionApiQuery {
    _phantom: PhantomData<bool>,
}

impl ActionApiQuery {
    pub fn info() -> ActionApiQueryInfoBuilder<NoTitlesOrPageids> {
        ActionApiQueryInfoBuilder::new()
    }

    pub fn linkshere() -> ActionApiQueryLinkshereBuilder<NoTitlesOrGenerator> {
        ActionApiQueryLinkshereBuilder::new()
    }
}
