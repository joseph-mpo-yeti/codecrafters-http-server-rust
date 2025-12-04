use std::{fs, path::MAIN_SEPARATOR_STR};

use crate::{core::server::Context, types::{request::HttpRequest, response::HttpResponse, status::StatusCode}};

pub fn index(_req: HttpRequest, _ctx: &Context) -> HttpResponse {
    HttpResponse::builder()
        .status_code(StatusCode::Ok)
        .build()
}

pub fn user_agent(req: HttpRequest, _ctx: &Context) -> HttpResponse {
    let user_agent = req.headers.get("User-Agent").expect("No user agent header found");
    HttpResponse::builder()
        .status_code(StatusCode::Ok)
        .body(user_agent.to_string())
        .header("Content-Type", "text/plain")
        .build()
}


pub fn get_str(req: HttpRequest, _ctx: &Context) -> HttpResponse {
    let str = req.path_params.get("str").unwrap().trim().to_string();
    let encoding_scheme = if let Some(scheme) = req.headers.get("Accept-Encoding"){
        scheme.clone()
    } else {
        String::new()
    };

    let mut response_builder = HttpResponse::builder()
        .status_code(StatusCode::Ok)
        .body(str)
        .header("Content-Type", "text/plain")
        .to_owned();
        
    if !encoding_scheme.is_empty() && encoding_scheme.eq("gzip") {
        response_builder.header("Content-Encoding", &encoding_scheme);
    }
        
    response_builder.build()
}

pub fn get_file(req: HttpRequest, ctx: &Context) -> HttpResponse {
    let filename = req.path_params.get("filename").unwrap().trim();
    println!("filename: {}", filename);
    let filepath = ctx.workdir.to_string() + MAIN_SEPARATOR_STR + filename;
    if let Ok(content) = fs::read(filepath.clone()) {
        println!("succesfully read file: {}", &filepath);
        HttpResponse::builder()
            .status_code(StatusCode::Ok)
            .file(content)
            .header("Content-Type", "application/octet-stream")
            .build()
    } else {
        println!("Failed to read file: {}", filepath);
        HttpResponse::builder()
            .status_code(StatusCode::NotFound)
            .build()
    }
    
}


pub fn create_file(req: HttpRequest, ctx: &Context) -> HttpResponse {
    let filename = req.path_params.get("filename").unwrap().trim();
    println!("filename: {}", filename);
    let filepath = ctx.workdir.to_string() + MAIN_SEPARATOR_STR + filename;
    if let Ok(()) = fs::write(filepath.clone(), req.body.as_bytes()) {
        println!("succesfully read file: {}", &filepath);
        HttpResponse::builder()
            .status_code(StatusCode::Created)
            .header("Content-Type", "application/octet-stream")
            .build()
    } else {
        println!("Failed to read file: {}", filepath);
        HttpResponse::builder()
            .status_code(StatusCode::NotFound)
            .build()
    }
    
}