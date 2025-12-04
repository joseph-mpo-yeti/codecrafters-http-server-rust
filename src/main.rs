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

    router.get("/echo/{str}", |req| {
        let str = req.path_params.get("str").unwrap().trim().to_string();
        HttpResponse::builder()
            .status_code(StatusCode::Ok)
            .body(str)
            .header("Content-Type", "text/plain")
            .build()
    });

    router.get("/echo/{name}/{age}", |req| {
        let name = req.path_params.get("name").unwrap().clone();
        let age = req.path_params.get("age").unwrap().clone();
        let mut body = String::from("{\"name\": \"");
        body.push_str(format!("{}", name).as_str());
        body.push_str("\", \"age\":\"");
        body.push_str(format!("{}", age).as_str());
        body.push_str("\"}");

        HttpResponse::builder()
            .status_code(StatusCode::Ok)
            .body(body)
            .header("Content-Type", "text/plain")
            .build()
    });

    // dbg!(&router);

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
