#[cfg(test)]
pub mod test_helpers_mod {
    use serde_json::{Value, json};
    use wiremock::matchers::query_param;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    pub fn enwiki_siteinfo() -> Value {
        json!({
            "batchcomplete": "",
            "query": {
                "general": {
                    "sitename": "Wikipedia",
                    "dbname": "enwiki",
                    "server": "https://en.wikipedia.org",
                    "case": "first-letter",
                    "lang": "en"
                },
                "namespaces": {
                    "0":  {"id": 0,  "case": "first-letter", "content": "", "*": ""},
                    "1":  {"id": 1,  "case": "first-letter", "*": "Talk", "canonical": "Talk"},
                    "2":  {"id": 2,  "case": "first-letter", "*": "User", "canonical": "User"},
                    "3":  {"id": 3,  "case": "first-letter", "*": "User talk", "canonical": "User talk"},
                    "4":  {"id": 4,  "case": "first-letter", "*": "Wikipedia", "canonical": "Project"},
                    "5":  {"id": 5,  "case": "first-letter", "*": "Wikipedia talk", "canonical": "Project talk"},
                    "6":  {"id": 6,  "case": "first-letter", "*": "File", "canonical": "File"},
                    "7":  {"id": 7,  "case": "first-letter", "*": "File talk", "canonical": "File talk"},
                    "8":  {"id": 8,  "case": "first-letter", "*": "MediaWiki", "canonical": "MediaWiki"},
                    "10": {"id": 10, "case": "first-letter", "*": "Template", "canonical": "Template"},
                    "14": {"id": 14, "case": "first-letter", "*": "Category", "canonical": "Category"},
                    "-1": {"id": -1, "case": "first-letter", "*": "Special", "canonical": "Special"},
                    "-2": {"id": -2, "case": "first-letter", "*": "Media", "canonical": "Media"}
                },
                "namespacealiases": [],
                "libraries": [],
                "extensions": [],
                "statistics": {"pages": 7000000, "articles": 6700000, "edits": 1000000000_i64}
            }
        })
    }

    pub fn wikidata_siteinfo(sparql_url: &str) -> Value {
        json!({
            "batchcomplete": "",
            "query": {
                "general": {
                    "sitename": "Wikidata",
                    "dbname": "wikidatawiki",
                    "wikibase-conceptbaseuri": "http://www.wikidata.org/entity/",
                    "wikibase-sparql": sparql_url
                },
                "namespaces": {
                    "0":  {"id": 0,  "case": "first-letter", "content": "", "*": ""},
                    "1":  {"id": 1,  "case": "first-letter", "*": "Talk", "canonical": "Talk"},
                    "2":  {"id": 2,  "case": "first-letter", "*": "User", "canonical": "User"},
                    "3":  {"id": 3,  "case": "first-letter", "*": "User talk", "canonical": "User talk"},
                    "4":  {"id": 4,  "case": "first-letter", "*": "Wikidata", "canonical": "Project"},
                    "5":  {"id": 5,  "case": "first-letter", "*": "Wikidata talk", "canonical": "Project talk"},
                    "6":  {"id": 6,  "case": "first-letter", "*": "File", "canonical": "File"},
                    "7":  {"id": 7,  "case": "first-letter", "*": "File talk", "canonical": "File talk"},
                    "10": {"id": 10, "case": "first-letter", "*": "Template", "canonical": "Template"},
                    "14": {"id": 14, "case": "first-letter", "*": "Category", "canonical": "Category"},
                    "-1": {"id": -1, "case": "first-letter", "*": "Special", "canonical": "Special"}
                },
                "namespacealiases": [
                    {"id": 0,   "*": "Item"},
                    {"id": 120, "*": "Property"}
                ],
                "libraries": [],
                "extensions": [],
                "statistics": {}
            }
        })
    }

    pub fn dewiki_siteinfo() -> Value {
        json!({
            "batchcomplete": "",
            "query": {
                "general": {
                    "sitename": "Wikipedia",
                    "dbname": "dewiki"
                },
                "namespaces": {
                    "0":  {"id": 0,  "case": "first-letter", "content": "", "*": ""},
                    "1":  {"id": 1,  "case": "first-letter", "*": "Diskussion", "canonical": "Talk"},
                    "2":  {"id": 2,  "case": "first-letter", "*": "Benutzer", "canonical": "User"},
                    "-1": {"id": -1, "case": "first-letter", "*": "Spezial", "canonical": "Special"}
                },
                "namespacealiases": [],
                "libraries": [],
                "extensions": [],
                "statistics": {}
            }
        })
    }

    /// Start a mock server that responds to siteinfo requests with English Wikipedia data.
    pub async fn start_enwiki_mock() -> MockServer {
        let server = MockServer::start().await;
        Mock::given(query_param("meta", "siteinfo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(enwiki_siteinfo()))
            .mount(&server)
            .await;
        server
    }

    /// Start a mock server with Wikidata siteinfo. SPARQL endpoint is at `{server_uri}/sparql`.
    pub async fn start_wikidata_mock() -> MockServer {
        let server = MockServer::start().await;
        let sparql_url = format!("{}/sparql", server.uri());
        Mock::given(query_param("meta", "siteinfo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(wikidata_siteinfo(&sparql_url)))
            .mount(&server)
            .await;
        server
    }

    /// Start a mock server with German Wikipedia siteinfo.
    pub async fn start_dewiki_mock() -> MockServer {
        let server = MockServer::start().await;
        Mock::given(query_param("meta", "siteinfo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(dewiki_siteinfo()))
            .mount(&server)
            .await;
        server
    }

    /// Load test data JSON from the `test_data/` directory.
    pub fn load_test_data(filename: &str) -> Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join(filename);
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));
        serde_json::from_str(&content)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {}", path.display(), e))
    }
}
