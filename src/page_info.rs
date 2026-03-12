/*!
Typed representations of MediaWiki [`action=query&prop=info`][mw-info] results.

[`PageInfo`] maps every field the API can return for a single page (both the
default fields and those activated by `inprop`). Any fields not explicitly
modelled are captured in the [`extra`](PageInfo::extra) catch-all so nothing
is ever lost.

[`PageInfoList`] collects [`PageInfo`] items across one or more paginated API
responses. You can populate it manually with [`add_from_result`] or let it
drive continuation automatically with [`fetch_all`] / [`fetch_all_sync`].

[mw-info]: https://www.mediawiki.org/wiki/API:Info

# Quick start

```rust
# tokio::runtime::Runtime::new().unwrap().block_on(async {
use mediawiki::prelude::*;
use mediawiki::page_info::PageInfoList;

let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();

let result = ActionApiQuery::info()
    .inprop(&["protection", "url", "displaytitle"])
    .titles(&["Albert Einstein", "Physics"])
    .run(&api)
    .await
    .unwrap();

let list = PageInfoList::from_result(&result);
for page in list.pages() {
    println!("{} (id {}): {}", page.title, page.pageid.unwrap_or(0), page.fullurl.as_deref().unwrap_or("n/a"));
}
# });
```

# Auto-continuation

```rust
# tokio::runtime::Runtime::new().unwrap().block_on(async {
use mediawiki::prelude::*;
use mediawiki::page_info::PageInfoList;

let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();

let builder = ActionApiQuery::info()
    .inprop(&["protection", "url", "displaytitle"])
    .titles(&["Albert Einstein", "Physics"]);

let list = PageInfoList::fetch_all(&builder, &api, None).await.unwrap();
assert!(!list.pages().is_empty());
# });
```
*/

#![deny(missing_docs)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::page_query::{PageQueryResult, PageQueryResultList};

/// A single protection entry as returned by `inprop=protection`.
///
/// Each entry describes one protection level applied to one action type.
///
/// # Example JSON
///
/// ```json
/// {"type": "edit", "level": "autoconfirmed", "expiry": "infinity"}
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectionEntry {
    /// The action being restricted (e.g. `"edit"`, `"move"`, `"upload"`).
    #[serde(rename = "type")]
    pub protection_type: String,
    /// The minimum user level required (e.g. `"autoconfirmed"`, `"sysop"`).
    pub level: String,
    /// When the protection expires — `"infinity"` or an ISO 8601 timestamp.
    #[serde(default)]
    pub expiry: String,
}

/// Typed representation of one page in a `prop=info` API response.
///
/// All default fields plus every `inprop`-gated field are represented. Fields
/// that require a specific `inprop` value are `Option`; they will be `None`
/// when the corresponding `inprop` was not requested. Any API fields not
/// explicitly modelled end up in [`extra`](Self::extra).
///
/// # Default fields (always present)
///
/// | Field | Type |
/// |---|---|
/// | `pageid` | `Option<u64>` (absent for missing pages) |
/// | `ns` | `i64` |
/// | `title` | `String` |
/// | `contentmodel` | `String` |
/// | `pagelanguage` | `String` |
/// | `pagelanguagehtmlcode` | `String` |
/// | `pagelanguagedir` | `String` |
/// | `touched` | `Option<String>` |
/// | `lastrevid` | `Option<u64>` |
/// | `length` | `Option<u64>` |
///
/// # Fields from `inprop`
///
/// | `inprop` value | Fields added |
/// |---|---|
/// | `protection` | `protection`, `restrictiontypes` |
/// | `talkid` | `talkid` |
/// | `subjectid` | `subjectid` |
/// | `associatedpage` | `associatedpage` |
/// | `url` | `fullurl`, `editurl`, `canonicalurl` |
/// | `displaytitle` | `displaytitle` |
/// | `varianttitles` | `varianttitles` |
/// | `watched` | `watched` |
/// | `watchers` | `watchers` |
/// | `visitingwatchers` | `visitingwatchers` |
/// | `notificationtimestamp` | `notificationtimestamp` |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageInfo {
    // --- Default fields ---
    /// Page ID. Absent (`None`) for missing pages.
    pub pageid: Option<u64>,
    /// Namespace ID.
    #[serde(default)]
    pub ns: i64,
    /// Full page title including namespace prefix.
    #[serde(default)]
    pub title: String,
    /// Content model (e.g. `"wikitext"`, `"json"`).
    #[serde(default)]
    pub contentmodel: String,
    /// Page language code (e.g. `"en"`).
    #[serde(default)]
    pub pagelanguage: String,
    /// Page language HTML code.
    #[serde(default)]
    pub pagelanguagehtmlcode: String,
    /// Page language direction (`"ltr"` or `"rtl"`).
    #[serde(default)]
    pub pagelanguagedir: String,
    /// Last-touched timestamp (ISO 8601). Absent for missing pages.
    pub touched: Option<String>,
    /// ID of the latest revision. Absent for missing pages.
    pub lastrevid: Option<u64>,
    /// Page length in bytes. Absent for missing pages.
    pub length: Option<u64>,

    // --- Flags ---
    /// `true` if the page does not exist.
    #[serde(default, deserialize_with = "deserialize_mw_bool")]
    pub missing: bool,
    /// `true` if the page has been newly created.
    #[serde(default, deserialize_with = "deserialize_mw_bool")]
    pub new: bool,
    /// `true` if the page is a redirect.
    #[serde(default, deserialize_with = "deserialize_mw_bool")]
    pub redirect: bool,

    // --- inprop=protection ---
    /// Protection levels. Requires `inprop=protection`.
    #[serde(default)]
    pub protection: Vec<ProtectionEntry>,
    /// Restriction types applicable to this page (e.g. `["edit", "move"]`).
    /// Requires `inprop=protection`.
    #[serde(default)]
    pub restrictiontypes: Vec<String>,

    // --- inprop=talkid ---
    /// Page ID of the corresponding talk page. Requires `inprop=talkid`.
    pub talkid: Option<u64>,

    // --- inprop=subjectid ---
    /// Page ID of the corresponding subject/content page. Requires `inprop=subjectid`.
    pub subjectid: Option<u64>,

    // --- inprop=associatedpage ---
    /// Title of the associated page (talk ↔ subject). Requires `inprop=associatedpage`.
    pub associatedpage: Option<String>,

    // --- inprop=url ---
    /// Full URL to the page. Requires `inprop=url`.
    pub fullurl: Option<String>,
    /// URL to edit the page. Requires `inprop=url`.
    pub editurl: Option<String>,
    /// Canonical URL. Requires `inprop=url`.
    pub canonicalurl: Option<String>,

    // --- inprop=displaytitle ---
    /// Display title (may contain HTML). Requires `inprop=displaytitle`.
    pub displaytitle: Option<String>,

    // --- inprop=varianttitles ---
    /// Variant titles keyed by language code. Requires `inprop=varianttitles`.
    pub varianttitles: Option<HashMap<String, String>>,

    // --- inprop=watched ---
    /// Whether the current user watches this page. Requires `inprop=watched`.
    pub watched: Option<bool>,

    // --- inprop=watchers ---
    /// Number of watchers. Requires `inprop=watchers` (may need privileges).
    pub watchers: Option<u64>,

    // --- inprop=visitingwatchers ---
    /// Number of watchers who have visited recent edits. Requires `inprop=visitingwatchers`.
    pub visitingwatchers: Option<u64>,

    // --- inprop=notificationtimestamp ---
    /// Watchlist notification timestamp. Requires `inprop=notificationtimestamp`.
    pub notificationtimestamp: Option<String>,

    /// Any additional fields not covered by the typed fields above.
    ///
    /// This ensures forward compatibility: new API fields are captured here
    /// automatically even if this struct has not been updated yet.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl PageQueryResult for PageInfo {
    fn from_page_value(page: &Value) -> Vec<Self> {
        serde_json::from_value(page.clone()).into_iter().collect()
    }
}

/// Custom deserializer for MediaWiki boolean fields.
///
/// In `formatversion=1`, boolean flags are present as `""` (empty string) when
/// true, and absent when false. In `formatversion=2`, they are proper booleans.
fn deserialize_mw_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = Value::deserialize(deserializer)?;
    match v {
        Value::Bool(b) => Ok(b),
        Value::String(_) => Ok(true), // formatversion=1: presence as "" means true
        Value::Null => Ok(false),
        _ => Ok(false),
    }
}

/// A collection of [`PageInfo`] items parsed from one or more API responses.
///
/// This is a type alias for [`PageQueryResultList<PageInfo>`]. Use
/// [`from_result`](PageQueryResultList::from_result) to create from a single response,
/// [`add_from_result`](PageQueryResultList::add_from_result) to append from additional pages,
/// or [`fetch_all`](PageQueryResultList::fetch_all) /
/// [`fetch_all_sync`](PageQueryResultList::fetch_all_sync) to automatically paginate.
///
/// # Example
///
/// ```rust
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use mediawiki::prelude::*;
/// use mediawiki::page_info::PageInfoList;
///
/// let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
///
/// let result = ActionApiQuery::info()
///     .inprop(&["protection", "url"])
///     .titles(&["Albert Einstein"])
///     .run(&api)
///     .await
///     .unwrap();
///
/// let list = PageInfoList::from_result(&result);
/// assert!(!list.pages().is_empty());
/// # });
/// ```
pub type PageInfoList = PageQueryResultList<PageInfo>;

impl PageInfoList {
    /// Returns a slice of all collected [`PageInfo`] items.
    ///
    /// This is a convenience alias for [`items()`](PageQueryResultList::items).
    pub fn pages(&self) -> &[PageInfo] {
        self.items()
    }

    /// Returns a mutable reference to the inner `Vec<PageInfo>`.
    ///
    /// This is a convenience alias for [`items_mut()`](PageQueryResultList::items_mut).
    pub fn pages_mut(&mut self) -> &mut Vec<PageInfo> {
        self.items_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // --- PageInfo deserialization ---

    #[test]
    fn deserialize_full_page() {
        let j = json!({
            "pageid": 22939,
            "ns": 0,
            "title": "Physics",
            "contentmodel": "wikitext",
            "pagelanguage": "en",
            "pagelanguagehtmlcode": "en",
            "pagelanguagedir": "ltr",
            "touched": "2024-01-15T10:30:00Z",
            "lastrevid": 1234567,
            "length": 90355,
            "protection": [
                {"type": "edit", "level": "autoconfirmed", "expiry": "infinity"},
                {"type": "move", "level": "sysop", "expiry": "infinity"}
            ],
            "restrictiontypes": ["edit", "move"],
            "talkid": 21492466,
            "associatedpage": "Talk:Physics",
            "fullurl": "https://en.wikipedia.org/wiki/Physics",
            "editurl": "https://en.wikipedia.org/w/index.php?title=Physics&action=edit",
            "canonicalurl": "https://en.wikipedia.org/wiki/Physics",
            "displaytitle": "Physics",
            "varianttitles": {"en": "Physics"}
        });
        let info: PageInfo = serde_json::from_value(j).unwrap();
        assert_eq!(info.pageid, Some(22939));
        assert_eq!(info.ns, 0);
        assert_eq!(info.title, "Physics");
        assert_eq!(info.contentmodel, "wikitext");
        assert_eq!(info.pagelanguage, "en");
        assert_eq!(info.pagelanguagedir, "ltr");
        assert_eq!(info.touched.as_deref(), Some("2024-01-15T10:30:00Z"));
        assert_eq!(info.lastrevid, Some(1234567));
        assert_eq!(info.length, Some(90355));
        assert!(!info.missing);
        assert!(!info.new);
        assert!(!info.redirect);
        assert_eq!(info.protection.len(), 2);
        assert_eq!(info.protection[0].protection_type, "edit");
        assert_eq!(info.protection[0].level, "autoconfirmed");
        assert_eq!(info.protection[1].protection_type, "move");
        assert_eq!(info.restrictiontypes, vec!["edit", "move"]);
        assert_eq!(info.talkid, Some(21492466));
        assert_eq!(info.associatedpage.as_deref(), Some("Talk:Physics"));
        assert!(info.fullurl.as_ref().unwrap().contains("Physics"));
        assert!(info.editurl.is_some());
        assert!(info.canonicalurl.is_some());
        assert_eq!(info.displaytitle.as_deref(), Some("Physics"));
        assert_eq!(
            info.varianttitles.as_ref().unwrap().get("en"),
            Some(&"Physics".to_string())
        );
    }

    #[test]
    fn deserialize_minimal_page() {
        let j = json!({
            "pageid": 1,
            "ns": 0,
            "title": "Main Page",
            "contentmodel": "wikitext",
            "pagelanguage": "en",
            "pagelanguagehtmlcode": "en",
            "pagelanguagedir": "ltr"
        });
        let info: PageInfo = serde_json::from_value(j).unwrap();
        assert_eq!(info.pageid, Some(1));
        assert_eq!(info.title, "Main Page");
        assert!(!info.missing);
        assert!(info.protection.is_empty());
        assert!(info.talkid.is_none());
        assert!(info.fullurl.is_none());
        assert!(info.displaytitle.is_none());
    }

    #[test]
    fn deserialize_missing_page_v1() {
        // formatversion=1: missing pages have "missing": ""
        let j = json!({
            "ns": 0,
            "title": "Nonexistent page",
            "missing": "",
            "contentmodel": "wikitext",
            "pagelanguage": "en",
            "pagelanguagehtmlcode": "en",
            "pagelanguagedir": "ltr"
        });
        let info: PageInfo = serde_json::from_value(j).unwrap();
        assert!(info.missing);
        assert!(info.pageid.is_none());
        assert_eq!(info.title, "Nonexistent page");
    }

    #[test]
    fn deserialize_missing_page_v2() {
        // formatversion=2: missing pages have "missing": true
        let j = json!({
            "ns": 0,
            "title": "Nonexistent page",
            "missing": true,
            "contentmodel": "wikitext",
            "pagelanguage": "en",
            "pagelanguagehtmlcode": "en",
            "pagelanguagedir": "ltr"
        });
        let info: PageInfo = serde_json::from_value(j).unwrap();
        assert!(info.missing);
    }

    #[test]
    fn deserialize_redirect_page() {
        let j = json!({
            "pageid": 42,
            "ns": 0,
            "title": "Some redirect",
            "contentmodel": "wikitext",
            "pagelanguage": "en",
            "pagelanguagehtmlcode": "en",
            "pagelanguagedir": "ltr",
            "redirect": true
        });
        let info: PageInfo = serde_json::from_value(j).unwrap();
        assert!(info.redirect);
    }

    #[test]
    fn unknown_fields_captured_in_extra() {
        let j = json!({
            "pageid": 1,
            "ns": 0,
            "title": "Test",
            "contentmodel": "wikitext",
            "pagelanguage": "en",
            "pagelanguagehtmlcode": "en",
            "pagelanguagedir": "ltr",
            "some_future_field": "hello",
            "another_field": 42
        });
        let info: PageInfo = serde_json::from_value(j).unwrap();
        assert_eq!(info.extra["some_future_field"], json!("hello"));
        assert_eq!(info.extra["another_field"], json!(42));
    }

    // --- ProtectionEntry ---

    #[test]
    fn protection_entry_deserialize() {
        let j = json!({"type": "edit", "level": "sysop", "expiry": "2025-01-01T00:00:00Z"});
        let entry: ProtectionEntry = serde_json::from_value(j).unwrap();
        assert_eq!(entry.protection_type, "edit");
        assert_eq!(entry.level, "sysop");
        assert_eq!(entry.expiry, "2025-01-01T00:00:00Z");
    }

    #[test]
    fn protection_entry_serialize_roundtrip() {
        let entry = ProtectionEntry {
            protection_type: "move".to_string(),
            level: "autoconfirmed".to_string(),
            expiry: "infinity".to_string(),
        };
        let j = serde_json::to_value(&entry).unwrap();
        assert_eq!(j["type"], "move");
        let back: ProtectionEntry = serde_json::from_value(j).unwrap();
        assert_eq!(back, entry);
    }

    // --- PageInfoList ---

    fn sample_result_v1() -> Value {
        json!({
            "query": {
                "pages": {
                    "22939": {
                        "pageid": 22939,
                        "ns": 0,
                        "title": "Physics",
                        "contentmodel": "wikitext",
                        "pagelanguage": "en",
                        "pagelanguagehtmlcode": "en",
                        "pagelanguagedir": "ltr",
                        "touched": "2024-01-01T00:00:00Z",
                        "lastrevid": 100,
                        "length": 50000
                    },
                    "736": {
                        "pageid": 736,
                        "ns": 0,
                        "title": "Albert Einstein",
                        "contentmodel": "wikitext",
                        "pagelanguage": "en",
                        "pagelanguagehtmlcode": "en",
                        "pagelanguagedir": "ltr",
                        "touched": "2024-01-02T00:00:00Z",
                        "lastrevid": 200,
                        "length": 80000
                    }
                }
            }
        })
    }

    fn sample_result_v2() -> Value {
        json!({
            "query": {
                "pages": [
                    {
                        "pageid": 22939,
                        "ns": 0,
                        "title": "Physics",
                        "contentmodel": "wikitext",
                        "pagelanguage": "en",
                        "pagelanguagehtmlcode": "en",
                        "pagelanguagedir": "ltr"
                    },
                    {
                        "pageid": 736,
                        "ns": 0,
                        "title": "Albert Einstein",
                        "contentmodel": "wikitext",
                        "pagelanguage": "en",
                        "pagelanguagehtmlcode": "en",
                        "pagelanguagedir": "ltr"
                    }
                ]
            }
        })
    }

    #[test]
    fn from_result_v1() {
        let list = PageInfoList::from_result(&sample_result_v1());
        assert_eq!(list.len(), 2);
        assert!(list.pages().iter().any(|p| p.title == "Physics"));
        assert!(list.pages().iter().any(|p| p.title == "Albert Einstein"));
    }

    #[test]
    fn from_result_v2() {
        let list = PageInfoList::from_result(&sample_result_v2());
        assert_eq!(list.len(), 2);
        assert_eq!(list.pages()[0].title, "Physics");
        assert_eq!(list.pages()[1].title, "Albert Einstein");
    }

    #[test]
    fn add_from_result_accumulates() {
        let mut list = PageInfoList::new();
        assert!(list.is_empty());
        list.add_from_result(&sample_result_v1());
        assert_eq!(list.len(), 2);
        list.add_from_result(&sample_result_v2());
        assert_eq!(list.len(), 4);
    }

    #[test]
    fn from_result_empty() {
        let list = PageInfoList::from_result(&json!({}));
        assert!(list.is_empty());

        let list = PageInfoList::from_result(&json!({"query": {"pages": {}}}));
        assert!(list.is_empty());

        let list = PageInfoList::from_result(&json!({"query": {"pages": []}}));
        assert!(list.is_empty());
    }

    #[test]
    fn from_result_with_missing_page() {
        let result = json!({
            "query": {
                "pages": {
                    "-1": {
                        "ns": 0,
                        "title": "Nonexistent",
                        "missing": "",
                        "contentmodel": "wikitext",
                        "pagelanguage": "en",
                        "pagelanguagehtmlcode": "en",
                        "pagelanguagedir": "ltr"
                    }
                }
            }
        });
        let list = PageInfoList::from_result(&result);
        assert_eq!(list.len(), 1);
        assert!(list.pages()[0].missing);
        assert_eq!(list.pages()[0].title, "Nonexistent");
    }

    #[test]
    fn into_iterator() {
        let list = PageInfoList::from_result(&sample_result_v2());
        let titles: Vec<String> = list.into_iter().map(|p| p.title).collect();
        assert_eq!(titles.len(), 2);
    }

    #[test]
    fn ref_iterator() {
        let list = PageInfoList::from_result(&sample_result_v2());
        let titles: Vec<&str> = (&list).into_iter().map(|p| p.title.as_str()).collect();
        assert_eq!(titles.len(), 2);
        // list is still usable
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn pages_mut_allows_modification() {
        let mut list = PageInfoList::from_result(&sample_result_v2());
        list.pages_mut().retain(|p| p.title == "Physics");
        assert_eq!(list.len(), 1);
    }

    // --- Integration tests (wiremock) ---

    #[tokio::test]
    async fn integration_from_result() {
        use crate::Api;
        use crate::action_api::{ActionApiQuery, ActionApiQueryCommonBuilder, ActionApiRunnable};
        use wiremock::matchers::query_param;
        use wiremock::{Mock, ResponseTemplate};
        let server = crate::test_helpers::test_helpers_mod::start_enwiki_mock().await;
        Mock::given(query_param("prop", "info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "pages": {
                        "736": {
                            "pageid": 736, "ns": 0, "title": "Albert Einstein",
                            "contentmodel": "wikitext", "pagelanguage": "en",
                            "fullurl": "https://en.wikipedia.org/wiki/Albert_Einstein",
                            "displaytitle": "Albert Einstein",
                            "protection": [{"type": "edit", "level": "autoconfirmed"}]
                        },
                        "22989": {
                            "pageid": 22989, "ns": 0, "title": "Physics",
                            "contentmodel": "wikitext", "pagelanguage": "en",
                            "fullurl": "https://en.wikipedia.org/wiki/Physics",
                            "displaytitle": "Physics",
                            "protection": []
                        }
                    }
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApiQuery::info()
            .inprop(&["protection", "url", "displaytitle"])
            .titles(&["Albert Einstein", "Physics"])
            .run(&api)
            .await
            .unwrap();
        let list = PageInfoList::from_result(&result);
        assert_eq!(list.len(), 2);
        for page in list.pages() {
            assert!(!page.title.is_empty());
            assert!(page.pageid.is_some());
            assert!(page.fullurl.is_some());
            assert!(page.displaytitle.is_some());
            assert!(!page.protection.is_empty() || page.title == "Physics");
        }
    }

    #[tokio::test]
    async fn integration_fetch_all() {
        use crate::Api;
        use crate::action_api::{ActionApiQuery, ActionApiQueryCommonBuilder};
        use wiremock::matchers::query_param;
        use wiremock::{Mock, ResponseTemplate};
        let server = crate::test_helpers::test_helpers_mod::start_enwiki_mock().await;
        Mock::given(query_param("prop", "info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "pages": {
                        "736": {
                            "pageid": 736, "ns": 0, "title": "Albert Einstein",
                            "fullurl": "https://en.wikipedia.org/wiki/Albert_Einstein",
                            "displaytitle": "Albert Einstein",
                            "protection": [{"type": "edit", "level": "autoconfirmed"}]
                        },
                        "22989": {
                            "pageid": 22989, "ns": 0, "title": "Physics",
                            "fullurl": "https://en.wikipedia.org/wiki/Physics",
                            "displaytitle": "Physics",
                            "protection": []
                        }
                    }
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let builder = ActionApiQuery::info()
            .inprop(&["protection", "url", "displaytitle"])
            .titles(&["Albert Einstein", "Physics"]);
        let list = PageInfoList::fetch_all(&builder, &api, None).await.unwrap();
        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);
        for page in list.pages() {
            assert!(!page.title.is_empty());
        }
    }

    #[test]
    fn sync_integration_fetch_all() {
        use crate::ApiSync;
        use crate::action_api::{ActionApiQuery, ActionApiQueryCommonBuilder};
        use wiremock::matchers::query_param;
        use wiremock::{Mock, ResponseTemplate};
        let rt = tokio::runtime::Runtime::new().unwrap();
        let server = rt.block_on(async {
            let server = crate::test_helpers::test_helpers_mod::start_enwiki_mock().await;
            Mock::given(query_param("prop", "info"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "batchcomplete": "",
                    "query": {
                        "pages": {
                            "736": {
                                "pageid": 736, "ns": 0, "title": "Albert Einstein",
                                "fullurl": "https://en.wikipedia.org/wiki/Albert_Einstein",
                                "displaytitle": "Albert Einstein",
                                "protection": []
                            },
                            "22989": {
                                "pageid": 22989, "ns": 0, "title": "Physics",
                                "fullurl": "https://en.wikipedia.org/wiki/Physics",
                                "displaytitle": "Physics",
                                "protection": []
                            }
                        }
                    }
                })))
                .mount(&server)
                .await;
            server
        });
        let api = ApiSync::new(&server.uri()).unwrap();
        let builder = ActionApiQuery::info()
            .inprop(&["protection", "url", "displaytitle"])
            .titles(&["Albert Einstein", "Physics"]);
        let list = PageInfoList::fetch_all_sync(&builder, &api, None).unwrap();
        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);
    }
}
