// use actix_web::{get, post, web, HttpRequest, Result, Responder};
use actix_web::{middleware, App, HttpServer, web};
// use serde::Deserialize;
use askama::Template;
// use actix_web_lab::respond::Html;
use std::env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use log::{error, info};

// Déclarations des modules
mod constants;
mod dx;
mod rx;
mod models;

#[derive(Template)]
#[template(path = "tx_portail.html")]
#[allow(dead_code)]
struct PortailTemplate {
    title: String,
    applications: Vec<String>,
}

#[derive(Clone)]
#[allow(dead_code)]
// https://actix.rs/docs/extractors#application-state-extractor
pub struct AppState {
    db: Pool<Postgres>,
    portail: dx::Portail,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    // env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    // env::set_var("RUST_LOG", std::env::var("RUST_LOG").expect("RUST_LOG must be set"));
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            error!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // pool accessible dans les requets
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                portail: dx::Portail::new().clone(),
            }))
            .service(rx::portail)
            .service(rx::list)
            .service(rx::get)
            .service(rx::create)
            .service(rx::delete)
        })
        .bind(("127.0.0.1", 8080))?
        .workers(1)
        .run()
        .await

}