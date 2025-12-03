use super::method::HttpRequestMethod;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpRequestMethod,
    pub target: String,
    pub version: String,
    pub body: String,
    // pubquery: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}

pub struct RequestLine(pub HttpRequestMethod, pub String, pub String);

impl HttpRequest {
    pub fn new(
        method: HttpRequestMethod,
        target: String,
        version: String,
        body: String,
        // query: HashMap<String, String>,
        headers: HashMap<String, String>,
    ) -> Self {
        Self {
            method,
            target,
            version,
            headers,
            body,
        }
    }
}
