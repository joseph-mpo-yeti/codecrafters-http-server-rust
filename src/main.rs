pub mod core;
pub mod types;

use crate::{
    core::{router::HttpRouter, server::HttpServer, logging::Logging}, 
    types::{response::HttpResponse, status::StatusCode}
};

fn main() {
    let mut router = HttpRouter::new();
    router.get("/", |_req| {
        HttpResponse::builder()
            .status_code(StatusCode::Ok)
            .build()
    });

    // router.post("/api/v1", |_req| {
    //     HttpResponse::builder()
    //         .status_code(StatusCode::Ok)
    //         .body("{\"message\":\"Hello World!\"}".to_string())
    //         .header("Content-Type", "application/json")
    //         .build()
    // });

    let mut server = HttpServer::new(router);
    server.enable_logging();
    server.listen(4221);
}
