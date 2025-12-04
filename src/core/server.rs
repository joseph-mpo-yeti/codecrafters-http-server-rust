use std::{fmt::Debug, sync::Arc};

use super::{handler::HttpRequestHandler, logging::Logging, router::HttpRouter};

use tokio::net::TcpListener;

#[derive(Debug, Default, Clone)]
pub struct Context {
    pub workdir: String,
}

#[derive(Debug)]
pub struct HttpServer {
    logging_enabled: bool,
    router: Arc<HttpRouter>,
    context: Context,
}

impl HttpServer {
    pub fn new(router: HttpRouter) -> Self {
        Self {
            logging_enabled: false,
            router: Arc::new(router),
            context: Context::default(),
        }
    }

    pub async fn listen(&self, port: u32) {
        let listen = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

        loop {
            match listen.accept().await {
                Ok((socket, _)) => {
                    let s_router = self.router.clone();
                    let mut handler = HttpRequestHandler::new(s_router);
                    // pin!(socket);
                    if self.logging_enabled() {
                        handler.enable_logging();
                    }
                    let ctx = self.context.clone();
                    tokio::spawn(async move {
                        let _ = handler.handle_incoming_request(socket, &ctx).await;
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

    pub fn set_context(&mut self, ctx: Context) {
        self.context = ctx;
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
