/*!
Typed representations of MediaWiki [`action=query&prop=revisions`][mw-rev] results.

[`PageRevisionEntry`] pairs a [`PageContext`] with a [`Revision`], so each
revision retains context about which page it belongs to.

[`PageRevisionList`] is the collection type that handles parsing and pagination.

[mw-rev]: https://www.mediawiki.org/wiki/API:Revisions

# Example

```rust
# tokio::runtime::Runtime::new().unwrap().block_on(async {
use mediawiki::prelude::*;
use mediawiki::page_revisions::PageRevisionList;

let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();

let result = ActionApiQuery::revisions()
    .rvprop(&["ids", "timestamp", "user", "comment", "size"])
    .rvlimit(5)
    .titles(&["Albert Einstein"])
    .run(&api)
    .await
    .unwrap();

let list = PageRevisionList::from_result(&result);
for entry in list.items() {
    println!("{}: rev {} by {}",
        entry.page.title,
        entry.revision.id(),
        entry.revision.user().unwrap_or("?"),
    );
}
# });
```
*/

#![deny(missing_docs)]

use serde_json::Value;

use crate::page_query::{PageContext, PageQueryResult, PageQueryResultList};
use crate::Revision;

/// A single revision together with the page it belongs to.
#[derive(Debug, Clone)]
pub struct PageRevisionEntry {
    /// Metadata about the page this revision belongs to.
    pub page: PageContext,
    /// The revision data.
    pub revision: Revision,
}

impl PageQueryResult for PageRevisionEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["revisions"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        Some(Self {
                            page: ctx.clone(),
                            revision: Revision::from_json(v).ok()?,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageRevisionEntry`] items parsed from API responses.
///
/// Type alias for [`PageQueryResultList<PageRevisionEntry>`].
pub type PageRevisionList = PageQueryResultList<PageRevisionEntry>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_result() -> Value {
        json!({
            "query": {
                "pages": {
                    "736": {
                        "pageid": 736,
                        "ns": 0,
                        "title": "Albert Einstein",
                        "revisions": [
                            {
                                "revid": 100,
                                "parentid": 99,
                                "user": "Alice",
                                "timestamp": "2024-01-15T10:30:00Z",
                                "size": 50000,
                                "comment": "Fixed typo"
                            },
                            {
                                "revid": 99,
                                "user": "Bob",
                                "timestamp": "2024-01-14T08:00:00Z",
                                "size": 49990,
                                "comment": "Added reference"
                            }
                        ]
                    }
                }
            }
        })
    }

    fn sample_result_v2_multi() -> Value {
        json!({
            "query": {
                "pages": [
                    {
                        "pageid": 1,
                        "ns": 0,
                        "title": "Page A",
                        "revisions": [{"revid": 10}]
                    },
                    {
                        "pageid": 2,
                        "ns": 0,
                        "title": "Page B",
                        "revisions": [{"revid": 20}, {"revid": 21}]
                    }
                ]
            }
        })
    }

    #[test]
    fn from_result_v1() {
        let list = PageRevisionList::from_result(&sample_result());
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].page.title, "Albert Einstein");
        assert_eq!(list.items()[0].revision.id(), 100);
        assert_eq!(list.items()[0].revision.user(), Some("Alice"));
        assert_eq!(list.items()[1].revision.id(), 99);
        assert_eq!(list.items()[1].revision.user(), Some("Bob"));
    }

    #[test]
    fn from_result_v2_multiple_pages() {
        let list = PageRevisionList::from_result(&sample_result_v2_multi());
        assert_eq!(list.len(), 3);
        assert_eq!(list.items()[0].page.title, "Page A");
        assert_eq!(list.items()[0].revision.id(), 10);
        assert_eq!(list.items()[1].page.title, "Page B");
        assert_eq!(list.items()[1].revision.id(), 20);
        assert_eq!(list.items()[2].revision.id(), 21);
    }

    #[test]
    fn page_with_no_revisions() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {"pageid": 1, "ns": 0, "title": "Empty"}
                }
            }
        });
        let list = PageRevisionList::from_result(&result);
        assert!(list.is_empty());
    }

    #[test]
    fn invalid_revision_skipped() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Test",
                        "revisions": [
                            {"revid": 10},
                            {"no_revid": true},
                            {"revid": 12}
                        ]
                    }
                }
            }
        });
        let list = PageRevisionList::from_result(&result);
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].revision.id(), 10);
        assert_eq!(list.items()[1].revision.id(), 12);
    }

    #[test]
    fn page_context_preserved() {
        let list = PageRevisionList::from_result(&sample_result());
        for entry in list.items() {
            assert_eq!(entry.page.pageid, Some(736));
            assert_eq!(entry.page.ns, 0);
            assert_eq!(entry.page.title, "Albert Einstein");
        }
    }

    #[tokio::test]
    async fn integration_fetch_revisions() {
        use crate::action_api::{ActionApiQuery, ActionApiQueryCommonBuilder, ActionApiRunnable};
        use crate::Api;
        use wiremock::{Mock, ResponseTemplate};
        use wiremock::matchers::query_param;
        let server = crate::test_helpers::test_helpers::start_enwiki_mock().await;
        Mock::given(query_param("prop", "revisions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "pages": {
                        "736": {
                            "pageid": 736, "ns": 0, "title": "Albert Einstein",
                            "revisions": [
                                {"revid": 1001, "parentid": 1000, "user": "Editor1", "timestamp": "2024-01-01T00:00:00Z", "size": 12345},
                                {"revid": 1000, "parentid": 999,  "user": "Editor2", "timestamp": "2023-12-01T00:00:00Z", "size": 12300},
                                {"revid": 999,  "parentid": 998,  "user": "Editor3", "timestamp": "2023-11-01T00:00:00Z", "size": 12200}
                            ]
                        }
                    }
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApiQuery::revisions()
            .rvprop(&["ids", "timestamp", "user", "size"])
            .rvlimit(3)
            .titles(&["Albert Einstein"])
            .run(&api)
            .await
            .unwrap();
        let list = PageRevisionList::from_result(&result);
        assert!(!list.is_empty());
        assert!(list.len() <= 3);
        for entry in list.items() {
            assert_eq!(entry.page.title, "Albert Einstein");
            assert!(entry.revision.id() > 0);
        }
    }
}
