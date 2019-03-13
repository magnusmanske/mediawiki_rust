//use serde_json::Value;
use crate::api::Api;
use std::collections::HashMap;
use wikibase;

/// A container of `Entity` values
pub struct EntityContainer {
    pub entities: HashMap<String, wikibase::Entity>,
}

impl EntityContainer {
    pub fn new() -> EntityContainer {
        EntityContainer {
            entities: HashMap::<String, wikibase::Entity>::new(),
        }
    }

    // Loads (new) entities from the MediaWiki API
    pub fn load_entities(&mut self, api: &mut Api, entity_ids: &Vec<String>) {
        let to_load = entity_ids
            .iter()
            .filter(|entity_id| !entity_id.is_empty())
            .filter(|entity_id| !self.entities.contains_key(*entity_id))
            .map(|entity_id| entity_id.to_owned())
            .collect::<Vec<String>>();

        // TODO multi-threaded
        for chunk in to_load.chunks(50) {
            let ids = chunk.join("|");
            let params: HashMap<_, _> = vec![("action", "wbgetentities"), ("ids", &ids)]
                .into_iter()
                .collect();

            let j = api.get_query_api_json(&params).unwrap();
            //println!("{}", ::serde_json::to_string_pretty(&j).unwrap());
            for (entity_id, entity_json) in j["entities"].as_object().unwrap() {
                match wikibase::from_json::entity_from_json(entity_json) {
                    Ok(entity) => {
                        self.entities.insert(entity_id.to_string(), entity);
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        dbg!(&self.entities);
    }
}
