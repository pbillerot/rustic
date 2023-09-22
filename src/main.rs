use actix_web::{middleware, App, HttpServer, web, cookie::Key};
use actix_files as fs;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Sqlite, SqlitePool};

use chrono::Local;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicPtr;
use actix_session::{
    storage::CookieSessionStore, SessionMiddleware,
};
use tera::Tera;

use dotenv;

// Déclarations des modules
// mod constants;
mod lexicer;
mod router;
mod middler;
mod cruder;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    db: Pool<Postgres>,
    dblite: Pool<Sqlite>,
    template: tera::Tera,
    plexic: Arc<AtomicPtr<lexicer::lex_lexic::Lexic>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");
    // Environnemnt d'exécution ?
    let env_file = match cfg!(debug_assertions) {
        true => std::env::var("DEVELOPMENT_CONFIG").expect("ERROR DEVELOPMENT_CONFIG non définie"),
        false => std::env::var("PRODUCTION_CONFIG").expect("ERROR PRODUCTION_CONFIG non définie"),
    };
    log::info!("Environnement : {:?}", env_file);
    dotenv::from_filename(env_file).expect("Unable to load environment variables");

    // let env = env_logger::Env::default();
    // println!("{:?}", env);
    env_logger::Builder::from_default_env().format(|buf, record| {
        let time = Local::now().format("%Y-%m-%D %H:%M:%S");
        // let time = std::time::SystemTime::now();
        writeln!(buf, "[{} {:5} {} {:4} {:?}] {}",
            time,// format_rfc3339_micros(time),
            record.level(),
            if let Some(s) = record.module_path_static() { s } else { "" },
            if let Some(v) = record.line() { v } else { 0 },
            std::thread::current().id(),
            record.args())
    }).init();

    // Environnemnt d'exécution ?
    let env_file = match cfg!(debug_assertions) {
        true => std::env::var("DEVELOPMENT_CONFIG").expect("ERROR DEVELOPMENT_CONFIG non définie"),
        false => std::env::var("PRODUCTION_CONFIG").expect("ERROR PRODUCTION_CONFIG non définie"),
    };
    log::info!("Environnement : {:?}", env_file);

    // Déclarations des pools de connexion aux bases de données
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
    let urlite = ":memory:?cache=shared";
    let dblite = match SqlitePool::connect(urlite).await {
        Ok(pool) => {
            log::info!("Connection to sqlite is successful!");
            pool
        }
        Err(err) => {
            log::error!("Failed to connect to sqlite: [{}]{:?}", urlite, err);
            std::process::exit(1);
        }
    };
    // le lexic sera partagé entre tous les threads du serveur
    let lexic = match lexicer::lex_lexic::Lexic::load() {
        Ok(t) => Box::new(t),
        Err(e) => {
            log::error!("Erreur chargement {}", e);
            std::process::exit(1);
        }
    };
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            log::error!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    // AppState doit être crée devant le HyypServer, sinon le ptr sera privé au thread
    let data = AppState {
        db: pool,
        dblite: dblite,
        template: tera,
        plexic: Arc::new(AtomicPtr::new(Box::into_raw(lexic))),
    };
    let lexic_path = match std::env::var("LEXIC_PATH") {
        Ok(s) => s,
        Err(err) => {
            log::error!("Failed to read lexic_path: {:?}", err);
            std::process::exit(1);
        }
    };

    log::info!("starting HTTP server at http://0.0.0.0:8080");

    HttpServer::new(move|| {
        App::new()
        .service(fs::Files::new("/lexic", lexic_path.clone()).show_files_listing())
        .service(fs::Files::new("/static", "./static").show_files_listing())
        .app_data(web::Data::new(data.clone()))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())

            // contrôle de le session utilisateur
            .wrap(middler::mid_session::SilexSession)

            // activation de actix-session
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::generate())
                    .cookie_name("_session".to_string())
                    .cookie_secure(false) // à true en https
                    .cookie_http_only(true)
                    .cookie_path("/".to_string())
                    // customize session and cookie expiration
                    // .session_lifecycle(
                    //     PersistentSession::default().session_ttl(cookie::time::Duration::minutes(10)),
                    // )
                    .build(),
            )

            .route("/", web::get().to(router::portail))
            // .route("/login", web::post().to(router::login))
            // .route("/logout", web::post().to(router::logout))
            .route("/app/{appid}", web::get().to(router::application))
            .route("/view/{appid}/{tableid}/{viewid}", web::get().to(router::view))
            // .route("/dashboard/{appid}/{tableid}/{viewid}", web::get().to(router::dashboard))
            .route("/form/{appid}/{tableid}/{viewid}/{formid}/{id}", web::get().to(router::form))
            .route("/add/{appid}/{tableid}/{viewid}/{formid}", web::get().to(router::add))
            .route("/edit/{appid}/{tableid}/{viewid}/{formid}/{id}", web::get().to(router::edit))
            .route("/update/{appid}/{tableid}/{viewid}/{formid}/{id}", web::post().to(router::edit_post))
            .route("/insert/{appid}/{tableid}/{viewid}/{formid}", web::post().to(router::add_post))
            .route("/delete/{appid}/{tableid}/{viewid}/{id}", web::post().to(router::delete_post))
            // .route("/actionv/{appid}/{tableid}/{viewid}/{iaction}", web::post().to(router::action_view))
            // .route("/actionp/{appid}/{tableid}/{viewid}/{id}", web::post().to(router::action_press))
            // .route("/actionf/{appid}/{tableid}/{viewid}/{formid}/{id}/{action}", web::post().to(router::action_form))
            // .route("/actione/{appid}/{tableid}/{viewid}/{formid}/{id}/{action}", web::post().to(router::action_element))
            // .route("/actionx/{appid}/{tableid}/{viewid}/{id}/{action}", web::post().to(router::action_ajax))
            // .route("/ajax/{appid}/{tableid}/{viewid}/{formid}/{action}", web::post().to(router::edit))
            // .route("/share/{appid}/{shareid}", web::post().to(router::share))
            // .route("/search/{appid}/{tableid}/{viewid}/{id}", web::post().to(router::search))
            .route("/filter/{appid}/{tableid}/{viewid}", web::post().to(router::filter))
            .route("/sort/{appid}/{tableid}/{viewid}", web::post().to(router::sort))
            // Gestion du lexique
            .route("/lexic/action/{action}", web::get().to(router::lexicall))
            // .route("/lexic/document", web::get().to(router::lexic))
            // .route("/lexic/document", web::post().to(router::lexic_post))
            // .route("/lexic/log", web::get().to(router::log))

        })
        .bind(("0.0.0.0", 8080))?
        .workers(match std::env::var("WORKERS") {
            Ok(ss) => ss.parse::<usize>().unwrap(),
            Err(_) => 1
        })
        .run()
        .await

}

