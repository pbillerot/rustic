use actix_web::{middleware, App, HttpServer, web, cookie::{self, Key}};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use std::sync::Arc;
use std::sync::atomic::AtomicPtr;
// use std::sync::atomic::Ordering;
// use std::sync::Mutex;

use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, SessionMiddleware,
};
use dotenv;
// Déclarations des modules
// mod constants;
mod lexic;
mod routic;
mod servic;
// mod sqlic;


#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    db: Pool<Postgres>,
    plexic: Arc<AtomicPtr<lexic::lex_lexic::Lexic>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Environnemnt d'exécution ?
    let env_file = match cfg!(debug_assertions) {
        true => std::env::var("DEVELOPMENT_CONFIG").expect("ERROR DEVELOPMENT_CONFIG non définie"),
        false => std::env::var("PRODUCTION_CONFIG").expect("ERROR PRODUCTION_CONFIG non définie"),
    };
    log::info!("Environnement : {:?}", env_file);

    dotenv::from_filename(env_file).expect("Unable to load environment variables");

    // Déclarations des pools de connexion aux base de données
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
    // le lexic sera partagé entre tous les threads du serveur
    let lexic = match lexic::lex_lexic::Lexic::load() {
        Ok(t) => Box::new(t),
        Err(e) => {
            log::error!("Erreur chargement {}", e);
            std::process::exit(1);
        }
    };

    log::info!("starting HTTP server at http://0.0.0.0:8080");

    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                plexic: Arc::new(AtomicPtr::new(Box::into_raw(lexic.clone()))),
            }))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())

            // activation du contrôle de connexion de l'utilisateur
            // .wrap(servic::sr_redirect::CheckLogin)

            // un message partagé
            // .wrap(servic::sr_data::AddMsg::enabled())

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
            .service(routic::application)
            .service(routic::lexicall)
        })
        .bind(("0.0.0.0", 8080))?
        .workers(match std::env::var("WORKERS") {
            Ok(ss) => ss.parse::<usize>().unwrap(),
            Err(_) => 1
        })
        .run()
        .await

}
