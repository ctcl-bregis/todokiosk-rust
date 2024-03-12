// ToDoKiosk - CTCL 2023-2024
// File: src/main.rs
// Purpose: Main code
// Created: March 10, 2024
// Modified: March 12, 2024

pub const VERSION: &str = "0.1.0";

use std::collections::HashMap;
use std::path::Path;
use actix_files as fs;
use actix_web::{web, App, HttpServer, HttpResponse, Result, Responder, Error};
use tera::Tera;
use serde::{Deserialize, Serialize};
use url::Url;
use ureq::Agent;
use todokiosk::read_file;
use chrono::{Local, NaiveDateTime, Utc};

#[derive(Deserialize, Serialize)]
struct Priority {
    name: String,
    color: String,
    value: u8
}

#[derive(Deserialize, Serialize)]
struct Status {
    name: String,
    color: String,
}

#[derive(Deserialize)]
struct Config {
    serverip: String,
    serverport: u16,
    autoreload: String,
    strftime: String,
    dav_url: String,
    cal_name: String,
    username: String,
    password: String,
    //mode: String,
    priority_sort: String,
    show_completed: bool,
    colors: HashMap<String, String>,
    status: HashMap<String, Status>,
    priority: HashMap<String, Priority>,
    class: HashMap<String, bool>
}

// Data is in a struct so it can be inserted into web::Data
struct Styling {
    styling: String
}

#[derive(Deserialize, Serialize)]
struct Task {
    color: String,
    status: Status,
    priority: Priority,
    summary: String,
    categories: String,
    created: String,
    modified: String
}

fn loadconfig() -> Result<Config, std::io::Error> {
    let config: Config;

    if Path::new("config_private.json").exists() {
        config = serde_json::from_str(&read_file("config_private.json").unwrap()).unwrap();
    } else if Path::new("config.json").exists() {
        config = serde_json::from_str(&read_file("config.json").unwrap()).unwrap();
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Neither config_private.json or config.json found"))
    }

    Ok(config)
}

fn todo2task(todo: minicaldav::Event, config: &actix_web::web::Data<Config>) -> Option<Task> {
    let mut tcolor: String = String::from("");
    let mut tstatus: Status = Status { name: "".to_string(), color: "".to_string() };
    let mut tpriority: Priority = Priority { name: "".to_string(), color: "".to_string(), value: 0 };
    let mut tcategories: String = String::from("");
    let mut tsummary: String = String::from("");
    let mut tcreated: String = String::from("");
    let mut tmodified: String = String::from("");

    for prop in &todo.ical().children.clone().into_iter().nth(0).unwrap().properties {
        //dbg!(&prop);
        if prop.name == "COLOR" {
            tcolor = config.colors.get(&prop.value.clone()).unwrap().to_string();
        }
        if prop.name == "STATUS" {
            if prop.value == "COMPLETED" && !config.show_completed {
                return None;
            }
            tstatus = Status { name: config.status.get(&prop.value.clone()).unwrap().name.clone(), color: config.status.get(&prop.value.clone()).unwrap().color.clone() };
        }
        if prop.name == "PRIORITY" {
            tpriority = Priority { name: config.priority.get(&prop.value.clone()).unwrap().name.clone(), color: config.priority.get(&prop.value.clone()).unwrap().color.clone(), value: config.priority.get(&prop.value.clone()).unwrap().value.clone()};
        }
        if prop.name == "SUMMARY" {
            tsummary = prop.value.clone();
            tsummary = tsummary.replace("\\,", ",");
        }
        if prop.name == "CREATED" {
            let parsedtime = NaiveDateTime::parse_from_str(&prop.value, "%Y%m%dT%H%M%SZ").unwrap().and_local_timezone(Utc).unwrap();
            tcreated = parsedtime.with_timezone(&Local).format(&config.strftime).to_string();
        }
        if prop.name == "LAST-MODIFIED" {
            let parsedtime = NaiveDateTime::parse_from_str(&prop.value, "%Y%m%dT%H%M%SZ").unwrap().and_local_timezone(Utc).unwrap();
            tmodified = parsedtime.with_timezone(&Local).format(&config.strftime).to_string();
        }
        if prop.name == "CLASS" {
            match config.class.get(&prop.value) {
                Some(false) => return None,
                Some(true) => (),
                None => ()
            }
        }
        if prop.name == "CATEGORIES" {
            tcategories = prop.value.replace(",", ", ")
        }
    }

    let task: Task = Task {
        color: tcolor,
        status: tstatus,
        priority: tpriority,
        summary: tsummary,
        categories: tcategories,
        created: tcreated,
        modified: tmodified
    };

    Some(task)
}

async fn index(tmpl: web::Data<tera::Tera>, config: web::Data<Config>, styling: web::Data<Styling>) -> Result<impl Responder, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("version", VERSION);
    ctx.insert("title", &format!("ToDoKiosk - {}", &config.cal_name));
    ctx.insert("cal_name", &config.cal_name);
    ctx.insert("autoreload", &config.autoreload);
    ctx.insert("styling", &styling.styling);

    let agent = Agent::new();
    let url = Url::parse(&config.dav_url).unwrap();
    let username = &config.username;
    let password = &config.password;
    let credentials = minicaldav::Credentials::Basic(username.into(), password.into());
    // Set calendars to an empty vec instead of panicking on a connection error
    let calendars = match minicaldav::get_calendars(agent.clone(), &credentials, &url) {
        Ok(calendars) => calendars,
        Err(_e) => Vec::new()
    };

    let mut tasks: Vec<Task> = Vec::new();
    if calendars.len() > 0 {
        let mut targetcalendar: Option<minicaldav::Calendar> = None;
        for calendar in calendars {
            if calendar.name() == &config.cal_name {
                targetcalendar = Some(calendar);
            }
        }
        if let Some(value) = targetcalendar {
            let credentials = minicaldav::Credentials::Basic(username.into(), password.into());
            let (todos, _errors) = minicaldav::get_todos(agent.clone(), &credentials, &value).unwrap();
            for todo in todos {
                match todo2task(todo, &config) {
                    Some(task) => tasks.push(task),
                    None => ()
                }
            }
        }
    }

    if config.priority_sort == "ascending" {
        tasks.sort_by(|a, b| b.priority.value.cmp(&a.priority.value));
    } else {
        tasks.sort_by(|a, b| a.priority.value.cmp(&b.priority.value));
    }

    ctx.insert("tasks", &tasks);

    let s = match tmpl.render("main.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(err) => return Ok(HttpResponse::InternalServerError().body(format!("{:?}", err)))
    };

    Ok(s)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config = loadconfig().expect("Config load failed");

    if config.dav_url == "" {
        panic!("CalDAV server URL is empty, one must be set in config.json")
    }

    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html").unwrap();
        
        let config: Config = loadconfig().expect("Config load failed");
        let styling: Styling = Styling { styling: read_file("static/common.css").unwrap() };

        App::new()
            .service(fs::Files::new("/static", "static/"))
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(config))
            .app_data(web::Data::new(styling))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind((config.serverip, config.serverport))?
    .run()
    .await
}