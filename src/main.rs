use actix_web::{get, post, web, HttpRequest, Result, Responder};
use serde::Deserialize;
use askama::Template;
use actix_web_lab::respond::Html;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Info {
    username: String,
}
#[derive(Deserialize)]
struct Group {
    user_id: u32,
    friend: String,
}

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn users(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[get("/useri/{user_id}/{friend}")] // <- define path parameters
async fn useri(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}

/// extract path info using serde
#[get("/groups/{user_id}/{friend}")] // <- define path parameters
async fn groups(group: web::Path<Group>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        group.friend, group.user_id
    ))
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/")]
async fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}

/// deserialize `Info` from request's body
#[post("/submit")]
async fn submit(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[derive(Template)]
#[template(path = "hello.html")]
#[allow(dead_code)]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
}
#[derive(Template)]
#[template(path = "index.html")]
struct Index;

// http://127.0.0.1:8080/hello?name=toto
#[get("/hello")]
async fn hello(query: web::Query<HashMap<String, String>>) -> Result<impl Responder> {
    let html = if let Some(name) = query.get("name") {
        UserTemplate {
            name,
            text: "Welcome!",
        }
        .render()
        .expect("template should be valid")
    } else {
        Index.render().expect("template should be valid")
    };

    Ok(Html(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    println!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| App::new()
        .service(index)
        .service(useri)
        .service(users)
        .service(groups)
        .service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}