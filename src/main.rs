pub mod api;
pub mod core;
pub mod types;

use std::env;

use crate::core::{
    logging::Logging,
    router::HttpRouter,
    server::{Context, HttpServer},
};

#[tokio::main]
async fn main() {
    let mut workdir = String::new();
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();

    if args.len() > 2 && args.get(1).unwrap().eq("--directory") {
        workdir = args.get(2).unwrap().to_string();
    }

    let mut router = HttpRouter::new();

    router.get("/", api::index);
    router.get("/user-agent", api::user_agent);
    router.get("/echo/{str}", api::get_str);
    router.get("/files/{filename}", api::get_file);
    router.post("/files/{filename}", api::create_file);

    let mut server = HttpServer::new(router);
    server.set_context(Context { workdir });
    server.enable_logging();

    // dbg!(&server);
    server.listen(4221).await;
}
