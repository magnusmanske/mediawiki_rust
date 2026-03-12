/*!
Typed representations of MediaWiki [`action=query&prop=categories`][mw-cat] results.

[`PageCategoryEntry`] pairs a [`PageContext`] with a category's namespace and title,
plus optional fields from `clprop` (sortkey, timestamp, hidden).

[`PageCategoryList`] is the collection type that handles parsing and pagination.

[mw-cat]: https://www.mediawiki.org/wiki/API:Categories

# Example

```rust
# tokio::runtime::Runtime::new().unwrap().block_on(async {
use mediawiki::prelude::*;
use mediawiki::page_categories::PageCategoryList;

let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();

let result = ActionApiQuery::categories()
    .cllimit(500)
    .titles(&["Albert Einstein"])
    .run(&api)
    .await
    .unwrap();

let list = PageCategoryList::from_result(&result);
for entry in list.items() {
    println!("{} is in category: {}", entry.page.title, entry.title);
}
# });
```
*/

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::page_query::{PageContext, PageQueryResult, PageQueryResultList};

/// A single category entry from a `prop=categories` response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageCategoryEntry {
    /// Metadata about the page this category belongs to.
    #[serde(skip)]
    pub page: PageContext,
    /// Namespace ID of the category (always 14).
    #[serde(default)]
    pub ns: i64,
    /// Full title of the category (e.g. `"Category:Physics"`).
    #[serde(default)]
    pub title: String,
    /// Sort key used within the category. Requires `clprop=sortkey`.
    pub sortkey: Option<String>,
    /// Sort key prefix. Requires `clprop=sortkey`.
    pub sortkeyprefix: Option<String>,
    /// Timestamp of when the page was added to the category. Requires `clprop=timestamp`.
    pub timestamp: Option<String>,
    /// Whether the category is hidden. Requires `clprop=hidden`.
    /// Present as empty string in formatversion=1, boolean in formatversion=2.
    pub hidden: Option<Value>,
}

impl PageCategoryEntry {
    /// Returns `true` if this is a hidden category.
    pub fn is_hidden(&self) -> bool {
        match &self.hidden {
            Some(Value::Bool(b)) => *b,
            Some(Value::String(_)) => true, // formatversion=1: "" means true
            _ => false,
        }
    }
}

impl PageQueryResult for PageCategoryEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["categories"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let mut entry: PageCategoryEntry = serde_json::from_value(v.clone()).ok()?;
                        entry.page = ctx.clone();
                        Some(entry)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageCategoryEntry`] items parsed from API responses.
///
/// Type alias for [`PageQueryResultList<PageCategoryEntry>`].
pub type PageCategoryList = PageQueryResultList<PageCategoryEntry>;

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
                        "categories": [
                            {"ns": 14, "title": "Category:1879 births"},
                            {"ns": 14, "title": "Category:German physicists"},
                            {"ns": 14, "title": "Category:Nobel laureates in Physics"}
                        ]
                    }
                }
            }
        })
    }

    #[test]
    fn from_result() {
        let list = PageCategoryList::from_result(&sample_result());
        assert_eq!(list.len(), 3);
        assert_eq!(list.items()[0].page.title, "Albert Einstein");
        assert_eq!(list.items()[0].ns, 14);
        assert_eq!(list.items()[0].title, "Category:1879 births");
    }

    #[test]
    fn from_result_v2() {
        let result = json!({
            "query": {
                "pages": [
                    {
                        "pageid": 1, "ns": 0, "title": "Page A",
                        "categories": [{"ns": 14, "title": "Category:Foo"}]
                    },
                    {
                        "pageid": 2, "ns": 0, "title": "Page B",
                        "categories": [
                            {"ns": 14, "title": "Category:Bar"},
                            {"ns": 14, "title": "Category:Baz"}
                        ]
                    }
                ]
            }
        });
        let list = PageCategoryList::from_result(&result);
        assert_eq!(list.len(), 3);
        assert_eq!(list.items()[0].page.title, "Page A");
        assert_eq!(list.items()[1].page.title, "Page B");
    }

    #[test]
    fn page_with_no_categories() {
        let result = json!({
            "query": {"pages": {"1": {"pageid": 1, "ns": 0, "title": "Test"}}}
        });
        let list = PageCategoryList::from_result(&result);
        assert!(list.is_empty());
    }

    #[test]
    fn with_sortkey_and_timestamp() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Test",
                        "categories": [{
                            "ns": 14,
                            "title": "Category:Foo",
                            "sortkey": "TEST",
                            "sortkeyprefix": "Test",
                            "timestamp": "2024-01-15T10:30:00Z"
                        }]
                    }
                }
            }
        });
        let list = PageCategoryList::from_result(&result);
        assert_eq!(list.len(), 1);
        assert_eq!(list.items()[0].sortkey.as_deref(), Some("TEST"));
        assert_eq!(list.items()[0].sortkeyprefix.as_deref(), Some("Test"));
        assert_eq!(
            list.items()[0].timestamp.as_deref(),
            Some("2024-01-15T10:30:00Z")
        );
    }

    #[test]
    fn hidden_category_v1() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Test",
                        "categories": [{"ns": 14, "title": "Category:Hidden", "hidden": ""}]
                    }
                }
            }
        });
        let list = PageCategoryList::from_result(&result);
        assert!(list.items()[0].is_hidden());
    }

    #[test]
    fn hidden_category_v2() {
        let result = json!({
            "query": {
                "pages": [{
                    "pageid": 1, "ns": 0, "title": "Test",
                    "categories": [{"ns": 14, "title": "Category:Hidden", "hidden": true}]
                }]
            }
        });
        let list = PageCategoryList::from_result(&result);
        assert!(list.items()[0].is_hidden());
    }

    #[test]
    fn not_hidden() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Test",
                        "categories": [{"ns": 14, "title": "Category:Visible"}]
                    }
                }
            }
        });
        let list = PageCategoryList::from_result(&result);
        assert!(!list.items()[0].is_hidden());
    }

    #[tokio::test]
    async fn integration_fetch_categories() {
        use crate::action_api::{ActionApiQuery, ActionApiQueryCommonBuilder, ActionApiRunnable};
        use crate::Api;
        use wiremock::{Mock, ResponseTemplate};
        use wiremock::matchers::query_param;
        let server = crate::test_helpers::test_helpers::start_enwiki_mock().await;
        Mock::given(query_param("prop", "categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {
                    "pages": {
                        "736": {
                            "pageid": 736, "ns": 0, "title": "Albert Einstein",
                            "categories": [
                                {"ns": 14, "title": "Category:1879 births"},
                                {"ns": 14, "title": "Category:German physicists"},
                                {"ns": 14, "title": "Category:Nobel laureates in Physics"}
                            ]
                        }
                    }
                }
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let result = ActionApiQuery::categories()
            .cllimit(10)
            .titles(&["Albert Einstein"])
            .run(&api)
            .await
            .unwrap();
        let list = PageCategoryList::from_result(&result);
        assert!(!list.is_empty());
        for entry in list.items() {
            assert_eq!(entry.page.title, "Albert Einstein");
            assert_eq!(entry.ns, 14);
            assert!(entry.title.starts_with("Category:"));
        }
    }
}
