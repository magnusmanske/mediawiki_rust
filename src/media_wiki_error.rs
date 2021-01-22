use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MediaWikiError {
    Serde(serde_json::Error),
    Reqwest(reqwest::Error),
    ReqwestHeader(reqwest::header::InvalidHeaderValue),
    String(String),
    Url(url::ParseError),
    Fmt(std::fmt::Error),
    Time(std::time::SystemTimeError)
}

impl Error for MediaWikiError {}
unsafe impl Send for MediaWikiError {}

impl fmt::Display for MediaWikiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Serde(e) => f.write_str(&e.to_string()),
            Self::Reqwest(e) => f.write_str(&e.to_string()),
            Self::ReqwestHeader(e) => f.write_str(&e.to_string()),
            Self::String(s) => f.write_str(s),
            Self::Url(e) => f.write_str(&e.to_string()),
            Self::Fmt(e) => f.write_str(&e.to_string()),
            Self::Time(e) => f.write_str(&e.to_string()),
        }
    }
}

impl From<serde_json::Error> for MediaWikiError {  
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}

impl From<reqwest::Error> for MediaWikiError {  
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for MediaWikiError {  
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        Self::ReqwestHeader(e)
    }
}

impl From<reqwest::header::ToStrError> for MediaWikiError {  
    fn from(e: reqwest::header::ToStrError) -> Self {
        Self::String(e.to_string())
    }
}

impl From<String> for MediaWikiError {  
    fn from(e: String) -> Self {
        Self::String(e)
    }
}

impl From<&str> for MediaWikiError {  
    fn from(e: &str) -> Self {
        Self::String(e.to_string())
    }
}

impl From<url::ParseError> for MediaWikiError {  
    fn from(e: url::ParseError) -> Self {
        Self::Url(e)
    }
}

impl From<std::fmt::Error> for MediaWikiError {  
    fn from(e: std::fmt::Error) -> Self {
        Self::Fmt(e)
    }
}

impl From<std::time::SystemTimeError> for MediaWikiError {  
    fn from(e: std::time::SystemTimeError) -> Self {
        Self::Time(e)
    }
}

