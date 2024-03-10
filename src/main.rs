// ToDoKiosk - CTCL 2023-2024
// File: src/main.rs
// Purpose: Main code
// Created: March 10, 2024
// Modified: March 10, 2024

use std::collections::HashMap;
use actix_files as fs;
use actix_web::{
    middleware, web, App, HttpServer
};
use tera::Tera;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Priority {
    name: String,
    color: String
}

#[derive(Deserialize)]
struct Status {
    name: String,
    color: String,
}

#[derive(Deserialize)]
struct Config {
    autoreload: u16,
    strftime: String,
    dav_url: String,
    cal_name: String,
    username: String,
    password: String,
    mode: String,
    priority_sort: String,
    colors: HashMap<String, String>,
    status: HashMap<String, Status>,
    priority: Hashmap<String, Priority>
}

async fn index() -> std::io::Result<()> {
    let ctx = tera::context::new();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html").unwrap();

        App::new()
            .service(fs::Files::new("/static", "static/"))
            .app_data(web::Data::new(tera))
            .service(web::resource("/").route(web::get().to(about_index)))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}