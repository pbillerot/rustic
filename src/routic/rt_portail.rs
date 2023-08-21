//! Page d'accueil de Silex
//! Ouverture du portail

use actix_web::{
    get,
    // delete,
    // post,
    // HttpResponse,
    web,
    // web::ReqData,
    Responder,
    Result,
};
// use log::info;
use actix_session::Session;
use actix_web_lab::respond::Html;
use askama::Template;
use std::sync::atomic::Ordering;

// use crate::servic;
use crate::AppState;
use crate::lexic::{lex_application, lex_portail};

#[derive(Template)]
#[template(path = "tpl_portail.html")]
#[allow(dead_code)]
struct PortailTemplate<'a> {
    portail: &'a lex_portail::Portail,
    applications: &'a Vec<&'a lex_application::Application>,
}

// cuerl http://0.0.0.0:8080/
#[get("/")]
pub async fn portail(
    _session: Session,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    // log::info!("Session {:?} {:?}", session.status(), session.entries());
    let ptr = data.plexic.load(Ordering::Relaxed);

    // unsafe { log::info!("ptr: {}", (*ptr).portail.title)}

    let appids = unsafe {(*ptr).portail.appids.clone()};
    let apps: &std::collections::HashMap<String, lex_application::Application> = unsafe {&(*ptr).applications};
    let mut vapp: Vec<&lex_application::Application> = Vec::new();
    for appid in appids {
        let app = apps.get(&appid).unwrap();
        vapp.push(&app);
    }

    let html = PortailTemplate {
        portail: unsafe {&(*ptr).portail},
        applications: &vapp,
    }
    .render()
    .expect("template should be valid");

    Ok(Html(html))
}
