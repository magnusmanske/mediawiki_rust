extern crate lazy_static;

//use crate::api::Api;
//use std::collections::HashMap;
use wikibase::{Entity, LocaleString};

#[derive(Debug, Default)]
pub struct EntityDiffParam {
    pub add: Vec<String>,
    pub remove: Vec<String>,
    pub alter: Vec<String>,
}

impl EntityDiffParam {
    pub fn all() -> EntityDiffParam {
        EntityDiffParam {
            add: vec!["*".to_string()],
            remove: vec!["*".to_string()],
            alter: vec!["*".to_string()],
        }
    }

    pub fn none() -> EntityDiffParam {
        EntityDiffParam {
            add: vec![],
            remove: vec![],
            alter: vec![],
        }
    }

    pub fn some(list: &Vec<String>) -> EntityDiffParam {
        EntityDiffParam {
            add: list.clone(),
            remove: list.clone(),
            alter: list.clone(),
        }
    }

    pub fn valid(&self, key: &String, action: &str) -> bool {
        lazy_static! {
            static ref STAR: String = "*".to_string();
        }
        match action {
            "add" => self.add.contains(key) || self.add.contains(&STAR),
            "remove" => self.remove.contains(key) || self.add.contains(&STAR),
            "alter" => self.alter.contains(key) || self.add.contains(&STAR),
            _ => panic!("Bad mode '{}' in EntityDiffParam::valid", &action),
        }
    }
}

#[derive(Debug, Default)]
pub struct EntityDiffParams {
    pub labels: EntityDiffParam,
    pub descriptions: EntityDiffParam,
    pub aliases: EntityDiffParam,
    pub claims: EntityDiffParam,
    pub sitelinks: EntityDiffParam,
}

impl EntityDiffParams {
    pub fn all() -> EntityDiffParams {
        EntityDiffParams {
            labels: EntityDiffParam::all(),
            descriptions: EntityDiffParam::all(),
            aliases: EntityDiffParam::all(),
            claims: EntityDiffParam::all(),
            sitelinks: EntityDiffParam::all(),
        }
    }

    pub fn none() -> EntityDiffParams {
        EntityDiffParams {
            labels: EntityDiffParam::none(),
            descriptions: EntityDiffParam::none(),
            aliases: EntityDiffParam::none(),
            claims: EntityDiffParam::none(),
            sitelinks: EntityDiffParam::none(),
        }
    }
}

#[derive(Debug, Default)]
pub struct EntityDiff {
    j: serde_json::Value,
}

impl EntityDiff {
    pub fn new(i1: &Entity, i2: &Entity, params: &EntityDiffParams) -> EntityDiff {
        let mut ret = EntityDiff { j: json!({}) };
        ret.diff(i1, i2, params);
        ret
    }

    pub fn actions(&self) -> &serde_json::Value {
        &self.j
    }

    fn diff(&mut self, i1: &Entity, i2: &Entity, params: &EntityDiffParams) {
        self.diff_labels(i1, i2, params);
        self.diff_descriptions(i1, i2, params);
        self.diff_aliases(i1, i2, params);
        //self.diff_claims(i1, i2, params);
        //self.diff_sitelinks(i1, i2, params);
    }

    fn diff_labels(&mut self, i1: &Entity, i2: &Entity, params: &EntityDiffParams) {
        self.diff_locales(i1.labels(), i2.labels(), &params.labels, "labels");
    }

    fn diff_descriptions(&mut self, i1: &Entity, i2: &Entity, params: &EntityDiffParams) {
        self.diff_locales(
            i1.descriptions(),
            i2.descriptions(),
            &params.descriptions,
            "descriptions",
        );
    }

    fn diff_aliases(&mut self, i1: &Entity, i2: &Entity, params: &EntityDiffParams) {
        self.diff_locales(i1.aliases(), i2.aliases(), &params.aliases, "aliases");
    }

    fn diff_locales(
        &mut self,
        l1: &Vec<LocaleString>,
        l2: &Vec<LocaleString>,
        params: &EntityDiffParam,
        mode: &str,
    ) {
        // Round 1: Add new (all modes) or alter existing (all modes except aliases) language values
        for s1 in l1 {
            let mut found = false;
            for s2 in l2 {
                if s1.language() == s2.language() {
                    if s1.value() == s2.value() {
                        found = true;
                    } else if mode != "aliases" && params.valid(&s1.language().to_string(), "alter")
                    {
                        // ALTERED s1 => s2 (not for aliases)
                        self.j[mode][s2.language()] =
                            json!({"language":s2.language(),"value":s2.value()});
                    }
                }
            }
            if !found && params.valid(&s1.language().to_string(), "remove") {
                if mode == "alias" {
                    panic!("TODO: EntityDiff.diff_locales REMOVE for aliases");
                } else {
                    self.j[mode][s1.language()] =
                        json!({"language":s1.language(),"value":s1.value(),"remove":""});
                }
            }
        }

        // Round 2: Remove old lanugage values
        for s2 in l2 {
            let mut found = false;
            for s1 in l1 {
                if s1 == s2 {
                    found = true;
                }
            }
            if !found && params.valid(&s2.language().to_string(), "add") {
                if mode == "alias" {
                    panic!("TODO: EntityDiff.diff_locales ADD for aliases");
                } else {
                    self.j[mode][s2.language()] =
                        json!({"language":s2.language(),"value":s2.value()});
                }
            }
        }
    }
}
