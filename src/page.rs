/*!
The `Page` class deals with operations done on pages, like editing.
*/

#![deny(missing_docs)]

use crate::Revision;
use crate::api::Api;
use crate::media_wiki_error::MediaWikiError;
use crate::title::Title;
use serde_json::Value;
use std::collections::HashMap;

/// Represents a page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    title: Title,
    page_id: Option<usize>,
    revision: Option<Revision>,
}

impl Page {
    /// Creates a new `Page` from a `Title`.
    pub fn new(title: Title) -> Self {
        Page {
            title,
            page_id: None,
            revision: None,
        }
    }

    /// Accesses the `Title` of this `Page`.
    pub fn title(&self) -> &Title {
        &self.title
    }

    /// Fetches the current text of this `Page`. If there is one slot in
    /// the current revision, it is fetched; if there are multiple slots,
    /// the "main" slot is fetched, or an error is returned if there is
    /// no "main" slot.
    ///
    /// The `revision` field of this `Page` is set to the fetched revision.
    ///
    /// # Errors
    /// If the page is missing, will return a `MediaWikiError::Missing`.
    ///
    /// [`Api::get_query_api_json`]: ../api/struct.Api.html#method.get_query_api_json
    pub async fn text(&mut self, api: &Api) -> Result<&str, MediaWikiError> {
        let title = self
            .title
            .full_with_underscores(api)
            .ok_or_else(|| MediaWikiError::BadTitle(self.title.clone()))?;
        let params = [
            ("action", "query"),
            ("prop", "revisions"),
            ("titles", &title),
            ("rvslots", "*"),
            ("rvprop", crate::revision::RVPROP),
            ("formatversion", "2"),
        ]
        .iter()
        .map(|&(k, v)| (k.to_string(), v.to_string()))
        .collect();
        let result = api.get_query_api_json(&params).await?;
        let page = &result["query"]["pages"][0];

        if !page.is_object() || page["missing"].as_bool() == Some(true) {
            return Err(MediaWikiError::Missing(self.title.clone()));
        }
        self.page_id = match page["pageid"].as_u64().map(|x| x as usize) {
            Some(x) => Some(x),
            None => return Err(MediaWikiError::BadResponse(result)),
        };
        self.revision = Some(Revision::from_json(&page["revisions"][0])?);
        let wikitext = self.revision.as_ref().unwrap().wikitext();
        let wikitext = match wikitext {
            Some(x) => x,
            None => return Err(MediaWikiError::BadResponse(result)),
        };
        Ok(wikitext)
    }

    /// Replaces the contents of this `Page` with the given text, using the given
    /// edit summary.
    ///
    /// # Errors
    /// May return a `MediaWikiError` if the edit fails or any error from [`Api::post_query_api_json`].
    ///
    /// [`Api::post_query_api_json`]: ../api/struct.Api.html#method.post_query_api_json
    pub async fn edit_text(
        &self,
        api: &mut Api,
        text: impl Into<String>,
        summary: impl Into<String>,
    ) -> Result<(), MediaWikiError> {
        let title = self
            .title
            .full_pretty(api)
            .ok_or_else(|| MediaWikiError::BadTitle(self.title.clone()))?;
        let bot = if api.user().is_bot() { "true" } else { "false" };
        let text = text.into();
        let summary = summary.into();
        let token = api.get_edit_token().await?;
        let mut params: HashMap<String, String> = [
            ("action", "edit"),
            ("title", title.as_str()),
            ("text", text.as_str()),
            ("summary", summary.as_str()),
            ("bot", bot),
            ("formatversion", "2"),
            ("token", token.as_str()),
        ]
        .iter()
        .map(|&(k, v)| (k.to_string(), v.to_string()))
        .collect();

        // Set the base revision ID if available, to avoid edit conflicts
        if let Some(baserevid) = self.revision.as_ref().map(|r| r.id()) {
            params.insert("baserevid".to_string(), baserevid.to_string());
        }

        if !api.user().user_name().is_empty() {
            params.insert("assert".to_string(), "user".to_string());
        }

        let result = api.post_query_api_json(&params).await?;
        match result["edit"]["result"].as_str() {
            Some("Success") => Ok(()),
            _ => Err(MediaWikiError::EditError(result)),
        }
    }

    /// Performs an "action=query" API action and returns the result.
    async fn action_query(
        &self,
        api: &Api,
        additional_params: &[(&str, &str)],
    ) -> Result<Value, MediaWikiError> {
        let title = self
            .title
            .full_pretty(api)
            .ok_or_else(|| MediaWikiError::BadTitle(self.title.clone()))?;
        let mut params = api.params_into(&[("action", "query"), ("titles", &title)]);
        for (k, v) in additional_params {
            params.insert(k.to_string(), v.to_string());
        }
        api.get_query_api_json_all(&params).await
    }

    // From an API result in the form of query/pages, extract a sub-object for each page (should be only one)
    fn extract_page_properties_from_api_results(
        &self,
        result: Value,
        subkey: &str,
    ) -> Result<Vec<Value>, MediaWikiError> {
        if result["query"]["pages"].is_null() {
            return Err(MediaWikiError::Missing(self.title.clone()));
        }

        result["query"]["pages"]
            .as_object()
            .map(|obj| {
                obj.iter()
                    .flat_map(|(_pageid, v_page)| {
                        v_page[subkey]
                            .as_array()
                            .map(|arr| arr.to_owned())
                            .unwrap_or_default()
                    })
                    .collect()
            })
            .ok_or_else(|| {
                MediaWikiError::UnexpectedResultFormat(format!("{:?}", &result["query"]["pages"]))
            })
    }

    fn json_result_into_titles(&self, arr: Vec<Value>, api: &Api) -> Vec<Title> {
        arr.iter()
            .filter_map(|v| {
                v["title"]
                    .as_str()
                    .map(|title| Title::new_from_full(title, api))
            })
            .collect()
    }

    /// Returns the categories of a page, as a JSON Value Vec
    pub async fn categories(&self, api: &Api) -> Result<Vec<Value>, MediaWikiError> {
        let result = self
            .action_query(
                api,
                &[
                    ("prop", "categories"),
                    ("cllimit", "max"),
                    ("clprop", "hidden|sortkey|timestamp"),
                ],
            )
            .await?;
        self.extract_page_properties_from_api_results(result, "categories")
    }

    /// Returns the interwiki links of a page, as a JSON Value Vec
    pub async fn interwiki_links(&self, api: &Api) -> Result<Vec<Value>, MediaWikiError> {
        let result = self
            .action_query(api, &[("prop", "iwlinks"), ("iwlimit", "max")])
            .await?;
        self.extract_page_properties_from_api_results(result, "iwlinks")
    }

    /// Returns the templates of a page, as a Title Vec
    pub async fn templates(&self, api: &Api) -> Result<Vec<Title>, MediaWikiError> {
        let result = self
            .action_query(
                api,
                &[
                    ("prop", "templates"),
                    ("tllimit", "max"),
                    ("tlnamespace", "*"),
                ],
            )
            .await?;
        let result = self.extract_page_properties_from_api_results(result, "templates")?;
        Ok(self.json_result_into_titles(result, api))
    }

    /// Returns the wiki-internal links on a page, as a Title Vec
    pub async fn links(&self, api: &Api) -> Result<Vec<Title>, MediaWikiError> {
        let result = self
            .action_query(
                api,
                &[("prop", "links"), ("pllimit", "max"), ("plnamespace", "*")],
            )
            .await?;
        let result = self.extract_page_properties_from_api_results(result, "links")?;
        Ok(self.json_result_into_titles(result, api))
    }

    /// Returns the wiki-internal links on a page, as a Title Vec
    pub async fn links_here(
        &self,
        api: &Api,
        direct_links: bool,
        redirects: bool,
    ) -> Result<Vec<Title>, MediaWikiError> {
        let lhshow = match (direct_links, redirects) {
            (true, true) => "!redirect|redirect",
            (true, false) => "!redirect",
            (false, true) => "redirect",
            (false, false) => "",
        };
        let result = self
            .action_query(
                api,
                &[
                    ("prop", "linkshere"),
                    ("lhlimit", "max"),
                    ("lhnamespace", "*"),
                    ("lhshow", lhshow),
                ],
            )
            .await?;
        let result = self.extract_page_properties_from_api_results(result, "linkshere")?;
        Ok(self.json_result_into_titles(result, api))
    }

    /// Returns the images used on a page, as a Title Vec
    pub async fn images(&self, api: &Api) -> Result<Vec<Title>, MediaWikiError> {
        let result = self
            .action_query(api, &[("prop", "images"), ("imlimit", "max")])
            .await?;
        let result = self.extract_page_properties_from_api_results(result, "images")?;
        Ok(self.json_result_into_titles(result, api))
    }

    /// Returns the coordinates of a page, as a JSON Value Vec
    pub async fn coordinates(&self, api: &Api) -> Result<Vec<Value>, MediaWikiError> {
        let result = self
            .action_query(
                api,
                &[
                    ("prop", "coordinates"),
                    ("cllimit", "max"),
                    ("coprop", "country|dim|globe|name|region|type"),
                    ("coprimary", "all"),
                ],
            )
            .await?;
        self.extract_page_properties_from_api_results(result, "coordinates")
    }

    /// Returns the coordinates of a page, including distance from a point, as a JSON Value Vec
    pub async fn coordinates_distance(
        &self,
        api: &Api,
        lat: f64,
        lon: f64,
    ) -> Result<Vec<Value>, MediaWikiError> {
        let distance_from_point = format!("{}|{}", lat, lon);
        let result = self
            .action_query(
                api,
                &[
                    ("prop", "coordinates"),
                    ("cllimit", "max"),
                    ("coprop", "country|dim|globe|name|region|type"),
                    ("coprimary", "all"),
                    ("codistancefrompoint", &distance_from_point),
                ],
            )
            .await?;
        self.extract_page_properties_from_api_results(result, "coordinates")
    }

    /// Returns the external links of a page, as a String Vec
    pub async fn external_links(&self, api: &Api) -> Result<Vec<String>, MediaWikiError> {
        let result = self
            .action_query(api, &[("prop", "extlinks"), ("ellimit", "max")])
            .await?;
        Ok(self
            .extract_page_properties_from_api_results(result, "extlinks")?
            .iter()
            .filter_map(|v| v["*"].as_str())
            .map(|v| v.to_string())
            .collect())
    }

    /// Returns the page ID (usually set after some API operation).
    pub fn page_id(&self) -> Option<usize> {
        self.page_id
    }

    /// Returns the loaded revision of the page (usually set after some API operation).
    pub fn revision(&self) -> Option<&Revision> {
        self.revision.as_ref()
    }

    /*
    TODO for action=query:
    extracts
    fileusage
    globalusage
    imageinfo
    images
    info
    langlinks
    linkshere
    pageimages
    pageprops
    pageterms
    pageviews
    redirects
    revisions
    transcludedin
    wbentityusage
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::*;
    use wiremock::matchers::query_param;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn wd_api() -> (MockServer, Api) {
        let server = crate::test_helpers::test_helpers_mod::start_wikidata_mock().await;
        let api = Api::new(&server.uri()).await.unwrap();
        (server, api)
    }

    #[tokio::test]
    async fn page_text_main_page_nonempty() {
        let (server, api) = wd_api().await;
        Mock::given(query_param("rvslots", "*"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"pages": [{
                    "pageid": 1234, "ns": 4, "title": "Wikidata:Main Page",
                    "revisions": [{"revid": 100, "slots": {"main": {"content": "Some non-empty page content."}}}]
                }]}
            })))
            .mount(&server)
            .await;
        let mut page = Page::new(Title::new("Main Page", 4));
        let text = page.text(&api).await.unwrap();
        assert!(!text.is_empty());
    }

    #[tokio::test]
    async fn page_text_nonexistent() {
        let (server, api) = wd_api().await;
        Mock::given(query_param("rvslots", "*"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"pages": [{"ns": 0, "title": "This page does not exist", "missing": true}]}
            })))
            .mount(&server)
            .await;
        let title = Title::new("This page does not exist", 0);
        let mut page = Page::new(title.clone());
        match page.text(&api).await {
            Err(MediaWikiError::Missing(t)) => assert!(t == title),
            x => panic!("expected missing error, found {:?}", x),
        }
    }

    #[tokio::test]
    async fn page_categories() {
        let (server, api) = wd_api().await;
        Mock::given(query_param("prop", "categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"pages": {"1": {
                    "pageid": 1, "ns": 4, "title": "Wikidata:Community portal",
                    "categories": [
                        {"ns": 14, "title": "Category:Wikidata"},
                        {"ns": 14, "title": "Category:Community"}
                    ]
                }}}
            })))
            .mount(&server)
            .await;
        let page = Page::new(Title::new("Community portal", 4));
        let result = page.categories(&api).await.unwrap();
        assert!(result.len() > 1);
    }

    #[tokio::test]
    async fn page_templates() {
        let (server, api) = wd_api().await;
        Mock::given(query_param("prop", "templates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"pages": {"1": {
                    "pageid": 1, "ns": 4, "title": "Wikidata:Community portal",
                    "templates": [
                        {"ns": 10, "title": "Template:Protected"},
                        {"ns": 10, "title": "Template:Documentation"},
                        {"ns": 10, "title": "Template:Navbox"},
                        {"ns": 10, "title": "Template:Reflist"},
                        {"ns": 10, "title": "Template:Cite web"},
                        {"ns": 10, "title": "Template:Short description"}
                    ]
                }}}
            })))
            .mount(&server)
            .await;
        let page = Page::new(Title::new("Community portal", 4));
        let result = page.templates(&api).await.unwrap();
        assert!(result.len() > 5);
        assert!(result.contains(&Title::new("Protected", 10)));
    }

    #[tokio::test]
    async fn page_coordinates() {
        let (server, api) = wd_api().await;
        // Simple coordinates mock (served first, up_to_n_times=1)
        Mock::given(query_param("prop", "coordinates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"pages": {"1": {
                    "pageid": 1, "ns": 0, "title": "Q64",
                    "coordinates": [{"lat": 52.5166667, "lon": 13.3833333, "primary": "", "globe": "earth"}]
                }}}
            })))
            .up_to_n_times(1)
            .mount(&server)
            .await;
        // Distance-including mock (served for the second coordinates call)
        Mock::given(query_param("prop", "coordinates"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"pages": {"1": {
                    "pageid": 1, "ns": 0, "title": "Q64",
                    "coordinates": [{"lat": 52.5166667, "lon": 13.3833333, "primary": "", "globe": "earth", "dist": 475700.5}]
                }}}
            })))
            .mount(&server)
            .await;
        let page = Page::new(Title::new("Q64", 0));
        let result = page.coordinates(&api).await.unwrap();
        assert!(!result.is_empty());
        let result = page
            .coordinates_distance(&api, 50.94222222, 6.95777778)
            .await
            .unwrap();
        result
            .iter()
            .filter(|v| v["primary"].as_str() == Some(""))
            .for_each(|v| {
                assert!(v["dist"].as_f64().unwrap() > 475700.0);
                assert!(v["dist"].as_f64().unwrap() < 475701.0);
            });
    }

    #[tokio::test]
    async fn page_external_links() {
        let (server, api) = wd_api().await;
        Mock::given(query_param("prop", "extlinks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"pages": {"1": {
                    "pageid": 1, "ns": 0, "title": "Q64",
                    "extlinks": [
                        {"*": "https://www.berlin.de/stadtplan/"},
                        {"*": "https://www.berlin.de/en/"}
                    ]
                }}}
            })))
            .mount(&server)
            .await;
        let page = Page::new(Title::new("Q64", 0));
        let result = page.external_links(&api).await.unwrap();
        assert!(result.contains(&"https://www.berlin.de/stadtplan/".to_string()));
    }

    #[tokio::test]
    async fn page_links() {
        let (server, api) = wd_api().await;
        Mock::given(query_param("prop", "links"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"pages": {"1": {
                    "pageid": 1, "ns": 4, "title": "Wikidata:Community portal",
                    "links": [
                        {"ns": 4, "title": "Wikidata:Bot requests"},
                        {"ns": 4, "title": "Wikidata:Help"}
                    ]
                }}}
            })))
            .mount(&server)
            .await;
        let page = Page::new(Title::new("Community portal", 4));
        let result = page.links(&api).await.unwrap();
        assert!(result.contains(&Title::new("Bot requests", 4)));
    }

    #[tokio::test]
    async fn page_images() {
        let (server, api) = wd_api().await;
        Mock::given(query_param("prop", "images"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"pages": {"1": {
                    "pageid": 1, "ns": 0, "title": "Q64",
                    "images": [
                        {"ns": 6, "title": "File:Cityscape Berlin.jpg"},
                        {"ns": 6, "title": "File:Berlin map.png"}
                    ]
                }}}
            })))
            .mount(&server)
            .await;
        let page = Page::new(Title::new("Q64", 0));
        let result = page.images(&api).await.unwrap();
        assert!(result.contains(&Title::new("Cityscape Berlin.jpg", 6)));
    }

    #[tokio::test]
    async fn page_links_here() {
        let (server, api) = wd_api().await;
        Mock::given(query_param("prop", "linkshere"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"pages": {"1": {
                    "pageid": 1, "ns": 0, "title": "Q1481",
                    "linkshere": [
                        {"ns": 0, "title": "Q7894"},
                        {"ns": 0, "title": "Q12345"}
                    ]
                }}}
            })))
            .mount(&server)
            .await;
        let page = Page::new(Title::new("Q1481", 0));
        let result = page.links_here(&api, true, false).await.unwrap();
        assert!(result.contains(&Title::new("Q7894", 0)));
    }

    #[tokio::test]
    async fn page_interwiki_links() {
        let (server, api) = wd_api().await;
        Mock::given(query_param("prop", "iwlinks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"pages": {"1": {
                    "pageid": 1, "ns": 10, "title": "Template:Wikidata list",
                    "iwlinks": [
                        {"prefix": "mw", "*": "Wikidata_query_service/User_Manual"},
                        {"prefix": "mw", "*": "Help:Contents"}
                    ]
                }}}
            })))
            .mount(&server)
            .await;
        let page = Page::new(Title::new("Wikidata list", 10));
        let result = page.interwiki_links(&api).await.unwrap();
        assert!(result.contains(&json!({"prefix":"mw","*":"Wikidata_query_service/User_Manual"})));
    }
}
