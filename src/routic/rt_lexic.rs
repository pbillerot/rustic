
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use actix_web::{
    get,
    // delete,
    // post,
    HttpResponse,
    web,
    // web::ReqData,
    Responder,
    // Result,
};
// use log::info;
// use actix_session::Session;
// use actix_web_lab::respond::Html;
// use std::sync::atomic::Ordering;
// use crate::servic;
use crate::AppState;
use crate::lexic;

#[derive(Debug)]

#[derive(Deserialize, Serialize)]
struct Info {
    action: String,
}

#[get("/lexic/{action}")]
async fn lexicall(info: web::Path<Info>, data: web::Data<AppState>) -> impl Responder {
    // println!("{:?}", info);
    if info.action == "refresh" {
        log::info!("On Lexic action [{}] ...", info.action);
        let ptr = data.plexic.load(Ordering::Relaxed);

        // Chargement d'un nouveau lexique
        let _newlexic = match lexic::lex_lexic::Lexic::load() {
            Ok(t) => {
                let _result = data.plexic.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
                    if x == ptr {
                        log::info!("New lexic ok");
                        Some(Box::into_raw(Box::new(t.clone())))
                    } else {
                        log::error!("Error update lexique");
                        None
                    }
                });

            },
            Err(e) => {
                log::error!("Error loading lexic {:?}", e);
            }
        };

        log::info!("On Lexic action [{}] end", info.action);
    }

    // let newptr = data.plexic.load(Ordering::Relaxed);
    // unsafe {println!("après {:?}", (*newptr).portail)}

    HttpResponse::Ok().body("Hello world!")
}