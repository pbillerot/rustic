
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use actix_web::{
    // get,
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
use crate::lexicer;

#[derive(Debug)]

#[derive(Deserialize, Serialize)]
pub struct Info {
    action: String,
}

// #[get("/lexic/{action}")]
pub async fn lexicall(info: web::Path<Info>, data: web::Data<AppState>) -> impl Responder {
    // println!("{:?}", info);
    if info.action == "refresh" {
        log::info!("On Lexic action [{}] ...", info.action);
        // Réservation du pointeur - les autres threads seront en attente
        let ptr1 = data.plexic.load(Ordering::Relaxed);
        unsafe { log::info!("ptr1: {}", (*ptr1).portail.title)}

        // Chargement d'un nouveau lexique
        let mut _newlexic = match lexicer::lex_lexic::Lexic::load() {
            Ok(t) => {
                let _oldptr = data.plexic.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_x| {
                    log::info!("New lexic ok");
                    Some(Box::into_raw(Box::new(t.clone())))
                });
            },
            Err(e) => {
                log::error!("Error loading lexic {:?}", e);
            }
        };
        let ptr2 = data.plexic.load(Ordering::Relaxed);
        unsafe { log::info!("ptr2: {}", (*ptr2).portail.title)}

        unsafe { log::info!("ptr1: {}", (*ptr1).portail.title)}

        log::info!("On Lexic action [{}] end", info.action);
    }

    // let newptr = data.plexic.load(Ordering::Relaxed);
    // unsafe {println!("après {:?}", (*newptr).portail)}

    HttpResponse::Ok().body("Hello world!")
}