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
    Result,
};
// use log::info;
// use actix_session::Session;
use actix_web_lab::respond::Html;
use tera::Context;

use std::sync::atomic::Ordering;

// use crate::service;
use crate::lexicer::lex_application;
use crate::service;
use crate::AppState;

// #[get("/")]
pub async fn portail(data: web::Data<AppState>) -> Result<impl Responder, Error> {
    // log::info!("Session {:?} {:?}", session.status(), session.entries());
    let ptr = data.plexic.load(Ordering::Relaxed);
    let mut messages = Vec::new();
    messages.push(service::Message::new("port:Tout va bien", service::MESSAGE_LEVEL_INFO));

    let appids = unsafe { (*ptr).portail.appids.clone() };
    let apps: &std::collections::HashMap<String, lex_application::Application> =
        unsafe { &(*ptr).applications };
    let mut vapp: Vec<&lex_application::Application> = Vec::new();
    for appid in appids {
        let app = apps.get(&appid).unwrap();
        vapp.push(&app);
    }

    let mut context = Context::new();
    context.insert("messages", &messages);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("applications", &vapp);
    let html = data.template.render("tpl_portail.html", &context).unwrap();

    Ok(Html(html))
}
