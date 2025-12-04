use std::collections::HashMap;
use regex::Regex;

use crate::core::server::Context;
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

    pub fn get(&mut self, path: &str, handler: fn(HttpRequest, &Context) -> HttpResponse) {
        self.register(HttpRequestMethod::GET, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: fn(HttpRequest, &Context) -> HttpResponse) {
        self.register(HttpRequestMethod::POST, path, handler);
    }

    pub fn patch(&mut self, path: &str, handler: fn(HttpRequest, &Context) -> HttpResponse) {
        self.register(HttpRequestMethod::PATCH, path, handler);
    }

    pub fn put(&mut self, path: &str, handler: fn(HttpRequest, &Context) -> HttpResponse) {
        self.register(HttpRequestMethod::PUT, path, handler);
    }

    pub fn options(&mut self, path: &str, handler: fn(HttpRequest, &Context) -> HttpResponse) {
        self.register(HttpRequestMethod::OPTIONS, path, handler);
    }

    pub fn delete(&mut self, path: &str, handler: fn(HttpRequest, &Context) -> HttpResponse) {
        self.register(HttpRequestMethod::DELETE, path, handler);
    }

    fn register(
        &mut self,
        method: HttpRequestMethod,
        path: &str,
        handler: fn(HttpRequest, &Context) -> HttpResponse
    ) {
        let key = String::from(path.trim());
        let mut path_params = Vec::new();

        let parts: Vec<(usize, String)> =  key.split("/")
            .enumerate()
            .map(|(index, str)| (index, str.to_string()))
            .filter(|(_, str)| !str.is_empty())
            .collect();

        let mut path_regex = String::from("");
        if !parts.is_empty() {
            let mut has_params = false;
            for param in &parts { 
                if param.1.starts_with("{") && param.1.ends_with("}") {
                    has_params = true;
                    break;
                }
            }
            if has_params {
                for param in parts {
                    path_regex.push_str("/");
                    if param.1.starts_with("{") && param.1.ends_with("}") {
                        path_params.push((param.0, param.1.strip_prefix("{").unwrap().strip_suffix("}").unwrap().to_string()));
                        path_regex.push_str(r"([a-zA-Z_\-0-9\.]+)");
                    } else {
                        path_regex.push_str(param.1.as_str());
                    }
                }
                path_regex.insert(0, '^');
                path_regex.push_str("$");
            }
        }
        
        // dbg!(&path_params);
        // dbg!(&path_regex);
        if path_regex.is_empty() {
            match self.routes.get_mut(&key) {
                Some(route) => {
                    route.handlers.insert(method, handler);
                    route.path_params = path_params;
                }
                None => {
                    let mut route = Route::new(method, handler);
                    route.path_params = path_params;
                    self.routes.insert( key, route);
                }
            }
        } else {
            match self.routes.get_mut(&path_regex) {
                Some(route) => {
                    route.handlers.insert(method, handler);
                    route.path_params = path_params;
                }
                None => {
                    let mut route = Route::new(method, handler);
                    route.path_params = path_params;
                    self.routes.insert( path_regex, route);
                }
            }
        }
        
    }

    pub fn get_handler(&self, req: &HttpRequest) -> Option<&fn(HttpRequest, &Context) -> HttpResponse> {
        match self.routes.get(&req.target) {
            Some(route) => {
                route.handlers.get(&req.method)
            },
            None => {
                for (path, route) in &self.routes {
                    if route.path_params.is_empty() {
                        continue;
                    }
                    let reg = Regex::new(path.as_str()).unwrap();
                    if reg.is_match(&req.target) {
                        // println!("found a match for {} : {}", req.target, path);
                        return route.handlers.get(&req.method);
                    }
                }

                None
            }
        }
    }

    pub fn get_routes(&self) -> &HashMap<String, Route> {
        &self.routes
    }
}

#[derive(Debug)]
pub struct Route {
    handlers: HashMap<HttpRequestMethod, fn(HttpRequest, &Context) -> HttpResponse>,
    path_params: Vec<(usize, String)>,
}

impl Route {
    pub fn new(method: HttpRequestMethod, handler: fn(HttpRequest, &Context) -> HttpResponse) -> Self {
        let mut handlers = HashMap::new();
        handlers.insert(method, handler);
        Self { 
            handlers,
            path_params: Vec::new(),
        }
    }

    pub fn get_path_params(&self) -> &Vec<(usize, String)> {
        &self.path_params
    }
}
