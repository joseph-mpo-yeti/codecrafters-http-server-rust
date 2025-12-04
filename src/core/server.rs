use std::{fmt::Debug, sync::Arc};

use super::{handler::HttpRequestHandler, logging::Logging, router::HttpRouter};

use std::net::TcpListener;

#[derive(Debug)]
pub struct HttpServer {
    logging_enabled: bool,
    router: Arc<HttpRouter>,
}

impl HttpServer {
    pub fn new(router: HttpRouter) -> Self {
        Self {
            logging_enabled: false,
            router: Arc::new(router),
        }
    }

    pub fn listen(&self, port: u32) {
        let listen = TcpListener::bind(format!("127.0.0.1:{}", port))
            .unwrap();

        loop {
            match listen.accept() {
                Ok((socket, _)) => {
                    let s_router = self.router.clone();
                    let mut handler = HttpRequestHandler::new(s_router);
                    // pin!(socket);
                    if self.logging_enabled() {
                        handler.enable_logging();
                    }
                    std::thread::spawn(move || {
                        let _ = handler.handle_incoming_request(socket);
                    });
                }
                Err(err) => {
                    if self.logging_enabled() {
                        println!("Request not processed! There was an error. Error: {}", err);
                    }
                }
            }
        }
    }
}

impl Logging for HttpServer {
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
