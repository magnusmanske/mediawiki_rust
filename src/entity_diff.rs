extern crate lazy_static;

use crate::api::Api;
use std::collections::HashMap;
use wikibase::*;

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

    pub fn valid<S: Into<String>>(&self, key: S, action: &str) -> bool {
        lazy_static! {
            static ref STAR: String = "*".to_string();
        }
        match action {
            "add" => self.add.contains(&key.into()) || self.add.contains(&STAR),
            "remove" => self.remove.contains(&key.into()) || self.add.contains(&STAR),
            "alter" => self.alter.contains(&key.into()) || self.add.contains(&STAR),
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

#[derive(Debug)]
enum EntityDiffClaimComparison {
    Same,
    Similar,
    Different,
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

    pub fn as_str(&self) -> Result<String, serde_json::Error> {
        ::serde_json::to_string(&self.j)
    }

    fn diff(&mut self, i1: &Entity, i2: &Entity, params: &EntityDiffParams) {
        self.diff_labels(i1, i2, params);
        self.diff_descriptions(i1, i2, params);
        self.diff_aliases(i1, i2, params);
        self.diff_claims(i1, i2, params);
        self.diff_sitelinks(i1, i2, params);
    }

    fn compare_snak_values(
        &self,
        value1: &Value,
        value2: &Value,
        _params: &EntityDiffParams,
    ) -> EntityDiffClaimComparison {
        match (value1, value2) {
            (Value::Coordinate(v1), Value::Coordinate(v2)) => {
                // Ignoting altitude and precision
                if v1.globe() == v2.globe()
                    && v1.latitude() == v2.latitude()
                    && v1.longitude() == v2.longitude()
                {
                    return EntityDiffClaimComparison::Same;
                }
            }
            (Value::MonoLingual(v1), Value::MonoLingual(v2)) => {
                if v1 == v2 {
                    return EntityDiffClaimComparison::Same;
                }
            }
            (Value::Entity(v1), Value::Entity(v2)) => {
                if v1 == v2 {
                    return EntityDiffClaimComparison::Same;
                }
            }
            (Value::Quantity(v1), Value::Quantity(v2)) => {
                // Ignoring upper and lower bound
                if v1.amount() == v2.amount() && v1.unit() == v2.unit() {
                    return EntityDiffClaimComparison::Same;
                }
            }
            (Value::StringValue(v1), Value::StringValue(v2)) => {
                if v1 == v2 {
                    return EntityDiffClaimComparison::Same;
                }
            }
            (Value::Time(v1), Value::Time(v2)) => {
                // Ignoring before/after/timezone
                if v1.calendarmodel() == v2.calendarmodel()
                    && v1.precision() == v2.precision()
                    && v1.time() == v2.time()
                {
                    return EntityDiffClaimComparison::Same;
                }
            }
            _ => {}
        }
        // Not the same => different
        EntityDiffClaimComparison::Different
    }

    fn compare_snaks(
        &self,
        s1: &Snak,
        s2: &Snak,
        params: &EntityDiffParams,
    ) -> EntityDiffClaimComparison {
        // TODO params
        if s1.property() != s2.property() {
            return EntityDiffClaimComparison::Different;
        }
        if s1.datatype() != s2.datatype() {
            return EntityDiffClaimComparison::Different;
        }
        if s1.snak_type() != s2.snak_type() {
            return EntityDiffClaimComparison::Different;
        }
        match (s1.data_value(), s2.data_value()) {
            (None, None) => EntityDiffClaimComparison::Same,
            (None, Some(_)) => EntityDiffClaimComparison::Different,
            (Some(_), None) => EntityDiffClaimComparison::Different,
            (Some(dv1), Some(dv2)) => {
                if dv1.value_type() != dv2.value_type() {
                    return EntityDiffClaimComparison::Different;
                }
                self.compare_snak_values(dv1.value(), dv2.value(), params)
            }
        }
    }

    fn compare_qualifiers(
        &self,
        qualifiers1: &Vec<Snak>,
        qualifiers2: &Vec<Snak>,
        params: &EntityDiffParams,
    ) -> EntityDiffClaimComparison {
        for q1 in qualifiers1 {
            let mut found = false;
            for q2 in qualifiers2 {
                match self.compare_snaks(q1, q2, params) {
                    EntityDiffClaimComparison::Same => found = true,
                    _ => {}
                }
            }
            if !found {
                return EntityDiffClaimComparison::Similar;
            }
        }
        for q2 in qualifiers2 {
            let mut found = false;
            for q1 in qualifiers1 {
                match self.compare_snaks(q1, q2, params) {
                    EntityDiffClaimComparison::Same => found = true,
                    _ => {}
                }
            }
            if !found {
                return EntityDiffClaimComparison::Similar;
            }
        }
        EntityDiffClaimComparison::Same
    }

    fn compare_references(
        &self,
        _references1: &Vec<Reference>,
        _references2: &Vec<Reference>,
        _params: &EntityDiffParams,
    ) -> EntityDiffClaimComparison {
        // TODO
        EntityDiffClaimComparison::Same
    }

    fn compare_claims(
        &self,
        s1: &Statement,
        s2: &Statement,
        params: &EntityDiffParams,
    ) -> EntityDiffClaimComparison {
        if s1.claim_type() != s2.claim_type() {
            return EntityDiffClaimComparison::Different;
        }
        match self.compare_snaks(s1.main_snak(), s2.main_snak(), params) {
            EntityDiffClaimComparison::Same => {}
            ret => return ret,
        }

        // Now either Same or Similar; return Similar if mismatch is found
        if s1.rank() != s2.rank() {
            return EntityDiffClaimComparison::Similar;
        }
        match self.compare_qualifiers(s1.qualifiers(), s2.qualifiers(), params) {
            EntityDiffClaimComparison::Same => {}
            ret => return ret,
        }
        match self.compare_references(s1.references(), s2.references(), params) {
            EntityDiffClaimComparison::Same => {}
            ret => return ret,
        }
        EntityDiffClaimComparison::Same
    }

    fn get_claim_property(&self, s: &Statement) -> String {
        s.main_snak().property().to_string()
    }

    fn diff_claims(&mut self, i1: &Entity, i2: &Entity, params: &EntityDiffParams) {
        // Round 1: Remove old, and alter existing
        for c1 in i1.claims() {
            let mut found = false;
            for c2 in i2.claims() {
                match self.compare_claims(&c1, &c2, &params) {
                    EntityDiffClaimComparison::Same => found = true,
                    EntityDiffClaimComparison::Similar => {
                        if params.claims.valid(self.get_claim_property(&c1), "alter") {
                            // TODO alter c1 => c2
                        }
                        found = true;
                    }
                    EntityDiffClaimComparison::Different => {}
                }
            }
            if !found {
                if params.claims.valid(self.get_claim_property(&c1), "remove") {
                    // TODO remove c1
                }
            }
        }

        // Round 2: Add new
        for c2 in i2.claims() {
            let mut found = false;
            for c1 in i1.claims() {
                match self.compare_claims(&c1, &c2, params) {
                    EntityDiffClaimComparison::Same => found = true,
                    EntityDiffClaimComparison::Similar => found = true, // Taken care of in Round 1
                    _ => {}
                }
            }
            if !found {
                if params.claims.valid(self.get_claim_property(&c2), "add") {
                    if !self.j["claims"].is_array() {
                        self.j["claims"] = json!([]);
                    }
                    let v = c2.as_stripped_json();
                    self.j["claims"].as_array_mut().unwrap().push(v);
                }
            }
        }
    }

    fn diff_sitelinks(&mut self, i1: &Entity, i2: &Entity, params: &EntityDiffParams) {
        match (i1.sitelinks(), i2.sitelinks()) {
            (Some(sl1), Some(sl2)) => {
                // Round 1: Remove old, and alter existing
                for s1 in sl1 {
                    let mut found = false;
                    for s2 in sl2 {
                        if s1 == s2 {
                            found = true;
                        } else if s1.site() == s2.site()
                            && params.sitelinks.valid(s2.site().as_str(), "alter")
                        {
                            self.j["sitelinks"][s1.site()] = serde_json::to_value(&s2).unwrap();
                            found = true;
                        }
                    }
                    if !found && params.sitelinks.valid(s1.site().as_str(), "remove") {
                        self.j["sitelinks"][s1.site()] = serde_json::to_value(&s1).unwrap();
                        self.j["sitelinks"][s1.site()]["remove"] = json!("");
                    }
                }

                // Round 2: Add new
                for s2 in sl2 {
                    let mut found = false;
                    for s1 in sl1 {
                        if s1 == s2 {
                            found = true;
                        }
                    }
                    if !found && params.sitelinks.valid(s2.site().as_str(), "add") {
                        self.j["sitelinks"][s2.site()] = serde_json::to_value(&s2).unwrap();
                    }
                }
            }
            (Some(sl1), None) => {
                // Remove all sitelinks
                for sl in sl1 {
                    if params.sitelinks.valid(sl.site().as_str(), "remove") {
                        self.j["sitelinks"][sl.site()] =
                            json!({"site":sl.site(),"title":sl.title(),"remove":""});
                    }
                }
            }
            (None, Some(sl2)) => {
                // Add all sitelinks
                for sl in sl2 {
                    if params.sitelinks.valid(sl.site().as_str(), "add") {
                        self.j["sitelinks"][sl.site()] = serde_json::to_value(&sl).unwrap();
                    }
                }
            }
            (None, None) => {}
        }
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
        // Round 1:  Remove old (all modes) or alter existing (all modes except aliases) language values
        for s1 in l1 {
            let mut found = false;
            for s2 in l2 {
                if s1.language() == s2.language() {
                    if s1.value() == s2.value() {
                        found = true;
                    } else if mode != "aliases" && params.valid(s1.language(), "alter") {
                        self.j[mode][s2.language()] =
                            json!({"language":s2.language(),"value":s2.value()});
                        found = true;
                    }
                }
            }
            if !found && params.valid(s1.language(), "remove") {
                let v = json!({"language":s1.language(),"value":s1.value(),"remove":""});
                if mode == "aliases" {
                    // TODO check this
                    if !self.j[mode].is_array() {
                        self.j[mode] = json!([]);
                    }
                    self.j[mode].as_array_mut().unwrap().push(v);
                } else {
                    if !self.j[mode].is_array() {
                        self.j[mode] = json!([]);
                    }
                    self.j[mode].as_array_mut().unwrap().push(v);
                }
            }
        }

        // Round 2: Add new lanugage values
        for s2 in l2 {
            let mut found = false;
            for s1 in l1 {
                if s1 == s2 {
                    found = true;
                }
            }
            if !found && params.valid(s2.language(), "add") {
                let v = json!({"language":s2.language(),"value":s2.value()});
                if mode == "aliases" {
                    // TODO check this
                    if !self.j[mode].is_array() {
                        self.j[mode] = json!([]);
                    }
                    self.j[mode].as_array_mut().unwrap().push(v);
                } else {
                    if !self.j[mode].is_array() {
                        self.j[mode] = json!([]);
                    }
                    self.j[mode].as_array_mut().unwrap().push(v);
                }
            }
        }
    }

    pub fn apply_diff(
        mw_api: &mut Api,
        diff: &EntityDiff,
        edit_target: EditTarget,
    ) -> Result<serde_json::Value, Box<::std::error::Error>> {
        let json = diff.as_str().unwrap();
        let token = mw_api.get_edit_token().unwrap();
        let mut params: HashMap<_, _> = vec![
            ("action", "wbeditentity"),
            ("data", &json),
            ("token", &token),
        ]
        .into_iter()
        .collect();

        let nk: String;
        let nv: String;
        match edit_target {
            EditTarget::Entity(id) => {
                nk = "id".to_string();
                nv = id;
            }
            EditTarget::New(entity_type) => {
                nk = "new".to_string();
                nv = entity_type;
            }
        };
        params.insert(nk.as_str(), nv.as_str());

        let res = mw_api.post_query_api_json(&params)?;

        match res["success"].as_i64() {
            Some(num) => {
                if num == 1 {
                    // Success, now use updated item JSON
                    match &res["entity"] {
                        serde_json::Value::Null => {}
                        entity_json => {
                            return Ok(entity_json.to_owned());
                        }
                    };
                }
            }
            None => {}
        }

        Err(From::from(format!(
            "Failed to apply diff '{:?}', result:{:?}",
            &diff, &res
        )))
    }

    pub fn get_entity_id(entity_json: &serde_json::Value) -> Option<String> {
        match &entity_json["id"] {
            serde_json::Value::String(s) => Some(s.to_string()),
            _ => None,
        }
    }
}

pub enum EditTarget {
    Entity(String),
    New(String),
}

#[cfg(test)]
mod tests {
    use super::{EntityDiff, EntityDiffParam, EntityDiffParams};
    use crate::api;
    use crate::entity_container;
    use wikibase::Entity;

    #[test]
    fn misc() {
        let api = api::Api::new("https://www.wikidata.org/w/api.php").unwrap();
        let mut ec = entity_container::EntityContainer::new();
        let i = ec.load_entity(&api, "Q42").unwrap();

        //
        let mut new_i = Entity::new_empty();
        new_i.set_label(wikibase::LocaleString::new("en", "testing"));

        let mut params = EntityDiffParams::none();
        params.labels = EntityDiffParam::some(&vec!["en".to_string()]);

        let diff = EntityDiff::new(&i, &new_i, &params);
        assert_eq!(
            r#"{"labels":{"en":{"language":"en","value":"testing"}}}"#,
            serde_json::to_string(diff.actions()).unwrap()
        );

        //
        let mut new_i = Entity::new_empty();
        new_i.set_label(wikibase::LocaleString::new("en", "Douglas Adams"));
        let diff = EntityDiff::new(&i, &new_i, &params);
        assert_eq!(r#"{}"#, serde_json::to_string(diff.actions()).unwrap());

        //
        let mut new_i = Entity::new_empty();
        new_i.set_sitelink(wikibase::SiteLink::new("enwiki", "Test123", vec![]));
        let mut params = EntityDiffParams::none();
        params.sitelinks = EntityDiffParam::some(&vec!["enwiki".to_string()]);
        let diff = EntityDiff::new(&i, &new_i, &params);
        assert_eq!(
            r#"{"sitelinks":{"enwiki":{"badges":[],"site":"enwiki","title":"Test123"}}}"#,
            serde_json::to_string(diff.actions()).unwrap()
        );
    }
}
