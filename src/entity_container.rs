//use serde_json::Value;
use crate::api::Api;
use std::collections::HashMap;
//use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;
use wikibase;

/// A container of `Entity` values
pub struct EntityContainer {
    pub entities: HashMap<String, wikibase::Entity>,
}

impl EntityContainer {
    /// Generates a new, empty `EntityContainer`
    pub fn new() -> EntityContainer {
        EntityContainer {
            entities: HashMap::<String, wikibase::Entity>::new(),
        }
    }

    /// Loads (new) entities from the MediaWiki API
    pub fn load_entities(&mut self, api: &Api, entity_ids: &Vec<String>) {
        let to_load = entity_ids
            .iter()
            .filter(|entity_id| !entity_id.is_empty())
            .filter(|entity_id| !self.entities.contains_key(*entity_id))
            .map(|entity_id| entity_id.to_owned())
            .collect::<Vec<String>>();

        let (tx, rx) = mpsc::channel();
        let mut chunks: u64 = 0;
        for chunk in to_load.chunks(50) {
            chunks = chunks + 1;
            let ids = chunk.join("|");
            let params: HashMap<_, _> = vec![
                ("action", "wbgetentities"),
                ("ids", &ids),
                ("format", "json"),
            ]
            .into_iter()
            .collect();
            let req = api
                .get_api_request_builder(&params, "GET")
                .expect("GET failed");

            let tx = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                let response = req.send().expect("Getting response from API failed");
                tx.send(response).expect("Sending of result failed");
            });
        }

        for _ in 0..chunks {
            let mut response = rx.recv().unwrap();
            let j: serde_json::Value = response.json().expect("Parsing response into JSON failed");
            for (entity_id, entity_json) in j["entities"]
                .as_object()
                .expect("Accessing entities failed")
            {
                match wikibase::from_json::entity_from_json(&entity_json) {
                    Ok(entity) => {
                        self.entities.insert(entity_id.to_string(), entity);
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
    }

    /// Returns `Some(entity)` with that ID from the cache, or `None`.
    /// This will _not_ load entities via the API!
    pub fn get_entity(&self, entity_id: &str) -> Option<&wikibase::Entity> {
        self.entities.get(entity_id)
    }

    /// Removes the entity with the given key from the cache, and returns `Some(entity)` or `None`
    pub fn remove_entity(&mut self, entity_id: &str) -> Option<wikibase::Entity> {
        self.entities.remove(entity_id)
    }

    /// Removes the entities with the given keys from the cache
    pub fn remove_entities(&mut self, entity_ids: &Vec<String>) {
        for entity_id in entity_ids {
            self.remove_entity(entity_id);
        }
    }

    /// Removes the entities with the given keys from the cache, then reloads them from the API
    pub fn reload_entities(&mut self, api: &Api, entity_ids: &Vec<String>) {
        self.remove_entities(entity_ids);
        self.load_entities(api, entity_ids);
    }
}
