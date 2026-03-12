/*!
Generic infrastructure for parsing paginated MediaWiki `action=query` results
into typed Rust collections.

[`PageQueryResult`] is the trait that each typed result implements. It defines
how to extract items from a single page JSON object in `result["query"]["pages"]`.

[`PageQueryResultList`] is the generic collection that drives pagination and
provides the shared `from_result`, `add_from_result`, `fetch_all`, and
`fetch_all_sync` methods.

# Two extraction patterns

**Page-level data** (`prop=info`, `prop=categoryinfo`): the page object itself
is the item. Implementations return a `Vec` with one element.

**Sub-array data** (`prop=revisions`, `prop=categories`, `prop=links`, …): each
page contains a named sub-array. Implementations iterate the sub-array and
return one item per entry.
*/

#![deny(missing_docs)]

use serde_json::Value;

use crate::action_api::{ActionApiContinuable, ActionApiRunnable};
use crate::{Api, ApiSync, MediaWikiError};

/// Minimal page metadata extracted from the enclosing page object.
///
/// Included in sub-array item types so each item retains context about which
/// page it belongs to.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PageContext {
    /// Page ID. `None` for missing pages.
    pub pageid: Option<u64>,
    /// Namespace ID.
    pub ns: i64,
    /// Full page title including namespace prefix.
    pub title: String,
}

impl PageContext {
    /// Extracts a `PageContext` from a page JSON object.
    pub fn from_value(page: &Value) -> Self {
        Self {
            pageid: page["pageid"].as_u64(),
            ns: page["ns"].as_i64().unwrap_or(0),
            title: page["title"].as_str().unwrap_or("").to_string(),
        }
    }
}

/// Trait for types that can be extracted from a page object in a
/// `query` API response.
///
/// Implement this for each `prop=` result type. The generic
/// [`PageQueryResultList<T>`] then handles pagination and collection
/// automatically.
pub trait PageQueryResult: Sized {
    /// Extract items from a single page JSON object.
    ///
    /// - For page-level data (like `prop=info`), return a `Vec` with one element.
    /// - For sub-array data (like `prop=revisions`), return all items from the sub-array.
    /// - Return an empty `Vec` if the page cannot be parsed.
    fn from_page_value(page: &Value) -> Vec<Self>;
}

/// Iterates over page values in a `result["query"]["pages"]` node,
/// handling both `formatversion=1` (object) and `formatversion=2` (array).
fn iter_pages(pages: &Value) -> Vec<&Value> {
    if let Some(obj) = pages.as_object() {
        obj.values().collect()
    } else if let Some(arr) = pages.as_array() {
        arr.iter().collect()
    } else {
        Vec::new()
    }
}

/// A generic collection of items parsed from one or more paginated API responses.
///
/// `T` must implement [`PageQueryResult`] to define how items are extracted
/// from each page object. This struct provides the shared pagination logic
/// used by all typed result lists.
#[derive(Debug, Clone)]
pub struct PageQueryResultList<T> {
    items: Vec<T>,
}

impl<T> Default for PageQueryResultList<T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T: PageQueryResult> PageQueryResultList<T> {
    /// Creates an empty list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a list from a single API response.
    ///
    /// Extracts all items from `result["query"]["pages"]`.
    pub fn from_result(result: &Value) -> Self {
        let mut list = Self::new();
        list.add_from_result(result);
        list
    }

    /// Appends items from an API response to this list.
    ///
    /// Handles both `formatversion=1` (object) and `formatversion=2` (array).
    pub fn add_from_result(&mut self, result: &Value) {
        let pages = &result["query"]["pages"];
        for page_value in iter_pages(pages) {
            self.items.extend(T::from_page_value(page_value));
        }
    }

    /// Returns a slice of all collected items.
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Returns a mutable reference to the inner `Vec<T>`.
    pub fn items_mut(&mut self) -> &mut Vec<T> {
        &mut self.items
    }

    /// Returns the number of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` if this list contains no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Drives pagination to completion, collecting all items from a
    /// continuable query builder.
    ///
    /// - `builder` — a `Runnable` + `ActionApiContinuable` query builder.
    /// - `api` — the async API handle.
    /// - `max` — optional maximum number of items to collect. `None` = unlimited.
    pub async fn fetch_all<B>(
        builder: &B,
        api: &Api,
        max: Option<usize>,
    ) -> Result<Self, MediaWikiError>
    where
        B: ActionApiRunnable + ActionApiContinuable + Clone + Sync,
    {
        let mut list = Self::new();
        let mut builder = builder.clone();
        loop {
            let result = builder.run(api).await?;
            list.add_from_result(&result);
            if let Some(max) = max {
                if list.len() >= max {
                    list.items.truncate(max);
                    break;
                }
            }
            if !builder.has_more(&result) {
                break;
            }
            builder = builder.continue_from(&result);
        }
        Ok(list)
    }

    /// Synchronous version of [`fetch_all`](Self::fetch_all).
    pub fn fetch_all_sync<B>(
        builder: &B,
        api: &ApiSync,
        max: Option<usize>,
    ) -> Result<Self, MediaWikiError>
    where
        B: ActionApiRunnable + ActionApiContinuable + Clone + Sync,
    {
        let mut list = Self::new();
        let mut builder = builder.clone();
        loop {
            let result = builder.run_sync(api)?;
            list.add_from_result(&result);
            if let Some(max) = max {
                if list.len() >= max {
                    list.items.truncate(max);
                    break;
                }
            }
            if !builder.has_more(&result) {
                break;
            }
            builder = builder.continue_from(&result);
        }
        Ok(list)
    }
}

impl<T> IntoIterator for PageQueryResultList<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a PageQueryResultList<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Simple test type: extracts page title as a String
    struct TitleOnly(String);

    impl PageQueryResult for TitleOnly {
        fn from_page_value(page: &Value) -> Vec<Self> {
            page["title"]
                .as_str()
                .map(|s| vec![TitleOnly(s.to_string())])
                .unwrap_or_default()
        }
    }

    // Sub-array test type: extracts items from "links" sub-array
    struct LinkTitle(String);

    impl PageQueryResult for LinkTitle {
        fn from_page_value(page: &Value) -> Vec<Self> {
            page["links"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v["title"].as_str().map(|s| LinkTitle(s.to_string())))
                        .collect()
                })
                .unwrap_or_default()
        }
    }

    #[test]
    fn page_context_from_value() {
        let v = json!({"pageid": 42, "ns": 1, "title": "Talk:Test"});
        let ctx = PageContext::from_value(&v);
        assert_eq!(ctx.pageid, Some(42));
        assert_eq!(ctx.ns, 1);
        assert_eq!(ctx.title, "Talk:Test");
    }

    #[test]
    fn page_context_missing_page() {
        let v = json!({"ns": 0, "title": "Missing", "missing": ""});
        let ctx = PageContext::from_value(&v);
        assert_eq!(ctx.pageid, None);
    }

    #[test]
    fn page_level_from_result_v1() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {"pageid": 1, "ns": 0, "title": "Alpha"},
                    "2": {"pageid": 2, "ns": 0, "title": "Beta"}
                }
            }
        });
        let list = PageQueryResultList::<TitleOnly>::from_result(&result);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn page_level_from_result_v2() {
        let result = json!({
            "query": {
                "pages": [
                    {"pageid": 1, "ns": 0, "title": "Alpha"},
                    {"pageid": 2, "ns": 0, "title": "Beta"}
                ]
            }
        });
        let list = PageQueryResultList::<TitleOnly>::from_result(&result);
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].0, "Alpha");
        assert_eq!(list.items()[1].0, "Beta");
    }

    #[test]
    fn sub_array_from_result() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Test",
                        "links": [
                            {"ns": 0, "title": "Link1"},
                            {"ns": 0, "title": "Link2"}
                        ]
                    }
                }
            }
        });
        let list = PageQueryResultList::<LinkTitle>::from_result(&result);
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].0, "Link1");
        assert_eq!(list.items()[1].0, "Link2");
    }

    #[test]
    fn add_from_result_accumulates() {
        let r1 = json!({"query": {"pages": {"1": {"title": "A"}}}});
        let r2 = json!({"query": {"pages": [{"title": "B"}, {"title": "C"}]}});
        let mut list = PageQueryResultList::<TitleOnly>::new();
        assert!(list.is_empty());
        list.add_from_result(&r1);
        assert_eq!(list.len(), 1);
        list.add_from_result(&r2);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn empty_result() {
        let list = PageQueryResultList::<TitleOnly>::from_result(&json!({}));
        assert!(list.is_empty());
    }

    #[test]
    fn into_iterator() {
        let result = json!({"query": {"pages": [{"title": "A"}, {"title": "B"}]}});
        let list = PageQueryResultList::<TitleOnly>::from_result(&result);
        let titles: Vec<String> = list.into_iter().map(|t| t.0).collect();
        assert_eq!(titles, vec!["A", "B"]);
    }

    #[test]
    fn ref_iterator() {
        let result = json!({"query": {"pages": [{"title": "A"}]}});
        let list = PageQueryResultList::<TitleOnly>::from_result(&result);
        let titles: Vec<&str> = (&list).into_iter().map(|t| t.0.as_str()).collect();
        assert_eq!(titles, vec!["A"]);
        assert_eq!(list.len(), 1); // still usable
    }

    #[test]
    fn items_mut_allows_modification() {
        let result = json!({"query": {"pages": [{"title": "A"}, {"title": "B"}]}});
        let mut list = PageQueryResultList::<TitleOnly>::from_result(&result);
        list.items_mut().retain(|t| t.0 == "A");
        assert_eq!(list.len(), 1);
    }
}
