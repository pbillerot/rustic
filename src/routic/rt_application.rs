//! Ouverture d'une application
//! /app/{{app}}
//!
use actix_web::{
    get,
    // delete,
    // post,
    // HttpResponse,
    web,
    web::Path,
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
#[template(path = "tpl_application.html")]
#[allow(dead_code)]
struct ApplicationTemplate {
    portail: lex_portail::Portail,
    application: lex_application::Application,
}

// cuerl http://0.0.0.0:8080/
#[get("/app/{appid}")]
pub async fn application(
    path: Path<String>,
    _session: Session,
    data: web::Data<AppState>,
    // msg: Option<ReqData<servic::sr_data::Msg>>,
) -> Result<impl Responder> {
    // log::info!("Session {:?} {:?} {:?}", session.status(), session.entries(), path);
    let appid = path.into_inner();
    let plexic = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe {&(*plexic).applications.clone()};
    let app = apps.get(&appid).unwrap();

    let html = ApplicationTemplate {
        portail: unsafe {(*plexic).portail.clone()},
        application: app.clone(),
    }
    .render()
    .expect("template should be valid");

    Ok(Html(html))
}
