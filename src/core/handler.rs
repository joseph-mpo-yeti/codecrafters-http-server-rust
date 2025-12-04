use super::logging::Logging;
use super::parser::Parser;
use super::router::HttpRouter;
use crate::core::server::Context;
use crate::types::response::HttpResponse;

use std::collections::HashSet;
use std::io::Write;
use std::{io::Error, sync::Arc};
use std::net::{TcpStream};

pub struct HttpRequestHandler {
    logging_enabled: bool,
    router: Arc<HttpRouter>,
    enconding_schemes: HashSet<String>
}

impl HttpRequestHandler {
    pub fn new(router: Arc<HttpRouter>) -> Self {
        Self {
            logging_enabled: false,
            router: router,
            enconding_schemes: HashSet::from([String::from("gzip")])
        }
    }

    pub fn handle_incoming_request(&self, mut socket: TcpStream, ctx: &Context) -> Result<(), Error> {
        let start = std::time::Instant::now();
        let parser = Parser::new();
        let parse_result = parser.parse_http_request(&mut socket, self.router.clone());

        let request = match parse_result {
            Ok(request) => request,
            Err(_) => {
                // println!("{}", err);
                let response = HttpResponse::builder()
                    .status_code(crate::types::status::StatusCode::BadRequest)
                    .build();
                self.write_and_close(socket, &response).unwrap_or_default();
                // println!("-- Bad Request");
                return Ok(());
            }
        };

        if self.logging_enabled() {
            // println!(
            //     "-- {} {:?} {}",
            //     request.version,
            //     request.method,
            //     request.target
            // );
            // println!("-- {} {request:?}", request.version);
        }

        let router = self.router.as_ref();

        if self.logging_enabled() {
            // println!("{router:?}");
        }
        let r = request.clone();
        let response = match router.get_handler(&r) {
            Some(handler) => {
                let encoding_schemes = if let Some(scheme) = r.headers.get("Accept-Encoding"){
                    scheme.split(",").collect()
                } else {
                    Vec::new()
                };

                let schemes: Vec<String> = encoding_schemes.iter()
                    .map(|m|m.trim().to_string())
                    .filter(|scheme| self.enconding_schemes.contains(scheme))
                    .collect();

                let mut res = handler(r, ctx);

                if !schemes.is_empty() {
                    let scheme = schemes.get(0).unwrap();
                    res.headers.insert("Content-Encoding".to_string(), scheme.to_owned());
                }

                res
            },
            _ => HttpResponse::builder()
                    .status_code(crate::types::status::StatusCode::NotFound)
                    .build(),
        };

        self.write_and_close(socket, &response).unwrap();

        let elapsed = start.elapsed();
        let _ = if elapsed.as_secs() > 0 {
            format!(" -- {}s", elapsed.as_secs_f32())
        } else {
            format!(" -- {}ms", elapsed.as_millis())
        };

        // println!(
        //     "-- {} {:?} {} {} {}",
        //     request.version,
        //     request.method,
        //     response.status_code,
        //     request.target,
        //     elapsed_time_st
        // );

        Ok(())
    }

    pub fn write_and_close(
        &self,
        mut socket: TcpStream,
        response: &HttpResponse,
    ) -> Result<(), Error> {
        let http_response = self.get_response_str(&response);

        if self.logging_enabled() {
            // println!("Response: {:?}", http_response);
        }

        socket.write_all(http_response.as_bytes())?;
        socket.flush()?;
        socket.shutdown(std::net::Shutdown::Both)?;

        Ok(())
    }

    pub fn write(
        &self,
        socket: &mut TcpStream,
        response: &HttpResponse,
    ) -> Result<(), Error> {
        let http_response = self.get_response_str(&response);

        if self.logging_enabled() {
            // println!("Response: {:?}", http_response);
        }

        socket.write_all(http_response.as_bytes())?;
        socket.flush()?;

        Ok(())
    }

    fn get_response_str(&self, response: &HttpResponse) -> String {
        let mut http_response = String::from(format!(
            "{} {} {}\r\n",
            response.protocol, response.status_code, response.reason
        ));
        for (key, value) in &response.headers {
            http_response.push_str(format!("{}: {}\r\n", key, value).as_str());
        }

        if response.body.len() > 0 {
            http_response
                .push_str(format!("Content-Length: {}\r\n", response.body.len()).as_str());
        }

        // http_response.push_str("Connection: close\r\n\r\n");
        http_response.push_str("\r\n");

        if response.body.len() > 0 {
            http_response.push_str(format!("{}", response.body).as_str());
        }

        http_response
    }

    pub fn add_encoding_scheme(&mut self, scheme: &str) {
        self.enconding_schemes.insert(scheme.to_string());
    }

}

impl Logging for HttpRequestHandler {
    fn enable_logging(&mut self) {
        self.logging_enabled = true;
    }
    fn disable_logging(&mut self) {
        self.logging_enabled = true;
    }

    fn logging_enabled(&self) -> bool {
        self.logging_enabled
    }
}
