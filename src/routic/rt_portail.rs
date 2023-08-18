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
struct PortailTemplate {
    portail: lex_portail::Portail,
    applications: Vec<lex_application::Application>,
}

// cuerl http://0.0.0.0:8080/
#[get("/")]
pub async fn portail(
    _session: Session,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    // log::info!("Session {:?} {:?}", session.status(), session.entries());
    let plexic = data.plexic.load(Ordering::Relaxed);
    let appids = unsafe {(*plexic).portail.applications.clone()};
    let apps = unsafe {&(*plexic).applications.clone()};
    let mut vapp = Vec::new();
    for appid in appids {
        let app = apps.get(&appid).unwrap();
        vapp.push(app.clone());
    }

    let html = PortailTemplate {
        portail: unsafe {(*plexic).portail.clone()},
        applications: vapp,
    }
    .render()
    .expect("template should be valid");

    Ok(Html(html))
}
