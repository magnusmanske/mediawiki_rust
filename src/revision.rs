/*!
The [`Revision`] class deals with page revisions.
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
}
