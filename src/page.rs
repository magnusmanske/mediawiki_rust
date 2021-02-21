/*!
The `Page` class deals with operations done on pages, like editing.
*/

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

use crate::media_wiki_error::MediaWikiError;
use crate::api::Api;
use crate::title::Title;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Represents a page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    title: Title,
}

impl Page {
    /// Creates a new `Page` from a `Title`.
    pub fn new(title: Title) -> Self {
        Page { title }
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
    /// # Errors
    /// If the page is missing, will return a `PageError::Missing`.
    ///
    /// [`Api::get_query_api_json`]: ../api/struct.Api.html#method.get_query_api_json
    pub async fn text(&self, api: &Api) -> Result<String, PageError> {
        let title = self
            .title
            .full_pretty(api)
            .ok_or_else(|| PageError::BadTitle(self.title.clone()))?;
        let params = [
            ("action", "query"),
            ("prop", "revisions"),
            ("titles", &title),
            ("rvslots", "*"),
            ("rvprop", "content"),
            ("formatversion", "2"),
        ]
        .iter()
        .map(|&(k, v)| (k.to_string(), v.to_string()))
        .collect();
        let result = api
            .get_query_api_json(&params)
            .await
            .map_err(PageError::MediaWiki)?;

        let page = &result["query"]["pages"][0];
        if page["missing"].as_bool() == Some(true) {
            Err(PageError::Missing(self.title.clone()))
        } else if let Some(slots) = page["revisions"][0]["slots"].as_object() {
            if let Some(the_slot) = {
                slots["main"].as_object().or_else(|| {
                    if slots.len() == 1 {
                        slots.values().next().unwrap().as_object() // unwrap OK, length is 1
                    } else {
                        None
                    }
                })
            } {
                match the_slot["content"].as_str() {
                    Some(string) => Ok(string.to_string()),
                    None => Err(PageError::BadResponse(result)),
                }
            } else {
                Err(PageError::BadResponse(result))
            }
        } else {
            Err(PageError::BadResponse(result))
        }
    }

    /// Replaces the contents of this `Page` with the given text, using the given
    /// edit summary.
    ///
    /// # Errors
    /// May return a `PageError` or any error from [`Api::post_query_api_json`].
    ///
    /// [`Api::post_query_api_json`]: ../api/struct.Api.html#method.post_query_api_json
    pub async fn edit_text(
        &self,
        api: &mut Api,
        text: impl Into<String>,
        summary: impl Into<String>,
    ) -> Result<(), Box<dyn Error>> {
        let title = self
            .title
            .full_pretty(api)
            .ok_or_else(|| PageError::BadTitle(self.title.clone()))?;
        let bot = if api.user().is_bot() { "true" } else { "false" };
        let mut params: HashMap<String, String> = [
            ("action", "edit"),
            ("title", &title),
            ("text", &text.into()),
            ("summary", &summary.into()),
            ("bot", bot),
            ("formatversion", "2"),
            ("token", &api.get_edit_token().await?),
        ]
        .iter()
        .map(|&(k, v)| (k.to_string(), v.to_string()))
        .collect();

        if !api.user().user_name().is_empty() {
            params.insert("assert".to_string(), "user".to_string());
        }

        let result = api.post_query_api_json(&params).await?;
        match result["edit"]["result"].as_str() {
            Some("Success") => Ok(()),
            _ => Err(Box::new(PageError::EditError(result))),
        }
    }

    /// Performs an "action=query" API action and returns the result.
    async fn action_query(
        &self,
        api: &Api,
        additional_params: &[(&str, &str)],
    ) -> Result<Value, PageError> {
        let title = self
            .title
            .full_pretty(api)
            .ok_or_else(|| PageError::BadTitle(self.title.clone()))?;
        let mut params = api.params_into(&[("action", "query"), ("titles", &title)]);
        for (k, v) in additional_params {
            params.insert(k.to_string(), v.to_string());
        }
        api.get_query_api_json_all(&params).await.map_err(|e|PageError::RequestError(Box::new(e)))
    }

    // From an API result in the form of query/pages, extract a sub-object for each page (should be only one)
    fn extract_page_properties_from_api_results(
        &self,
        result: Value,
        subkey: &str,
    ) -> Result<Vec<Value>, Box<dyn Error>> {
        match result["query"]["pages"].is_null() {
            true => Err(Box::new(PageError::Missing(self.title.clone()))),
            false => match result["query"]["pages"].as_object() {
                Some(obj) => Ok(obj
                    .iter()
                    .flat_map(|(_pageid, v_page)| match v_page[subkey].as_array() {
                        Some(arr) => arr.to_owned(),
                        None => vec![],
                    })
                    .collect()),
                None => Err(Box::new(PageError::UnexpectedResultFormat(format!(
                    "{:?}",
                    &result["query"]["pages"]
                )))),
            },
        }
    }

    fn json_result_into_titles(&self, arr: Vec<Value>, api: &Api) -> Vec<Title> {
        arr.iter()
            .filter_map(|v| match v["title"].as_str() {
                Some(title) => Some(Title::new_from_full(title, api)),
                None => None,
            })
            .collect()
    }

    /// Returns the categories of a page, as a JSON Value Vec
    pub async fn categories(&self, api: &Api) -> Result<Vec<Value>, Box<dyn Error>> {
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

    /// Returns the categories of a page, as a JSON Value Vec
    pub async fn interwiki_links(&self, api: &Api) -> Result<Vec<Value>, Box<dyn Error>> {
        let result = self
            .action_query(api, &[("prop", "iwlinks"), ("iwlimit", "max")])
            .await?;
        self.extract_page_properties_from_api_results(result, "iwlinks")
    }

    /// Returns the templates of a page, as a Title Vec
    pub async fn templates(&self, api: &Api) -> Result<Vec<Title>, Box<dyn Error>> {
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
    pub async fn links(&self, api: &Api) -> Result<Vec<Title>, Box<dyn Error>> {
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
    ) -> Result<Vec<Title>, Box<dyn Error>> {
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
    pub async fn images(&self, api: &Api) -> Result<Vec<Title>, Box<dyn Error>> {
        let result = self
            .action_query(api, &[("prop", "images"), ("imlimit", "max")])
            .await?;
        let result = self.extract_page_properties_from_api_results(result, "images")?;
        Ok(self.json_result_into_titles(result, api))
    }

    /// Returns the coordinates of a page, as a JSON Value Vec
    pub async fn coordinates(&self, api: &Api) -> Result<Vec<Value>, Box<dyn Error>> {
        self.extract_page_properties_from_api_results(
            self.action_query(
                api,
                &[
                    ("prop", "coordinates"),
                    ("cllimit", "max"),
                    ("coprop", "country|dim|globe|name|region|type"),
                    ("coprimary", "all"),
                ],
            )
            .await?,
            "coordinates",
        )
    }

    /// Returns the coordinates of a page, including distance from a point, as a JSON Value Vec
    pub async fn coordinates_distance(
        &self,
        api: &Api,
        lat: f64,
        lon: f64,
    ) -> Result<Vec<Value>, Box<dyn Error>> {
        self.extract_page_properties_from_api_results(
            self.action_query(
                api,
                &[
                    ("prop", "coordinates"),
                    ("cllimit", "max"),
                    ("coprop", "country|dim|globe|name|region|type"),
                    ("coprimary", "all"),
                    ("codistancefrompoint", format!("{}|{}", lat, lon).as_str()),
                ],
            )
            .await?,
            "coordinates",
        )
    }

    /// Returns the external links of a page, as a String Vec
    pub async fn external_links(&self, api: &Api) -> Result<Vec<String>, Box<dyn Error>> {
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

/// Errors that can go wrong while performing operations on a `Page`.
#[derive(Debug)]
#[non_exhaustive]
pub enum PageError {
    /// Couldn't obtain the title for this page for use in an API request.
    BadTitle(Title),

    /// Couldn't understand the API response (provided).
    BadResponse(Value),

    /// Missing page.
    Missing(Title),

    /// Edit failed; API response is provided.
    EditError(Value),

    /// Error while performing the API request.
    RequestError(Box<dyn Error>),

    /// Unexpected data structure (eg array instead of object) in API JSON result
    UnexpectedResultFormat(String),

    /// MediaWikiError wrapper
    MediaWiki(MediaWikiError),
}

impl fmt::Display for PageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PageError::BadTitle(title) => write!(f, "invalid title for this Page: {:?}", title),
            PageError::BadResponse(response) => write!(
                f,
                "bad API response while fetching revision content: {:?}",
                response
            ),
            PageError::Missing(title) => write!(f, "page missing: {:?}", title),
            PageError::EditError(response) => write!(f, "edit resulted in error: {:?}", response),
            PageError::RequestError(error) => write!(f, "request error: {}", error),
            PageError::UnexpectedResultFormat(error) => write!(f, "result format error: {}", error),
            PageError::MediaWiki(error) => write!(f, "result format error: {}", error),
        }
    }
}

impl Error for PageError {}
/*
impl From<MediaWikiError> for PageError {  
    fn from(e: MediaWikiError) -> Self {
        match e {
            MediaWikiError::Reqwest(e) => PageError::RequestError(Box::new(e)),
            MediaWikiError::ReqwestHeader(e) => PageError::RequestError(Box::new(e)),
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::*;

    async fn wd_api() -> Api {
        Api::new("https://www.wikidata.org/w/api.php")
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn page_text_main_page_nonempty() {
        let page = Page::new(Title::new("Main Page", 4));
        let text = page.text(&wd_api().await).await.unwrap();
        assert!(!text.is_empty());
    }

    #[tokio::test]
    async fn page_text_nonexistent() {
        let title = Title::new("This page does not exist", 0);
        let page = Page::new(title.clone());
        match page.text(&wd_api().await).await {
            Err(PageError::Missing(t)) => assert!(t == title),
            x => panic!("expected missing error, found {:?}", x),
        }
    }

    #[tokio::test]
    async fn page_categories() {
        let page = Page::new(Title::new("Community portal", 4));
        let result = page.categories(&wd_api().await).await.unwrap();
        assert!(result.len() > 1);
    }

    #[tokio::test]
    async fn page_templates() {
        let page = Page::new(Title::new("Community portal", 4));
        let result = page.templates(&wd_api().await).await.unwrap();
        assert!(result.len() > 5);
        assert!(result.contains(&Title::new("Protected", 10)))
    }

    #[tokio::test]
    async fn page_coordinates() {
        let page = Page::new(Title::new("Q64", 0)); // Berlin
        let result = page.coordinates(&wd_api().await).await.unwrap();
        assert!(!result.is_empty());

        // Distance to Cologne
        let result = page
            .coordinates_distance(&wd_api().await, 50.94222222, 6.95777778)
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
        let page = Page::new(Title::new("Q64", 0));
        let result = page.external_links(&wd_api().await).await.unwrap();
        assert!(result.contains(&"https://www.berlin.de/".to_string()));
    }

    #[tokio::test]
    async fn page_links() {
        let page = Page::new(Title::new("Community portal", 4));
        let result = page.links(&wd_api().await).await.unwrap();
        assert!(result.contains(&Title::new("Bot requests", 4)))
    }

    #[tokio::test]
    async fn page_images() {
        let page = Page::new(Title::new("Q64", 0));
        let result = page.images(&wd_api().await).await.unwrap();
        assert!(result.contains(&Title::new("Cityscape Berlin.jpg", 6)))
    }

    #[tokio::test]
    async fn page_links_here() {
        let page = Page::new(Title::new("Q1481", 0));
        let result = page.links_here(&wd_api().await, true, false).await.unwrap();
        assert!(result.contains(&Title::new("Q7894", 0)))
    }

    #[tokio::test]
    async fn page_interwiki_links() {
        let page = Page::new(Title::new("Wikidata list", 10));
        let result = page.interwiki_links(&wd_api().await).await.unwrap();
        // println!("{:?}", &result);
        assert!(result.contains(&json!({"prefix":"mw","*":"Wikidata_query_service/User_Manual"})));
    }
}
