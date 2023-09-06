//! Page d'accueil de Silex
//! Ouverture du portail

use actix_web::{
    // get,
    Error,
    // delete,
    // post,
    // HttpResponse,
    web,
    // web::ReqData,
    Responder,
    Result, HttpResponse
};
use actix_web_flash_messages::IncomingFlashMessages;
use tera::Context;

use std::sync::atomic::Ordering;

// use crate::service;
use crate::lexicer::lex_application;
use crate::AppState;

// #[get("/")]
pub async fn portail(data: web::Data<AppState>,
    flash: IncomingFlashMessages
    ) -> Result<impl Responder, Error> {

    let ptr = data.plexic.load(Ordering::Relaxed);

    let appids = unsafe { (*ptr).portail.appids.clone() };
    let apps: &std::collections::HashMap<String, lex_application::Application> =
        unsafe { &(*ptr).applications };
    let mut vapp: Vec<&lex_application::Application> = Vec::new();
    for appid in appids {
        let app = apps.get(&appid).unwrap();
        vapp.push(&app);
    }

    let mut context = Context::new();
    context.insert("messages", &flash);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("applications", &vapp);
    let html = data.template.render("tpl_portail.html", &context).unwrap();

    Ok(HttpResponse::Ok().body(html))
}
