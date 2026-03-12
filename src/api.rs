/*!
The `Api` class serves as a universal interface to a MediaWiki API.
*/

#![deny(missing_docs)]

use crate::media_wiki_error::MediaWikiError;
use crate::title::Title;
use crate::user::User;
use base64::prelude::*;
use futures::{Stream, StreamExt};
use hmac::{Hmac, Mac};
use nanoid::nanoid;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{RequestBuilder, StatusCode};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use url::Url;

/// Alias for a namespace (could be -1 for Special pages etc.)
pub type NamespaceID = i64;

const DEFAULT_USER_AGENT: &str = "Rust mediawiki API";
const DEFAULT_MAXLAG: Option<u64> = Some(5);
const DEFAULT_MAX_RETRY_ATTEMPTS: u64 = 5;
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);
const DEFAULT_DELAY_FOR_TOO_MANY_REQUESTS: u64 = 30;

type HmacSha1 = Hmac<sha1::Sha1>;

/// `OAuthParams` contains parameters for OAuth requests.
///
/// Some fields are stored for completeness when deserializing from JSON
/// but are not actively used by this library.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct OAuthParams {
    /// Consumer Key
    pub g_consumer_key: Option<String>,
    /// Consumer secret
    pub g_consumer_secret: Option<String>,
    /// Token key
    pub g_token_key: Option<String>,
    /// Token secret
    pub g_token_secret: Option<String>,
    /// User agent (stored for completeness)
    g_user_agent: Option<String>,
    /// Agent identifier (stored for completeness)
    agent: Option<String>,
    /// Alternative consumer key field (stored for completeness)
    consumer_key: Option<String>,
    /// Alternative consumer secret field (stored for completeness)
    consumer_secret: Option<String>,
    /// API URL (stored for completeness)
    api_url: Option<String>,
    /// Public MW OAuth URL (stored for completeness)
    public_mw_oauth_url: Option<String>,
    /// Tool identifier (stored for completeness)
    tool: Option<String>,
}

impl OAuthParams {
    /// Imports data from JSON stored in the QuickStatements DB batch_oauth.serialized_json field
    pub fn new_from_json(j: &Value) -> Self {
        Self {
            g_consumer_key: j["gConsumerKey"].as_str().map(ToString::to_string),
            g_consumer_secret: j["gConsumerSecret"].as_str().map(ToString::to_string),
            g_token_key: j["gTokenKey"].as_str().map(ToString::to_string),
            g_token_secret: j["gTokenSecret"].as_str().map(ToString::to_string),
            g_user_agent: j["gUserAgent"].as_str().map(ToString::to_string),
            agent: j["params"]["agent"].as_str().map(ToString::to_string),
            consumer_key: j["params"]["consumerKey"].as_str().map(ToString::to_string),
            consumer_secret: j["params"]["consumerSecret"]
                .as_str()
                .map(ToString::to_string),
            api_url: j["apiUrl"].as_str().map(ToString::to_string),
            public_mw_oauth_url: j["publicMwOAuthUrl"].as_str().map(ToString::to_string),
            tool: j["tool"].as_str().map(ToString::to_string),
        }
    }
}

/// `Api` is the main class to interact with a MediaWiki API
#[derive(Debug, Clone)]
pub struct Api {
    api_url: String,
    site_info: Value,
    client: reqwest::Client,
    user: User,
    user_agent: String,
    maxlag_seconds: Option<u64>,
    edit_delay_ms: Option<u64>,
    max_retry_attempts: u64,
    oauth: Option<OAuthParams>,
    oauth2: Option<String>,
}

impl Api {
    /// Returns a new `Api` element, and loads the MediaWiki site info from the `api_url` site.
    /// This is done both to get basic information about the site, and to test the API.
    ///
    /// # Examples
    ///
    /// ```
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let api = mediawiki::api::Api::new("https://en.wikipedia.org/w/api.php").await.unwrap();
    /// # });
    /// ```
    pub async fn new(api_url: &str) -> Result<Api, MediaWikiError> {
        Api::new_from_builder(api_url, reqwest::Client::builder().timeout(DEFAULT_TIMEOUT)).await
    }

    /// Returns a new `Api` element, and loads the MediaWiki site info from the `api_url` site.
    /// This is done both to get basic information about the site, and to test the API.
    /// Uses a bespoke reqwest::ClientBuilder.
    pub async fn new_from_builder(
        api_url: &str,
        builder: reqwest::ClientBuilder,
    ) -> Result<Api, MediaWikiError> {
        let mut ret = Api {
            api_url: api_url.to_string(),
            site_info: Value::Object(Default::default()),
            client: builder.cookie_store(true).build()?,
            user: User::new(),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            maxlag_seconds: DEFAULT_MAXLAG,
            max_retry_attempts: DEFAULT_MAX_RETRY_ATTEMPTS,
            edit_delay_ms: None,
            oauth: None,
            oauth2: None,
        };
        ret.load_site_info().await?;
        Ok(ret)
    }

    /// Returns the API url
    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    /// Sets the OAuth parameters
    pub fn set_oauth(&mut self, oauth: Option<OAuthParams>) {
        self.oauth = oauth;
    }

    /// Set an OAuth 2 access token
    pub fn set_oauth2(&mut self, oauth2: &str) {
        self.oauth2 = Some(oauth2.to_string());
    }

    /// Returns a reference to the current OAuth parameters
    pub fn oauth(&self) -> &Option<OAuthParams> {
        &self.oauth
    }

    /// Returns a reference to the reqwest client
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Returns a mutable reference to the reqwest client
    pub fn client_mut(&mut self) -> &mut reqwest::Client {
        &mut self.client
    }

    /// Returns a reference to the current user object
    pub fn user(&self) -> &User {
        &self.user
    }

    /// Returns a mutable reference to the current user object
    pub fn user_mut(&mut self) -> &mut User {
        &mut self.user
    }

    /// Loads the current user info; returns Ok(()) is successful
    pub async fn load_current_user_info(&mut self) -> Result<(), MediaWikiError> {
        let mut user = std::mem::take(&mut self.user);
        self.load_user_info(&mut user).await?;
        self.user = user;
        Ok(())
    }

    /// Returns the maximum number of retry attempts
    pub fn max_retry_attempts(&self) -> u64 {
        self.max_retry_attempts
    }

    /// Sets the maximum number of retry attempts
    pub fn set_max_retry_attempts(&mut self, max_retry_attempts: u64) {
        self.max_retry_attempts = max_retry_attempts;
    }

    /// Returns a reference to the serde_json Value containing the site info
    pub fn get_site_info(&self) -> &Value {
        &self.site_info
    }

    /// Returns a serde_json Value in site info, within the `["query"]` object.
    pub fn get_site_info_value<'a>(&'a self, k1: &str, k2: &str) -> &'a Value {
        &self.get_site_info()["query"][k1][k2]
    }

    /// Returns a String from the site info, matching `["query"][k1][k2]`
    pub fn get_site_info_string<'a>(
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

    /// Returns the raw data for the namespace, matching `["query"]["namespaces"][namespace_id]`
    pub fn get_namespace_info(&self, namespace_id: NamespaceID) -> &Value {
        self.get_site_info_value("namespaces", &namespace_id.to_string())
    }

    /// Returns the canonical namespace name for a namespace ID, if defined
    pub fn get_canonical_namespace_name(&self, namespace_id: NamespaceID) -> Option<&str> {
        let info = self.get_namespace_info(namespace_id);
        info["canonical"].as_str().or_else(|| info["*"].as_str())
    }

    /// Returns the local namespace name for a namespace ID, if defined
    pub fn get_local_namespace_name(&self, namespace_id: NamespaceID) -> Option<&str> {
        let info = self.get_namespace_info(namespace_id);
        info["*"].as_str().or_else(|| info["canonical"].as_str())
    }

    /// Loads the site info.
    /// Should only ever be called from `new()`
    async fn load_site_info(&mut self) -> Result<&Value, MediaWikiError> {
        let params = hashmap!["action".to_string()=>"query".to_string(),"meta".to_string()=>"siteinfo".to_string(),"siprop".to_string()=>"general|namespaces|namespacealiases|libraries|extensions|statistics".to_string()];
        self.site_info = self.get_query_api_json(&params).await?;
        Ok(&self.site_info)
    }

    /// Merges two JSON objects that are MediaWiki API results.
    /// If an array already exists in the `a` object, it will be expanded with the array from the `b` object
    /// This allows for combining multiple API results via the `continue` parameter
    fn json_merge(a: &mut Value, b: Value) {
        match (a, b) {
            (a @ &mut Value::Object(_), Value::Object(b)) => {
                if let Some(a) = a.as_object_mut() {
                    for (k, v) in b {
                        Self::json_merge(a.entry(k).or_insert(Value::Null), v);
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

    /// Turns a Vec of str tuples into a Hashmap of String, to be used in API calls
    pub fn params_into(&self, params: &[(&str, &str)]) -> HashMap<String, String> {
        params
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    /// Returns an empty parameter HashMap
    pub fn no_params(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    /// Returns a token of a `token_type`, such as `login` or `csrf` (for editing)
    pub async fn get_token(&mut self, token_type: &str) -> Result<String, MediaWikiError> {
        let mut params = hashmap!["action".to_string()=>"query".to_string(),"meta".to_string()=>"tokens".to_string()];
        if !token_type.is_empty() {
            params.insert("type".to_string(), token_type.to_string());
        }
        let mut key = token_type.to_string();
        key += "token";
        if token_type.is_empty() {
            key = "csrftoken".into()
        }
        let x = self.query_api_json_mut(&params, "GET").await?;
        match &x["query"]["tokens"][&key] {
            Value::String(s) => Ok(s.to_string()),
            _ => Err(From::from(format!("Could not get token: {:?}", x))),
        }
    }

    /// Calls `get_token()` to return an edit token
    pub async fn get_edit_token(&mut self) -> Result<String, MediaWikiError> {
        self.get_token("csrf").await
    }

    /// Same as `get_query_api_json` but automatically loads all results via the `continue` parameter
    pub async fn get_query_api_json_all(
        &self,
        params: &HashMap<String, String>,
    ) -> Result<Value, MediaWikiError> {
        self.get_query_api_json_limit(params, None).await
    }

    /// Tries to return the len() of an API query result. Returns 0 if unknown
    fn query_result_count(&self, result: &Value) -> usize {
        match result["query"].as_object() {
            Some(query) => query
                .iter()
                .filter_map(|(_key, part)| part.as_array().map(|a| a.len()))
                .next()
                .unwrap_or(0),
            None => 0, // Don't know size
        }
    }

    /// Same as `get_query_api_json` but automatically loads more results via the `continue` parameter
    pub async fn get_query_api_json_limit(
        &self,
        params: &HashMap<String, String>,
        max: Option<usize>,
    ) -> Result<Value, MediaWikiError> {
        self.get_query_api_json_limit_iter(params, max)
            .await
            .fold(Ok(Value::Null), |acc, result| async move {
                match (acc, result) {
                    (Ok(mut acc), Ok(result)) => {
                        Self::json_merge(&mut acc, result);
                        Ok(acc)
                    }
                    (Ok(_), e @ Err(_)) => e,
                    (e @ Err(_), _) => e,
                }
            })
            .await
    }

    /// Same as `get_query_api_json` but automatically loads more results via the `continue` parameter.
    /// Returns a stream; each item is a "page" of results.
    pub async fn get_query_api_json_limit_iter<'a>(
        &'a self,
        params: &HashMap<String, String>,
        max: Option<usize>,
    ) -> impl Stream<Item = Result<Value, MediaWikiError>> + 'a {
        struct QueryState<'a> {
            api: &'a Api,
            params: HashMap<String, String>,
            values_remaining: Option<usize>,
            continue_params: Value,
        }

        let initial_query_state = QueryState {
            api: self,
            params: params.clone(),
            values_remaining: max,
            continue_params: Value::Null,
        };

        futures::stream::unfold(initial_query_state, |mut query_state| async move {
            if let Some(0) = query_state.values_remaining {
                return None;
            }

            let mut current_params = query_state.params.clone();
            if let Value::Object(obj) = &query_state.continue_params {
                current_params.extend(
                    obj.iter()
                        // The default to_string() method for Value puts double-quotes around strings
                        .map(|(k, v)| {
                            (k.to_string(), v.as_str().map_or(v.to_string(), Into::into))
                        }),
                );
            }

            let query_result = query_state.api.get_query_api_json(&current_params).await;

            let ret = match query_result {
                Ok(mut result) => {
                    query_state.continue_params = result["continue"].clone();
                    if query_state.continue_params.is_null() {
                        query_state.values_remaining = Some(0);
                    } else if let Some(num) = query_state.values_remaining {
                        query_state.values_remaining =
                            Some(num.saturating_sub(query_state.api.query_result_count(&result)));
                    }
                    result.as_object_mut().map(|r| r.remove("continue"));
                    Ok(result)
                }
                e @ Err(_) => {
                    query_state.values_remaining = Some(0);
                    e
                }
            };
            Some((ret, query_state))
        })
    }

    /// Runs a query against the MediaWiki API, using `method` GET or POST.
    /// Parameters are a hashmap; `format=json` is enforced.
    pub async fn query_api_json(
        &self,
        params: &HashMap<String, String>,
        method: &str,
    ) -> Result<Value, MediaWikiError> {
        let mut params = params.clone();
        let mut attempts_left = self.max_retry_attempts;
        params.insert("format".to_string(), "json".to_string());
        let mut cumulative: u64 = 0;
        loop {
            self.set_cumulative_maxlag_params(&mut params, method, cumulative);
            let t = self.query_api_raw(&params, method).await?;
            let v: Value = serde_json::from_str(&t)?;
            match self.check_maxlag(&v) {
                Some(lag_seconds) => {
                    if attempts_left == 0 {
                        return Err(From::from(format!(
                            "Max attempts reached [MAXLAG] after {} attempts, cumulative maxlag {}",
                            &self.max_retry_attempts, cumulative
                        )));
                    }
                    attempts_left -= 1;
                    cumulative += lag_seconds;
                    tokio::time::sleep(Duration::from_millis(1000 * lag_seconds)).await;
                }
                None => return Ok(v),
            }
        }
    }

    /// Runs a query against the MediaWiki API, using `method` GET or POST.
    /// Parameters are a hashmap; `format=json` is enforced.
    async fn query_api_json_mut(
        &mut self,
        params: &HashMap<String, String>,
        method: &str,
    ) -> Result<Value, MediaWikiError> {
        let mut params = params.clone();
        let mut attempts_left = self.max_retry_attempts;
        params.insert("format".to_string(), "json".to_string());
        let mut cumulative: u64 = 0;
        loop {
            self.set_cumulative_maxlag_params(&mut params, method, cumulative);
            let t = self.query_api_raw_mut(&params, method).await?;
            let v: Value = serde_json::from_str(&t)?;
            match self.check_maxlag(&v) {
                Some(lag_seconds) => {
                    if attempts_left == 0 {
                        return Err(From::from(format!(
                            "Max attempts reached [MAXLAG] after {} attempts, cumulative maxlag {}",
                            &self.max_retry_attempts, cumulative
                        )));
                    }
                    attempts_left -= 1;
                    cumulative += lag_seconds;
                    tokio::time::sleep(Duration::from_millis(1000 * lag_seconds)).await;
                }
                None => return Ok(v),
            }
        }
    }

    /// Returns the delay time after edits, in milliseconds, if set
    pub fn edit_delay(&self) -> &Option<u64> {
        &self.edit_delay_ms
    }

    /// Sets the delay time after edits in milliseconds (or `None`).
    /// This is independent of, and additional to, MAXLAG
    pub fn set_edit_delay(&mut self, edit_delay_ms: Option<u64>) {
        self.edit_delay_ms = edit_delay_ms;
    }

    /// Returns the maxlag, in seconds, if set
    pub fn maxlag(&self) -> &Option<u64> {
        &self.maxlag_seconds
    }

    /// Sets the maxlag in seconds (or `None`)
    pub fn set_maxlag(&mut self, maxlag_seconds: Option<u64>) {
        self.maxlag_seconds = maxlag_seconds;
    }

    /// Checks if a query is an edit, based on parameters and method (GET/POST)
    fn is_edit_query(&self, params: &HashMap<String, String>, method: &str) -> bool {
        // Editing only through POST (?)
        if method != "POST" {
            return false;
        }
        // Editing requires a token
        if !params.contains_key("token") {
            return false;
        }
        true
    }

    /// Sets the maxlag parameter for a query, if necessary
    fn set_cumulative_maxlag_params(
        &self,
        params: &mut HashMap<String, String>,
        method: &str,
        cumulative: u64,
    ) {
        if !self.is_edit_query(params, method) {
            return;
        }
        if let Some(maxlag_seconds) = self.maxlag_seconds {
            let added = cumulative + maxlag_seconds;
            params.insert("maxlag".to_string(), added.to_string());
        }
    }

    /// Checks for a maxlag error, and returns the lag if so
    fn check_maxlag(&self, v: &Value) -> Option<u64> {
        match v["error"]["code"].as_str() {
            Some("maxlag") => v["error"]["lag"].as_u64().or(self.maxlag_seconds), // Current lag, if given, or fallback
            _ => None,
        }
    }

    /// GET wrapper for `query_api_json`
    pub async fn get_query_api_json(
        &self,
        params: &HashMap<String, String>,
    ) -> Result<Value, MediaWikiError> {
        self.query_api_json(params, "GET").await
    }

    /// POST wrapper for `query_api_json`
    pub async fn post_query_api_json(
        &self,
        params: &HashMap<String, String>,
    ) -> Result<Value, MediaWikiError> {
        self.query_api_json(params, "POST").await
    }

    /// POST wrapper for `query_api_json`.
    /// Requires `&mut self`, for session cookie storage
    pub async fn post_query_api_json_mut(
        &mut self,
        params: &HashMap<String, String>,
    ) -> Result<Value, MediaWikiError> {
        self.query_api_json_mut(params, "POST").await
    }

    /// Runs a query against the MediaWiki API, and returns a text.
    /// Uses `query_raw`
    pub async fn query_api_raw(
        &self,
        params: &HashMap<String, String>,
        method: &str,
    ) -> Result<String, MediaWikiError> {
        self.query_raw(&self.api_url, params, method).await
    }

    /// Runs a query against the MediaWiki API, and returns a text.
    /// Uses `query_raw_mut`
    async fn query_api_raw_mut(
        &mut self,
        params: &HashMap<String, String>,
        method: &str,
    ) -> Result<String, MediaWikiError> {
        self.query_raw_mut(&self.api_url.clone(), params, method)
            .await
    }

    /// Generates a `RequestBuilder` for the API URL
    pub fn get_api_request_builder(
        &self,
        params: &HashMap<String, String>,
        method: &str,
    ) -> Result<RequestBuilder, MediaWikiError> {
        self.request_builder(&self.api_url, params, method)
    }

    /// Returns the user agent name
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Sets the user agent name
    pub fn set_user_agent<S: Into<String>>(&mut self, agent: S) {
        self.user_agent = agent.into();
    }

    /// Returns the user agent string, as it is passed to the API through a HTTP header
    pub fn user_agent_full(&self) -> String {
        format!(
            "{}; {}-rust/{}",
            self.user_agent,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
    }

    /// Encodes a string
    fn rawurlencode(&self, s: &str) -> String {
        urlencoding::encode(s).into_owned()
    }

    /// Signs an OAuth request
    fn sign_oauth_request(
        &self,
        method: &str,
        api_url: &str,
        to_sign: &HashMap<String, String>,
        oauth: &OAuthParams,
    ) -> Result<String, MediaWikiError> {
        let mut keys: Vec<String> = to_sign.keys().map(|k| self.rawurlencode(k)).collect();
        keys.sort();

        let ret: Vec<String> = keys
            .iter()
            .filter_map(|k| match to_sign.get(k) {
                Some(k2) => {
                    let v = self.rawurlencode(k2);
                    Some(k.clone() + "=" + &v)
                }
                None => None,
            })
            .collect();

        let url = Url::parse(api_url)?;
        let mut url_string = url.scheme().to_owned() + "://";
        url_string += url.host_str().ok_or("url.host_str is None")?;
        if let Some(port) = url.port() {
            write!(url_string, ":{}", port)?
        }
        url_string += url.path();

        let ret = self.rawurlencode(method)
            + "&"
            + &self.rawurlencode(&url_string)
            + "&"
            + &self.rawurlencode(&ret.join("&"));

        let key: String = match (&oauth.g_consumer_secret, &oauth.g_token_secret) {
            (Some(g_consumer_secret), Some(g_token_secret)) => {
                self.rawurlencode(g_consumer_secret) + "&" + &self.rawurlencode(g_token_secret)
            }
            _ => {
                return Err(From::from("g_consumer_secret or g_token_secret not set"));
            }
        };

        let mut hmac =
            HmacSha1::new_from_slice(&key.into_bytes()).map_err(|e| format!("{:?}", e))?;
        hmac.update(&ret.into_bytes());
        let bytes = hmac.finalize().into_bytes();
        let ret: String = BASE64_STANDARD.encode(bytes);

        Ok(ret)
    }

    /// Returns a signed OAuth POST `RequestBuilder`
    fn oauth_request_builder(
        &self,
        method: &str,
        api_url: &str,
        params: &HashMap<String, String>,
    ) -> Result<RequestBuilder, MediaWikiError> {
        let oauth = match &self.oauth {
            Some(oauth) => oauth,
            None => {
                return Err(From::from(
                    "oauth_request_builder called but self.oauth is None",
                ));
            }
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .to_string();

        let nonce = nanoid!(10);

        let mut headers = HeaderMap::new();

        headers.insert(
            "oauth_consumer_key",
            oauth
                .g_consumer_key
                .as_ref()
                .ok_or("Failed to get ref for oauth_consumer_key")?
                .parse()?,
        );
        headers.insert(
            "oauth_token",
            oauth
                .g_token_key
                .as_ref()
                .ok_or("Falied to get ref for g_token_key")?
                .parse()?,
        );
        headers.insert("oauth_version", "1.0".parse()?);
        headers.insert("oauth_nonce", nonce.parse()?);
        headers.insert("oauth_timestamp", timestamp.parse()?);
        headers.insert("oauth_signature_method", "HMAC-SHA1".parse()?);

        // Prepage signing
        let mut to_sign = params.clone();
        for (key, value) in headers.iter() {
            if key == "oauth_signature" {
                continue;
            }
            to_sign.insert(key.to_string(), value.to_str()?.to_string());
        }

        headers.insert(
            "oauth_signature",
            self.sign_oauth_request(method, api_url, &to_sign, oauth)?
                .parse()?,
        );

        // Collapse headers
        let mut header = "OAuth ".to_string();
        let mut parts = Vec::new();
        for (key, value) in &headers {
            let key = key.to_string();
            let value = value.to_str().map_err(|e| e.to_string())?;
            let key = self.rawurlencode(&key);
            let value = self.rawurlencode(value);
            let part = key + "=\"" + &value + "\"";
            parts.push(part);
        }
        header += &parts.join(", ");

        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(header.as_str())?,
        );
        headers.insert(reqwest::header::USER_AGENT, self.user_agent_full().parse()?);

        match method {
            "GET" => Ok(self.client.get(api_url).headers(headers).query(&params)),
            "POST" => Ok(self.client.post(api_url).headers(headers).form(&params)),
            other => Err(MediaWikiError::String(format!(
                "Unsupported method '{}' for OAuth requests",
                other
            ))),
        }
    }

    /// Returns a `RequestBuilder` for a generic URL
    fn request_builder(
        &self,
        api_url: &str,
        params: &HashMap<String, String>,
        method: &str,
    ) -> Result<RequestBuilder, MediaWikiError> {
        // Use OAuth if set
        if self.oauth.is_some() {
            return self.oauth_request_builder(method, api_url, params);
        }

        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, self.user_agent_full().parse()?);
        if let Some(access_token) = &self.oauth2 {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", access_token).parse()?,
            );
        }

        Ok(match method {
            "GET" => self.client.get(api_url).headers(headers).query(&params),
            "POST" => self.client.post(api_url).headers(headers).form(&params),
            "PATCH" => self.client.patch(api_url).headers(headers).form(&params),
            "PUT" => self.client.put(api_url).headers(headers).form(&params),
            "DELETE" => self.client.delete(api_url).headers(headers).form(&params),
            other => return Err(From::from(format!("Unsupported method '{}'", other))),
        })
    }

    /// Performs a query, pauses if required, and returns the raw response
    async fn query_raw_response(
        &self,
        api_url: &str,
        params: &HashMap<String, String>,
        method: &str,
    ) -> Result<reqwest::Response, MediaWikiError> {
        let mut response;
        loop {
            let req = self.request_builder(api_url, params, method)?;
            response = req.send().await?;

            // If the API is overloaded, wait the requested time and try again
            if response.status() == StatusCode::TOO_MANY_REQUESTS {
                let wait_sec: u64 = response
                    .headers()
                    .get("Retry-After")
                    .map(|v| v.as_bytes())
                    .and_then(|bytes| std::str::from_utf8(bytes).ok())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(DEFAULT_DELAY_FOR_TOO_MANY_REQUESTS); // Fallback value
                tokio::time::sleep(Duration::from_secs(wait_sec)).await;
                continue;
            }

            break;
        }
        self.enact_edit_delay(params, method).await;
        Ok(response)
    }

    /// Delays the current thread, if the query performs an edit, and a delay time is set
    async fn enact_edit_delay(&self, params: &HashMap<String, String>, method: &str) {
        if !self.is_edit_query(params, method) {
            return;
        }
        if let Some(ms) = self.edit_delay_ms {
            tokio::time::sleep(Duration::from_millis(ms)).await;
        }
    }

    /// Runs a query against a generic URL, stores cookies, and returns a text
    /// Used for non-stateless queries, such as logins
    async fn query_raw_mut(
        &mut self,
        api_url: &str,
        params: &HashMap<String, String>,
        method: &str,
    ) -> Result<String, MediaWikiError> {
        let resp = self.query_raw_response(api_url, params, method).await?;
        resp.text().await.map_err(MediaWikiError::Reqwest)
    }

    /// Runs a query against a generic URL, and returns a text.
    /// Does not store cookies, but also does not require `&self` to be mutable.
    /// Used for simple queries
    pub async fn query_raw(
        &self,
        api_url: &str,
        params: &HashMap<String, String>,
        method: &str,
    ) -> Result<String, MediaWikiError> {
        let resp = self.query_raw_response(api_url, params, method).await?;
        resp.text().await.map_err(MediaWikiError::Reqwest)
    }

    /// Performs a login against the MediaWiki API.
    /// If successful, user information is stored in `User`, and in the cookie jar
    pub async fn login<S: Into<String>>(
        &mut self,
        lgname: S,
        lgpassword: S,
    ) -> Result<(), MediaWikiError> {
        let lgname: &str = &lgname.into();
        let lgpassword: &str = &lgpassword.into();
        let lgtoken = self.get_token("login").await?;
        let params = hashmap!("action".to_string()=>"login".to_string(),"lgname".to_string()=>lgname.into(),"lgpassword".to_string()=>lgpassword.into(),"lgtoken".to_string()=>lgtoken);
        let res = self.query_api_json_mut(&params, "POST").await?;
        if res["login"]["result"] == "Success" {
            self.user.set_from_login(&res["login"])?;
            self.load_current_user_info().await
        } else {
            Err(From::from("Login failed"))
        }
    }

    /// From an API result that has a list of entries with "title" and "ns" (e.g. search), returns a vector of `Title` objects.
    pub fn result_array_to_titles(data: &Value) -> Vec<Title> {
        // See if it's the "root" of the result, then try each sub-object separately
        if let Some(obj) = data.as_object() {
            obj.iter()
                .flat_map(|(_k, v)| Api::result_array_to_titles(v))
                .collect()
        } else if let Some(arr) = data.as_array() {
            arr.iter().map(Title::new_from_api_result).collect()
        } else {
            vec![]
        }
    }

    /// Performs a SPARQL query against a wikibase installation.
    /// Tries to get the SPARQL endpoint URL from the site info
    pub async fn sparql_query(&self, query: &str) -> Result<Value, MediaWikiError> {
        let query_api_url = self.get_site_info_string("general", "wikibase-sparql")?;
        let params = hashmap!["query".to_string()=>query.to_string(),"format".to_string()=>"json".to_string()];
        let response = self
            .query_raw_response(query_api_url, &params, "POST")
            .await?;
        match response.json().await {
            Ok(json) => Ok(json),
            Err(e) => Err(From::from(format!("{}", e))),
        }
    }

    /// Performs a SPARQL query against a wikibase installation.
    /// Uses the given sparql endpoint
    pub async fn sparql_query_endpoint(
        &self,
        query: &str,
        query_api_url: &str,
    ) -> Result<Value, MediaWikiError> {
        let params = hashmap!["query".to_string()=>query.to_string(),"format".to_string()=>"json".to_string()];
        let response = self
            .query_raw_response(query_api_url, &params, "POST")
            .await?;
        let bytes = match response.bytes().await {
            Ok(bytes) => bytes,
            Err(e) => {
                return Err(From::from(format!("{}", e)));
            }
        };
        match serde_json::from_slice(&bytes) {
            Ok(json) => Ok(json),
            Err(e) => {
                let bytes_start: Vec<u8> = bytes.iter().take(100).cloned().collect();
                let bytes_start = String::from_utf8_lossy(&bytes_start);
                Err(From::from(format!("{e}: {bytes_start}"))) // Error plus first 100 chars of response
            }
        }
    }

    /// Given a `uri` (usually, an URL) that points to a Wikibase entity on this MediaWiki installation, returns the item ID
    pub fn extract_entity_from_uri(&self, uri: &str) -> Result<String, MediaWikiError> {
        let concept_base_uri = self.get_site_info_string("general", "wikibase-conceptbaseuri")?;
        match uri.strip_prefix(concept_base_uri) {
            Some(s) => Ok(s.to_string()),
            None => Err(From::from(format!(
                "{} does not start with {}",
                uri, concept_base_uri
            ))),
        }
    }

    /// Returns a vector of entity IDs (as String) from a SPARQL result, given a variable name
    pub fn entities_from_sparql_result(
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

    /// Loads the user info from the API into the user structure
    pub async fn load_user_info(&self, user: &mut User) -> Result<(), MediaWikiError> {
        if !user.has_user_info() {
            let params: HashMap<String, String> = [
                ("action", "query"),
                ("meta", "userinfo"),
                ("uiprop", "blockinfo|groups|groupmemberships|implicitgroups|rights|options|ratelimits|realname|registrationdate|unreadcount|centralids|hasmsg"),
            ]
            .iter()
            .map(|x| (x.0.to_string(), x.1.to_string()))
            .collect();
            let res = self.query_api_json(&params, "GET").await?;
            user.set_user_info(Some(res));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Api, Title};
    use wiremock::matchers::query_param;
    use wiremock::{Mock, ResponseTemplate};

    #[tokio::test]
    async fn site_info() {
        let server = crate::test_helpers::test_helpers_mod::start_wikidata_mock().await;
        let api = Api::new(&server.uri()).await.unwrap();
        assert_eq!(
            api.get_site_info_string("general", "sitename").unwrap(),
            "Wikidata"
        );
        assert!(api.get_site_info_string("general", "notarealkey").is_err());
    }

    #[tokio::test]
    async fn get_token() {
        let server = crate::test_helpers::test_helpers_mod::start_wikidata_mock().await;
        Mock::given(query_param("meta", "tokens"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"tokens": {"csrftoken": "+\\"}}
            })))
            .mount(&server)
            .await;
        let mut api = Api::new(&server.uri()).await.unwrap();
        assert!(!api.user.logged_in());
        assert_eq!("+\\", api.get_token("csrf").await.unwrap());
        assert_eq!("+\\", api.get_edit_token().await.unwrap());
        assert!(api.get_token("notarealtokentype").await.is_err());
    }

    #[tokio::test]
    async fn api_limit() {
        let server = crate::test_helpers::test_helpers_mod::start_wikidata_mock().await;
        // Return exactly 20 search results, no continue
        let results: Vec<serde_json::Value> = (1..=20)
            .map(|i| json!({"ns": 0, "title": format!("Result {}", i), "pageid": i}))
            .collect();
        Mock::given(query_param("list", "search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "batchcomplete": "",
                "query": {"search": results}
            })))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let params =
            api.params_into(&[("action", "query"), ("list", "search"), ("srsearch", "the")]);
        let result = api
            .get_query_api_json_limit(&params, Some(20))
            .await
            .unwrap();
        assert_eq!(result["query"]["search"].as_array().unwrap().len(), 20);
    }

    #[tokio::test]
    async fn api_no_limit() {
        let server = crate::test_helpers::test_helpers_mod::start_wikidata_mock().await;
        let page1 = crate::test_helpers::test_helpers_mod::load_test_data("search_page1.json");
        let page2 = crate::test_helpers::test_helpers_mod::load_test_data("search_page2.json");
        let page3 = crate::test_helpers::test_helpers_mod::load_test_data("search_page3.json");
        Mock::given(query_param("list", "search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(page1))
            .up_to_n_times(1)
            .mount(&server)
            .await;
        Mock::given(query_param("list", "search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(page2))
            .up_to_n_times(1)
            .mount(&server)
            .await;
        Mock::given(query_param("list", "search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(page3))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let params = api.params_into(&[
            ("action", "query"),
            ("list", "search"),
            ("srlimit", "500"),
            ("srsearch", "John"),
        ]);
        let result = api.get_query_api_json_all(&params).await.unwrap();
        match result["query"]["search"].as_array() {
            Some(arr) => assert!(arr.len() > 10),
            None => panic!("result.query.search is not an array"),
        }
    }

    #[tokio::test]
    async fn sparql_query() {
        let server = crate::test_helpers::test_helpers_mod::start_wikidata_mock().await;
        let sparql_results =
            crate::test_helpers::test_helpers_mod::load_test_data("sparql_results.json");
        Mock::given(wiremock::matchers::path("/sparql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sparql_results))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let res = api
            .sparql_query("SELECT ?q ?qLabel ?fellow_id { ?q wdt:P31 wd:Q5 . }")
            .await
            .unwrap();
        assert!(!res["results"]["bindings"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn entities_from_sparql_result() {
        let server = crate::test_helpers::test_helpers_mod::start_wikidata_mock().await;
        let sparql_results =
            crate::test_helpers::test_helpers_mod::load_test_data("sparql_results.json");
        Mock::given(wiremock::matchers::path("/sparql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sparql_results))
            .mount(&server)
            .await;
        let api = Api::new(&server.uri()).await.unwrap();
        let res = api
            .sparql_query("SELECT ?q ?qLabel ?fellow_id { ?q wdt:P31 wd:Q5 . }")
            .await
            .unwrap();
        let titles = api.entities_from_sparql_result(&res, "q");
        assert!(titles.contains(&"Q36499535".to_string()));
    }

    #[tokio::test]
    async fn extract_entity_from_uri() {
        let server = crate::test_helpers::test_helpers_mod::start_wikidata_mock().await;
        let api = Api::new(&server.uri()).await.unwrap();
        assert_eq!(
            api.extract_entity_from_uri("http://www.wikidata.org/entity/Q123")
                .unwrap(),
            "Q123"
        );
        assert_eq!(
            api.extract_entity_from_uri("http://www.wikidata.org/entity/P456")
                .unwrap(),
            "P456"
        );
        assert!(
            api.extract_entity_from_uri("http:/www.wikidata.org/entity/Q123")
                .is_err()
        );
    }

    #[tokio::test]
    async fn result_array_to_titles() {
        assert_eq!(
            Api::result_array_to_titles(
                &json!({"something":[{"title":"Foo","ns":7},{"title":"Bar","ns":8},{"title":"Prefix:Baz","ns":9}]})
            ),
            vec![
                Title::new("Foo", 7),
                Title::new("Bar", 8),
                Title::new("Baz", 9)
            ]
        );
    }

    #[tokio::test]
    async fn result_namespaces() {
        let server = crate::test_helpers::test_helpers_mod::start_dewiki_mock().await;
        let api = Api::new(&server.uri()).await.unwrap();
        assert_eq!(api.get_local_namespace_name(0), Some(""));
        assert_eq!(api.get_local_namespace_name(1), Some("Diskussion"));
        assert_eq!(api.get_canonical_namespace_name(1), Some("Talk"));
    }

    // --- Pure unit tests below (no HTTP calls) ---

    #[test]
    fn json_merge_objects() {
        let mut a = json!({"key1": "val1"});
        Api::json_merge(&mut a, json!({"key2": "val2"}));
        assert_eq!(a, json!({"key1": "val1", "key2": "val2"}));
    }

    #[test]
    fn json_merge_objects_overwrite() {
        let mut a = json!({"key": "old"});
        Api::json_merge(&mut a, json!({"key": "new"}));
        assert_eq!(a, json!({"key": "new"}));
    }

    #[test]
    fn json_merge_arrays() {
        let mut a = json!([1, 2]);
        Api::json_merge(&mut a, json!([3, 4]));
        assert_eq!(a, json!([1, 2, 3, 4]));
    }

    #[test]
    fn json_merge_nested() {
        let mut a = json!({"query": {"pages": [{"id": 1}]}});
        Api::json_merge(&mut a, json!({"query": {"pages": [{"id": 2}]}}));
        assert_eq!(a, json!({"query": {"pages": [{"id": 1}, {"id": 2}]}}));
    }

    #[test]
    fn json_merge_scalar_overwrite() {
        let mut a = json!("old");
        Api::json_merge(&mut a, json!("new"));
        assert_eq!(a, json!("new"));
    }

    #[test]
    fn json_merge_null_base() {
        let mut a = json!(null);
        Api::json_merge(&mut a, json!({"key": "val"}));
        assert_eq!(a, json!({"key": "val"}));
    }

    #[test]
    fn is_edit_query_post_with_token() {
        let api = Api {
            api_url: String::new(),
            site_info: json!({}),
            client: reqwest::Client::new(),
            user: crate::user::User::new(),
            user_agent: String::new(),
            maxlag_seconds: None,
            edit_delay_ms: None,
            max_retry_attempts: 5,
            oauth: None,
            oauth2: None,
        };
        let mut params = std::collections::HashMap::new();
        params.insert("token".to_string(), "abc".to_string());
        assert!(api.is_edit_query(&params, "POST"));
        assert!(!api.is_edit_query(&params, "GET"));

        let no_token = std::collections::HashMap::new();
        assert!(!api.is_edit_query(&no_token, "POST"));
    }

    #[test]
    fn check_maxlag_detects_lag() {
        let api = Api {
            api_url: String::new(),
            site_info: json!({}),
            client: reqwest::Client::new(),
            user: crate::user::User::new(),
            user_agent: String::new(),
            maxlag_seconds: Some(5),
            edit_delay_ms: None,
            max_retry_attempts: 5,
            oauth: None,
            oauth2: None,
        };
        let v = json!({"error": {"code": "maxlag", "lag": 10}});
        assert_eq!(api.check_maxlag(&v), Some(10));

        let v_no_lag = json!({"error": {"code": "maxlag"}});
        assert_eq!(api.check_maxlag(&v_no_lag), Some(5)); // falls back to maxlag_seconds

        let v_ok = json!({"query": {}});
        assert_eq!(api.check_maxlag(&v_ok), None);
    }

    #[test]
    fn user_agent_full_format() {
        let api = Api {
            api_url: String::new(),
            site_info: json!({}),
            client: reqwest::Client::new(),
            user: crate::user::User::new(),
            user_agent: "TestBot".to_string(),
            maxlag_seconds: None,
            edit_delay_ms: None,
            max_retry_attempts: 5,
            oauth: None,
            oauth2: None,
        };
        let ua = api.user_agent_full();
        assert!(ua.starts_with("TestBot; "));
        assert!(ua.contains("mediawiki-rust/"));
    }

    #[test]
    fn params_into_converts_correctly() {
        let api = Api {
            api_url: String::new(),
            site_info: json!({}),
            client: reqwest::Client::new(),
            user: crate::user::User::new(),
            user_agent: String::new(),
            maxlag_seconds: None,
            edit_delay_ms: None,
            max_retry_attempts: 5,
            oauth: None,
            oauth2: None,
        };
        let params = api.params_into(&[("action", "query"), ("meta", "siteinfo")]);
        assert_eq!(params.len(), 2);
        assert_eq!(params["action"], "query");
        assert_eq!(params["meta"], "siteinfo");
    }

    #[test]
    fn no_params_is_empty() {
        let api = Api {
            api_url: String::new(),
            site_info: json!({}),
            client: reqwest::Client::new(),
            user: crate::user::User::new(),
            user_agent: String::new(),
            maxlag_seconds: None,
            edit_delay_ms: None,
            max_retry_attempts: 5,
            oauth: None,
            oauth2: None,
        };
        assert!(api.no_params().is_empty());
    }

    #[test]
    fn result_array_to_titles_empty_object() {
        assert!(Api::result_array_to_titles(&json!({})).is_empty());
    }

    #[test]
    fn result_array_to_titles_non_array_non_object() {
        assert!(Api::result_array_to_titles(&json!(42)).is_empty());
        assert!(Api::result_array_to_titles(&json!(null)).is_empty());
    }
}
