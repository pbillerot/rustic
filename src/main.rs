// use actix_web::{get, post, web, HttpRequest, Result, Responder};
use actix_web::{middleware, App, HttpServer, web, cookie::{self, Key},};
// use serde::Deserialize;
// use actix_web_lab::respond::Html;
use std::env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, SessionMiddleware,
};

// Déclarations des modules
mod constants;
mod lexic;
mod routic;
mod servic;
mod models;

#[derive(Clone)]
#[allow(dead_code)]
// https://actix.rs/docs/extractors#application-state-extractor
pub struct AppState {
    db: Pool<Postgres>,
    portail: lexic::Portail,
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
            log::info!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            log::error!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                portail: lexic::Portail::new().clone(),
            }))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // activation du contrôle de connexion de l'utilisateur
            .wrap(servic::sr_redirect::CheckLogin)
            // un message partagé
            // .wrap(servic::sr_data::AddMsg::enabled())
            // données disponibles dans les requetes
            // activation de actix-session
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false) // à true en https
                    // customize session and cookie expiration
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(cookie::time::Duration::minutes(1)),
                    )
                    .build(),
            )
            .service(routic::portail)
            .service(routic::login)
            .service(routic::login_post)
            .service(routic::logout)
        })
        .bind(("127.0.0.1", 8080))?
        .workers(1)
        .run()
        .await

}