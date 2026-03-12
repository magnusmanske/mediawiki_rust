/*!
Shared utilities and trait for [`Api`](crate::Api) and [`ApiSync`](crate::ApiSync).
*/

use crate::media_wiki_error::MediaWikiError;
use crate::title::Title;
use crate::user::User;
use serde_json::Value;
use std::collections::HashMap;

/// Alias for a namespace ID (may be negative, e.g. -1 for Special pages).
pub type NamespaceID = i64;

/// Common interface implemented by both [`Api`](crate::Api) and [`ApiSync`](crate::ApiSync).
///
/// Covers all pure (non-async, non-HTTP) methods whose implementations are identical
/// between the two types. Import via [`prelude::*`](crate::prelude) to use it.
pub trait MediaWikiApi {
    /// Returns the API URL.
    fn api_url(&self) -> &str;

    /// Returns a reference to the site info JSON.
    fn get_site_info(&self) -> &Value;

    /// Returns a reference to the current user object.
    fn user(&self) -> &User;

    /// Returns the user agent name.
    fn user_agent(&self) -> &str;

    /// Returns the maxlag setting in seconds, if set.
    fn maxlag(&self) -> &Option<u64>;

    /// Returns the edit delay in milliseconds, if set.
    fn edit_delay(&self) -> &Option<u64>;

    /// Returns the maximum number of retry attempts.
    fn max_retry_attempts(&self) -> u64;

    // --- Default implementations ---

    /// Returns the full user agent string as passed to the API via HTTP header.
    fn user_agent_full(&self) -> String {
        format!(
            "{}; {}-rust/{}",
            self.user_agent(),
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
    }

    /// Returns a serde_json Value in site info, within the `["query"]` object.
    fn get_site_info_value<'a>(&'a self, k1: &str, k2: &str) -> &'a Value {
        &self.get_site_info()["query"][k1][k2]
    }

    /// Returns a string from the site info, matching `["query"][k1][k2]`.
    fn get_site_info_string<'a>(
        &'a self,
        k1: &str,
        k2: &str,
    ) -> Result<&'a str, MediaWikiError> {
        match self.get_site_info_value(k1, k2).as_str() {
            Some(s) => Ok(s),
            None => Err(MediaWikiError::String(format!(
                "No 'query.{}.{}' value in site info",
                k1, k2
            ))),
        }
    }

    /// Returns the raw data for the namespace, matching `["query"]["namespaces"][namespace_id]`.
    fn get_namespace_info(&self, namespace_id: NamespaceID) -> &Value {
        self.get_site_info_value("namespaces", &namespace_id.to_string())
    }

    /// Returns the canonical namespace name for a namespace ID, if defined.
    fn get_canonical_namespace_name(&self, namespace_id: NamespaceID) -> Option<&str> {
        let info = self.get_namespace_info(namespace_id);
        info["canonical"].as_str().or_else(|| info["*"].as_str())
    }

    /// Returns the local namespace name for a namespace ID, if defined.
    fn get_local_namespace_name(&self, namespace_id: NamespaceID) -> Option<&str> {
        let info = self.get_namespace_info(namespace_id);
        info["*"].as_str().or_else(|| info["canonical"].as_str())
    }

    /// Turns a slice of `(&str, &str)` tuples into a `HashMap<String, String>`.
    fn params_into(&self, params: &[(&str, &str)]) -> HashMap<String, String> {
        params
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    /// Returns an empty parameter `HashMap`.
    fn no_params(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    /// Given a URI pointing to a Wikibase entity on this installation, returns the entity ID.
    fn extract_entity_from_uri(&self, uri: &str) -> Result<String, MediaWikiError> {
        let concept_base_uri =
            self.get_site_info_string("general", "wikibase-conceptbaseuri")?;
        match uri.strip_prefix(concept_base_uri) {
            Some(s) => Ok(s.to_string()),
            None => Err(From::from(format!(
                "{} does not start with {}",
                uri, concept_base_uri
            ))),
        }
    }

    /// Returns a vector of entity IDs from a SPARQL result, given a variable name.
    fn entities_from_sparql_result(
        &self,
        sparql_result: &Value,
        variable_name: &str,
    ) -> Vec<String> {
        let mut entities = vec![];
        if let Some(bindings) = sparql_result["results"]["bindings"].as_array() {
            for b in bindings {
                if let Some(entity_url) = b[variable_name]["value"].as_str() {
                    if let Ok(entity) = self.extract_entity_from_uri(entity_url) {
                        entities.push(entity);
                    }
                }
            }
        }
        entities
    }
}

/// Merges two JSON objects that are MediaWiki API results.
///
/// If an array already exists in `a`, it is expanded with the array from `b`.
/// This allows combining multiple paginated API results via the `continue` parameter.
pub(crate) fn json_merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            if let Some(a) = a.as_object_mut() {
                for (k, v) in b {
                    json_merge(a.entry(k).or_insert(Value::Null), v);
                }
            }
        }
        (a @ &mut Value::Array(_), Value::Array(b)) => {
            if let Some(a) = a.as_array_mut() {
                for v in b {
                    a.push(v);
                }
            }
        }
        (a, b) => *a = b,
    }
}

/// Returns `true` if the query is an edit (POST with a `token` parameter).
pub(crate) fn is_edit_query(params: &HashMap<String, String>, method: &str) -> bool {
    method == "POST" && params.contains_key("token")
}

/// Checks for a maxlag error; returns the lag in seconds if present.
pub(crate) fn check_maxlag(v: &Value, maxlag_seconds: Option<u64>) -> Option<u64> {
    match v["error"]["code"].as_str() {
        Some("maxlag") => v["error"]["lag"].as_u64().or(maxlag_seconds),
        _ => None,
    }
}

/// Inserts the cumulative `maxlag` parameter into an edit query's params.
pub(crate) fn set_cumulative_maxlag_params(
    params: &mut HashMap<String, String>,
    method: &str,
    maxlag_seconds: Option<u64>,
    cumulative: u64,
) {
    if !is_edit_query(params, method) {
        return;
    }
    if let Some(maxlag) = maxlag_seconds {
        params.insert("maxlag".to_string(), (cumulative + maxlag).to_string());
    }
}

/// Percent-encodes a string.
pub(crate) fn rawurlencode(s: &str) -> String {
    urlencoding::encode(s).into_owned()
}

/// From an API result with `"title"` and `"ns"` entries, returns a `Vec<Title>`.
pub fn result_array_to_titles(data: &Value) -> Vec<Title> {
    if let Some(obj) = data.as_object() {
        obj.iter()
            .flat_map(|(_k, v)| result_array_to_titles(v))
            .collect()
    } else if let Some(arr) = data.as_array() {
        arr.iter().map(Title::new_from_api_result).collect()
    } else {
        vec![]
    }
}
