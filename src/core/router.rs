use std::collections::HashMap;

use crate::types::method::*;
use crate::types::request::*;
use crate::types::response::*;

#[derive(Debug)]
pub struct HttpRouter {
    routes: HashMap<String, Route>,
}

impl HttpRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: fn(HttpRequest) -> HttpResponse) {
        self.register(HttpRequestMethod::GET, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: fn(HttpRequest) -> HttpResponse) {
        self.register(HttpRequestMethod::POST, path, handler);
    }

    pub fn patch(&mut self, path: &str, handler: fn(HttpRequest) -> HttpResponse) {
        self.register(HttpRequestMethod::PATCH, path, handler);
    }

    pub fn put(&mut self, path: &str, handler: fn(HttpRequest) -> HttpResponse) {
        self.register(HttpRequestMethod::PUT, path, handler);
    }

    pub fn options(&mut self, path: &str, handler: fn(HttpRequest) -> HttpResponse) {
        self.register(HttpRequestMethod::OPTIONS, path, handler);
    }

    pub fn delete(&mut self, path: &str, handler: fn(HttpRequest) -> HttpResponse) {
        self.register(HttpRequestMethod::DELETE, path, handler);
    }

    fn register(
        &mut self,
        method: HttpRequestMethod,
        path: &str,
        handler: fn(HttpRequest) -> HttpResponse,
    ) {
        let key = String::from(path.trim());
        match self.routes.get_mut(&key) {
            Some(route) => {
                route.handlers.insert(method, handler);
            }
            None => {
                let route = Route::new(method, handler);
                self.routes.insert(key, route);
            }
        }
    }

    pub fn get_handler(&self, req: &HttpRequest) -> Option<&fn(HttpRequest) -> HttpResponse> {
        let route = self.routes.get(&req.target)?;
        route.handlers.get(&req.method)
    }
}

#[derive(Debug)]
struct Route {
    handlers: HashMap<HttpRequestMethod, fn(HttpRequest) -> HttpResponse>,
}

impl Route {
    pub fn new(method: HttpRequestMethod, handler: fn(HttpRequest) -> HttpResponse) -> Self {
        let mut handlers = HashMap::new();
        handlers.insert(method, handler);
        Self { handlers }
    }
}
