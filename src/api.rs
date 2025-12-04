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
        .plain_text(user_agent.to_string())
        .build()
}


pub fn get_str(req: HttpRequest, _ctx: &Context) -> HttpResponse {
    let str = req.path_params.get("str").unwrap().trim().to_string();
    HttpResponse::builder()
        .status_code(StatusCode::Ok)
        .plain_text(str)
        .build()
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