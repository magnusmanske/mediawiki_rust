/*!
Typed representations of MediaWiki [`action=query&prop=links`][mw-links] results.

[`PageLinkEntry`] pairs a [`PageContext`] with a linked page's namespace and title.

[`PageLinkList`] is the collection type that handles parsing and pagination.

[mw-links]: https://www.mediawiki.org/wiki/API:Links

# Example

```rust
# tokio::runtime::Runtime::new().unwrap().block_on(async {
use mediawiki::prelude::*;
use mediawiki::page_links::PageLinkList;

let api = Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();

let result = ActionApiQuery::links()
    .pllimit(20)
    .titles(&["Albert Einstein"])
    .run(&api)
    .await
    .unwrap();

let list = PageLinkList::from_result(&result);
for entry in list.items() {
    println!("{} links to: {}", entry.page.title, entry.title);
}
# });
```
*/

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::page_query::{PageContext, PageQueryResult, PageQueryResultList};

/// A single link entry from a `prop=links` response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageLinkEntry {
    /// Metadata about the page containing this link.
    #[serde(skip)]
    pub page: PageContext,
    /// Namespace ID of the linked page.
    #[serde(default)]
    pub ns: i64,
    /// Full title of the linked page.
    #[serde(default)]
    pub title: String,
}

impl PageQueryResult for PageLinkEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["links"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let mut entry: PageLinkEntry = serde_json::from_value(v.clone()).ok()?;
                        entry.page = ctx.clone();
                        Some(entry)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageLinkEntry`] items parsed from API responses.
///
/// Type alias for [`PageQueryResultList<PageLinkEntry>`].
pub type PageLinkList = PageQueryResultList<PageLinkEntry>;

/// A single template entry from a `prop=templates` response.
///
/// Structurally identical to [`PageLinkEntry`] but extracted from the
/// `"templates"` sub-array.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageTemplateEntry {
    /// Metadata about the page containing this template.
    #[serde(skip)]
    pub page: PageContext,
    /// Namespace ID of the template (usually 10).
    #[serde(default)]
    pub ns: i64,
    /// Full title of the template.
    #[serde(default)]
    pub title: String,
}

impl PageQueryResult for PageTemplateEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["templates"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let mut entry: PageTemplateEntry =
                            serde_json::from_value(v.clone()).ok()?;
                        entry.page = ctx.clone();
                        Some(entry)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageTemplateEntry`] items parsed from API responses.
///
/// Type alias for [`PageQueryResultList<PageTemplateEntry>`].
pub type PageTemplateList = PageQueryResultList<PageTemplateEntry>;

/// A single image entry from a `prop=images` response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageImageEntry {
    /// Metadata about the page containing this image.
    #[serde(skip)]
    pub page: PageContext,
    /// Namespace ID of the image (usually 6).
    #[serde(default)]
    pub ns: i64,
    /// Full title of the image file.
    #[serde(default)]
    pub title: String,
}

impl PageQueryResult for PageImageEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["images"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let mut entry: PageImageEntry = serde_json::from_value(v.clone()).ok()?;
                        entry.page = ctx.clone();
                        Some(entry)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageImageEntry`] items parsed from API responses.
///
/// Type alias for [`PageQueryResultList<PageImageEntry>`].
pub type PageImageList = PageQueryResultList<PageImageEntry>;

/// A single external link from a `prop=extlinks` response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageExtLinkEntry {
    /// Metadata about the page containing this external link.
    pub page: PageContext,
    /// The external URL.
    pub url: String,
}

impl PageQueryResult for PageExtLinkEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["extlinks"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        // formatversion=1: {"*": "url"}, formatversion=2: {"url": "url"}
                        let url = v["url"]
                            .as_str()
                            .or_else(|| v["*"].as_str())?
                            .to_string();
                        Some(Self {
                            page: ctx.clone(),
                            url,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageExtLinkEntry`] items parsed from API responses.
///
/// Type alias for [`PageQueryResultList<PageExtLinkEntry>`].
pub type PageExtLinkList = PageQueryResultList<PageExtLinkEntry>;

/// A single "links here" entry from a `prop=linkshere` response.
///
/// Represents a page that links to the queried page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageLinksHereEntry {
    /// Metadata about the queried page (the target of the links).
    #[serde(skip)]
    pub page: PageContext,
    /// Page ID of the linking page.
    pub pageid: Option<u64>,
    /// Namespace ID of the linking page.
    #[serde(default)]
    pub ns: i64,
    /// Title of the linking page.
    #[serde(default)]
    pub title: String,
    /// Whether the linking page is a redirect.
    pub redirect: Option<Value>,
}

impl PageLinksHereEntry {
    /// Returns `true` if the linking page is a redirect.
    pub fn is_redirect(&self) -> bool {
        match &self.redirect {
            Some(Value::Bool(b)) => *b,
            Some(Value::String(_)) => true,
            _ => false,
        }
    }
}

impl PageQueryResult for PageLinksHereEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["linkshere"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let mut entry: PageLinksHereEntry =
                            serde_json::from_value(v.clone()).ok()?;
                        entry.page = ctx.clone();
                        Some(entry)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageLinksHereEntry`] items parsed from API responses.
///
/// Type alias for [`PageQueryResultList<PageLinksHereEntry>`].
pub type PageLinksHereList = PageQueryResultList<PageLinksHereEntry>;

/// A single interwiki link from a `prop=iwlinks` response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageIwLinkEntry {
    /// Metadata about the page containing this interwiki link.
    #[serde(skip)]
    pub page: PageContext,
    /// Interwiki prefix (e.g. `"wikt"`, `"commons"`).
    #[serde(default)]
    pub prefix: String,
    /// Title on the target wiki.
    #[serde(rename = "*", default)]
    pub title: String,
    /// URL of the interwiki link. Requires `iwprop=url`.
    pub url: Option<String>,
}

impl PageQueryResult for PageIwLinkEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["iwlinks"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let mut entry: PageIwLinkEntry = serde_json::from_value(v.clone()).ok()?;
                        entry.page = ctx.clone();
                        Some(entry)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageIwLinkEntry`] items parsed from API responses.
///
/// Type alias for [`PageQueryResultList<PageIwLinkEntry>`].
pub type PageIwLinkList = PageQueryResultList<PageIwLinkEntry>;

/// A single language link from a `prop=langlinks` response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageLangLinkEntry {
    /// Metadata about the page containing this language link.
    #[serde(skip)]
    pub page: PageContext,
    /// Language code (e.g. `"de"`, `"fr"`).
    #[serde(default)]
    pub lang: String,
    /// Title on the target language wiki.
    #[serde(rename = "*", default)]
    pub title: String,
    /// URL of the language link. Requires `llprop=url`.
    pub url: Option<String>,
    /// Autonym (native name of the language). Requires `llprop=autonym`.
    pub autonym: Option<String>,
    /// Language name in the requested language. Requires `llprop=langname`.
    pub langname: Option<String>,
}

impl PageQueryResult for PageLangLinkEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["langlinks"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let mut entry: PageLangLinkEntry =
                            serde_json::from_value(v.clone()).ok()?;
                        entry.page = ctx.clone();
                        Some(entry)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageLangLinkEntry`] items parsed from API responses.
///
/// Type alias for [`PageQueryResultList<PageLangLinkEntry>`].
pub type PageLangLinkList = PageQueryResultList<PageLangLinkEntry>;

/// A single contributor from a `prop=contributors` response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageContributorEntry {
    /// Metadata about the page.
    pub page: PageContext,
    /// User ID of the contributor.
    pub userid: u64,
    /// Username of the contributor.
    pub name: String,
}

impl PageQueryResult for PageContributorEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["contributors"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        Some(Self {
                            page: ctx.clone(),
                            userid: v["userid"].as_u64()?,
                            name: v["name"].as_str()?.to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageContributorEntry`] items parsed from API responses.
///
/// Type alias for [`PageQueryResultList<PageContributorEntry>`].
pub type PageContributorList = PageQueryResultList<PageContributorEntry>;

/// A single redirect entry from a `prop=redirects` response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageRedirectEntry {
    /// Metadata about the target page.
    #[serde(skip)]
    pub page: PageContext,
    /// Page ID of the redirect page.
    pub pageid: Option<u64>,
    /// Namespace ID of the redirect page.
    #[serde(default)]
    pub ns: i64,
    /// Title of the redirect page.
    #[serde(default)]
    pub title: String,
    /// Fragment (section anchor) if any.
    pub fragment: Option<String>,
}

impl PageQueryResult for PageRedirectEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["redirects"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let mut entry: PageRedirectEntry =
                            serde_json::from_value(v.clone()).ok()?;
                        entry.page = ctx.clone();
                        Some(entry)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageRedirectEntry`] items parsed from API responses.
pub type PageRedirectList = PageQueryResultList<PageRedirectEntry>;

/// A single file usage entry from a `prop=fileusage` response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageFileUsageEntry {
    /// Metadata about the file page.
    #[serde(skip)]
    pub page: PageContext,
    /// Page ID of the page using the file.
    pub pageid: Option<u64>,
    /// Namespace ID of the page using the file.
    #[serde(default)]
    pub ns: i64,
    /// Title of the page using the file.
    #[serde(default)]
    pub title: String,
    /// Whether this is a redirect. Requires `fuprop=redirect`.
    pub redirect: Option<Value>,
}

impl PageQueryResult for PageFileUsageEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["fileusage"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let mut entry: PageFileUsageEntry =
                            serde_json::from_value(v.clone()).ok()?;
                        entry.page = ctx.clone();
                        Some(entry)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageFileUsageEntry`] items parsed from API responses.
pub type PageFileUsageList = PageQueryResultList<PageFileUsageEntry>;

/// A single "transcluded in" entry from a `prop=transcludedin` response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageTranscludedInEntry {
    /// Metadata about the transcluded page (template).
    #[serde(skip)]
    pub page: PageContext,
    /// Page ID of the page that transcludes.
    pub pageid: Option<u64>,
    /// Namespace ID of the page that transcludes.
    #[serde(default)]
    pub ns: i64,
    /// Title of the page that transcludes.
    #[serde(default)]
    pub title: String,
    /// Whether the transcluding page is a redirect.
    pub redirect: Option<Value>,
}

impl PageQueryResult for PageTranscludedInEntry {
    fn from_page_value(page: &Value) -> Vec<Self> {
        let ctx = PageContext::from_value(page);
        page["transcludedin"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let mut entry: PageTranscludedInEntry =
                            serde_json::from_value(v.clone()).ok()?;
                        entry.page = ctx.clone();
                        Some(entry)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// A collection of [`PageTranscludedInEntry`] items parsed from API responses.
pub type PageTranscludedInList = PageQueryResultList<PageTranscludedInEntry>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // --- PageLinkEntry ---

    #[test]
    fn links_from_result() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Test",
                        "links": [
                            {"ns": 0, "title": "Physics"},
                            {"ns": 0, "title": "Mathematics"},
                            {"ns": 14, "title": "Category:Science"}
                        ]
                    }
                }
            }
        });
        let list = PageLinkList::from_result(&result);
        assert_eq!(list.len(), 3);
        assert_eq!(list.items()[0].page.title, "Test");
        assert_eq!(list.items()[0].title, "Physics");
        assert_eq!(list.items()[2].ns, 14);
    }

    #[test]
    fn links_empty_page() {
        let result = json!({
            "query": {"pages": {"1": {"pageid": 1, "ns": 0, "title": "Empty"}}}
        });
        let list = PageLinkList::from_result(&result);
        assert!(list.is_empty());
    }

    // --- PageTemplateEntry ---

    #[test]
    fn templates_from_result() {
        let result = json!({
            "query": {
                "pages": [{
                    "pageid": 1, "ns": 0, "title": "Test",
                    "templates": [
                        {"ns": 10, "title": "Template:Cite web"},
                        {"ns": 10, "title": "Template:Infobox"}
                    ]
                }]
            }
        });
        let list = PageTemplateList::from_result(&result);
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].title, "Template:Cite web");
        assert_eq!(list.items()[0].ns, 10);
    }

    // --- PageImageEntry ---

    #[test]
    fn images_from_result() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Test",
                        "images": [
                            {"ns": 6, "title": "File:Example.jpg"},
                            {"ns": 6, "title": "File:Photo.png"}
                        ]
                    }
                }
            }
        });
        let list = PageImageList::from_result(&result);
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].ns, 6);
        assert_eq!(list.items()[0].title, "File:Example.jpg");
    }

    // --- PageExtLinkEntry ---

    #[test]
    fn extlinks_from_result_v1() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Test",
                        "extlinks": [
                            {"*": "https://example.com"},
                            {"*": "https://example.org"}
                        ]
                    }
                }
            }
        });
        let list = PageExtLinkList::from_result(&result);
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].url, "https://example.com");
        assert_eq!(list.items()[0].page.title, "Test");
    }

    #[test]
    fn extlinks_from_result_v2() {
        let result = json!({
            "query": {
                "pages": [{
                    "pageid": 1, "ns": 0, "title": "Test",
                    "extlinks": [
                        {"url": "https://example.com"}
                    ]
                }]
            }
        });
        let list = PageExtLinkList::from_result(&result);
        assert_eq!(list.len(), 1);
        assert_eq!(list.items()[0].url, "https://example.com");
    }

    // --- PageLinksHereEntry ---

    #[test]
    fn linkshere_from_result() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Target",
                        "linkshere": [
                            {"pageid": 10, "ns": 0, "title": "Linking page"},
                            {"pageid": 11, "ns": 0, "title": "Redirect", "redirect": ""}
                        ]
                    }
                }
            }
        });
        let list = PageLinksHereList::from_result(&result);
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].page.title, "Target");
        assert_eq!(list.items()[0].title, "Linking page");
        assert!(!list.items()[0].is_redirect());
        assert!(list.items()[1].is_redirect());
    }

    // --- PageIwLinkEntry ---

    #[test]
    fn iwlinks_from_result() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Test",
                        "iwlinks": [
                            {"prefix": "wikt", "*": "physics"}
                        ]
                    }
                }
            }
        });
        let list = PageIwLinkList::from_result(&result);
        assert_eq!(list.len(), 1);
        assert_eq!(list.items()[0].prefix, "wikt");
        assert_eq!(list.items()[0].title, "physics");
    }

    // --- PageLangLinkEntry ---

    #[test]
    fn langlinks_from_result() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Physics",
                        "langlinks": [
                            {"lang": "de", "*": "Physik"},
                            {"lang": "fr", "*": "Physique", "url": "https://fr.wikipedia.org/wiki/Physique"}
                        ]
                    }
                }
            }
        });
        let list = PageLangLinkList::from_result(&result);
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].lang, "de");
        assert_eq!(list.items()[0].title, "Physik");
        assert!(list.items()[0].url.is_none());
        assert_eq!(list.items()[1].lang, "fr");
        assert!(list.items()[1].url.is_some());
    }

    // --- PageContributorEntry ---

    #[test]
    fn contributors_from_result() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Test",
                        "contributors": [
                            {"userid": 100, "name": "Alice"},
                            {"userid": 200, "name": "Bob"}
                        ]
                    }
                }
            }
        });
        let list = PageContributorList::from_result(&result);
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].userid, 100);
        assert_eq!(list.items()[0].name, "Alice");
        assert_eq!(list.items()[1].name, "Bob");
    }

    // --- PageRedirectEntry ---

    #[test]
    fn redirects_from_result() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 0, "title": "Physics",
                        "redirects": [
                            {"pageid": 50, "ns": 0, "title": "Fysics"},
                            {"pageid": 51, "ns": 0, "title": "Physics (science)", "fragment": "Overview"}
                        ]
                    }
                }
            }
        });
        let list = PageRedirectList::from_result(&result);
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].title, "Fysics");
        assert!(list.items()[0].fragment.is_none());
        assert_eq!(list.items()[1].fragment.as_deref(), Some("Overview"));
    }

    // --- PageFileUsageEntry ---

    #[test]
    fn fileusage_from_result() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 6, "title": "File:Example.jpg",
                        "fileusage": [
                            {"pageid": 10, "ns": 0, "title": "Albert Einstein"},
                            {"pageid": 11, "ns": 0, "title": "Physics"}
                        ]
                    }
                }
            }
        });
        let list = PageFileUsageList::from_result(&result);
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].page.title, "File:Example.jpg");
        assert_eq!(list.items()[0].title, "Albert Einstein");
    }

    // --- PageTranscludedInEntry ---

    #[test]
    fn transcludedin_from_result() {
        let result = json!({
            "query": {
                "pages": {
                    "1": {
                        "pageid": 1, "ns": 10, "title": "Template:Infobox",
                        "transcludedin": [
                            {"pageid": 100, "ns": 0, "title": "Physics"},
                            {"pageid": 101, "ns": 0, "title": "Chemistry"}
                        ]
                    }
                }
            }
        });
        let list = PageTranscludedInList::from_result(&result);
        assert_eq!(list.len(), 2);
        assert_eq!(list.items()[0].page.title, "Template:Infobox");
        assert_eq!(list.items()[0].title, "Physics");
    }

    // --- Multi-page ---

    #[test]
    fn links_multiple_pages() {
        let result = json!({
            "query": {
                "pages": [
                    {
                        "pageid": 1, "ns": 0, "title": "Page A",
                        "links": [{"ns": 0, "title": "X"}]
                    },
                    {
                        "pageid": 2, "ns": 0, "title": "Page B",
                        "links": [{"ns": 0, "title": "Y"}, {"ns": 0, "title": "Z"}]
                    }
                ]
            }
        });
        let list = PageLinkList::from_result(&result);
        assert_eq!(list.len(), 3);
        assert_eq!(list.items()[0].page.title, "Page A");
        assert_eq!(list.items()[1].page.title, "Page B");
    }
}
