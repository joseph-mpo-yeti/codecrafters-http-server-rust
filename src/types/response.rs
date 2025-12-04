use std::{collections::HashMap, error::Error, fmt::Display};

use super::status::StatusCode;

#[derive(Debug)]
pub struct HttpResponse {
    pub protocol: String,
    pub status_code: u32,
    pub body: String,
    pub reason: String,
    // pubquery: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct HttpResponseBuilder {
    protocol: String,
    status_code: Option<StatusCode>,
    body: String,
    headers: HashMap<String, String>
}

impl HttpResponseBuilder {
    fn new() -> Self {
        Self {
            protocol: "HTTP/1.1".to_string(),
            status_code: None,
            body: String::new(),
            headers: HashMap::new()
        }
    }

    pub fn status_code(&mut self, status: StatusCode) -> &mut Self {
        self.status_code = Some(status);
        self
    }

    pub fn protocol(&mut self, protocol: String) -> &mut Self {
        self.protocol = protocol;
        self
    }

    pub fn file(&mut self, content: Vec<u8>) -> &mut Self {
        self.body = String::from_utf8(content).unwrap_or(String::new());
        self.header("Content-Type", "application/octet-stream")
    }

    pub fn body(&mut self, body: String) -> &mut Self {
        self.body = body;
        self
    }

    pub fn json(&mut self, body: String) -> &mut Self {
        self.body = body;
        self.header("Content-Type", "application/json")
    }

    pub fn plain_text(&mut self, body: String) -> &mut Self {
        self.body = body;
        self.header("Content-Type", "text/plain")
    }

    pub fn header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn get(&self) -> Self {
        self.clone()
    }

    pub fn build(&mut self) -> HttpResponse {
        let status = match self.status_code {
            Some(st) => st,
            None => StatusCode::InternalServerError,
        };

        match status {
            StatusCode::InternalServerError => {
                self.body = "".to_string();
                self.headers.clear();
            }
            _ => {}
        }

        HttpResponse {
            protocol: self.protocol.to_string(),
            status_code: status as u32,
            body: self.body.to_string(),
            reason: status.reason().to_string(),
            headers: self.headers.clone(),
        }
    }
}

#[derive(Debug)]
pub struct HttpResponseBuilderError {
    message: String,
}

impl Display for HttpResponseBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot build HttpResponse: {}", self.message)
    }
}

impl Error for HttpResponseBuilderError {}

impl HttpResponse {
    pub fn builder() -> HttpResponseBuilder {
        HttpResponseBuilder::new()
    }
}
