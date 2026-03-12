/*!
The `Revision` class deals with page revisions.
*/

#![deny(missing_docs)]

use chrono::NaiveDateTime;
use serde_json::Value;

use crate::MediaWikiError;

/// The revision properties to fetch.
pub(crate) const RVPROP: &str = "ids|content|timestamp|size|sha1|comment|tags|user|userid";

/// Repesents a revision of a page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Revision {
    id: u64,
    parent_id: Option<u64>,
    wikitext: Option<String>,
    timestamp: Option<NaiveDateTime>,
    size: Option<usize>,
    sha1: Option<String>,
    tags: Vec<String>,
    user: Option<String>,
    userid: Option<u64>,
}

impl Revision {
    /// Creates a new revision from API-returned JSON.
    pub fn from_json(j: &Value) -> Result<Self, MediaWikiError> {
        let id = j["revid"]
            .as_u64()
            .ok_or_else(|| MediaWikiError::UnexpectedResultFormat("No revision ID".to_string()))?;
        Ok(Self {
            id,
            parent_id: j["parentid"].as_u64(),
            wikitext: j["slots"]["main"]["content"]
                .as_str()
                .map(|s| s.to_string()),
            timestamp: j["timestamp"]
                .as_str()
                .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%SZ").ok()),
            size: j["size"].as_u64().map(|s| s as usize),
            sha1: j["sha1"].as_str().map(|s| s.to_string()),
            user: j["user"].as_str().map(|s| s.to_string()),
            userid: j["userid"].as_u64(),
            tags: j["tags"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| Some(v.as_str()?.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
        })
    }

    /// Returns the revision ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns the parent revision ID.
    pub fn parent_id(&self) -> Option<u64> {
        self.parent_id
    }

    /// Returns the timestamp of the revision.
    pub fn timestamp(&self) -> Option<&NaiveDateTime> {
        self.timestamp.as_ref()
    }

    /// Returns the wikitext of the revision.
    pub fn wikitext(&self) -> Option<&str> {
        self.wikitext.as_deref()
    }

    /// Returns the size of the revision in bytes.
    pub fn size(&self) -> Option<usize> {
        self.size
    }

    /// Returns the SHA-1 hash of the revision content.
    pub fn sha1(&self) -> Option<&str> {
        self.sha1.as_deref()
    }

    /// Returns the username of the user who made the revision.
    pub fn user(&self) -> Option<&str> {
        self.user.as_deref()
    }

    /// Returns the user ID of the user who made the revision.
    pub fn userid(&self) -> Option<u64> {
        self.userid
    }

    /// Returns the tags associated with this revision.
    pub fn tags(&self) -> &[String] {
        &self.tags
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn from_json_full() {
        let j = json!({
            "revid": 42,
            "parentid": 41,
            "slots": {"main": {"content": "hello world"}},
            "timestamp": "2024-01-15T10:30:00Z",
            "size": 1234,
            "sha1": "abc123",
            "user": "TestUser",
            "userid": 99,
            "tags": ["tag1", "tag2"]
        });
        let rev = Revision::from_json(&j).unwrap();
        assert_eq!(rev.id(), 42);
        assert_eq!(rev.parent_id(), Some(41));
        assert_eq!(rev.wikitext(), Some("hello world"));
        assert!(rev.timestamp().is_some());
        assert_eq!(rev.size(), Some(1234));
        assert_eq!(rev.sha1(), Some("abc123"));
        assert_eq!(rev.user(), Some("TestUser"));
        assert_eq!(rev.userid(), Some(99));
        assert_eq!(rev.tags(), &["tag1", "tag2"]);
    }

    #[test]
    fn from_json_minimal() {
        let j = json!({"revid": 1});
        let rev = Revision::from_json(&j).unwrap();
        assert_eq!(rev.id(), 1);
        assert_eq!(rev.parent_id(), None);
        assert_eq!(rev.wikitext(), None);
        assert!(rev.timestamp().is_none());
        assert_eq!(rev.size(), None);
        assert_eq!(rev.sha1(), None);
        assert_eq!(rev.user(), None);
        assert_eq!(rev.userid(), None);
        assert!(rev.tags().is_empty());
    }

    #[test]
    fn from_json_missing_revid_is_error() {
        let j = json!({"parentid": 1});
        assert!(Revision::from_json(&j).is_err());
    }

    #[test]
    fn from_json_empty_tags() {
        let j = json!({"revid": 1, "tags": []});
        let rev = Revision::from_json(&j).unwrap();
        assert!(rev.tags().is_empty());
    }
}
