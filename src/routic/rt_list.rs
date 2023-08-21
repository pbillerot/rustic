//! Ouverture d'une application
//! /app/{{app}}
//!
// use std::collections::HashMap;
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
use crate::lexic::{lex_application, lex_portail, lex_table};

#[derive(Template)]
#[template(path = "tpl_list.html")]
#[allow(dead_code)]
struct ListTemplate<'a> {
    portail: &'a lex_portail::Portail,
    application: &'a lex_application::Application,
    table: &'a lex_table::Table,
    view: &'a lex_table::View,
}

// cuerl http://0.0.0.0:8080/
#[get("/list/{appid}/{tableid}/{viewid}")]
pub async fn list(
    path: Path<(String, String, String)>,
    _session: Session,
    data: web::Data<AppState>,
    // msg: Option<ReqData<servic::sr_data::Msg>>,
) -> Result<impl Responder> {
    // log::info!("Session {:?} {:?} {:?}", session.status(), session.entries(), path);
    let (appid, tableid, viewid) = path.into_inner();
    let plexic = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe {&(*plexic).applications.clone()};
    let app = apps.get(&appid).unwrap();
    let table = app.tables.get(&tableid).unwrap();
    let view = table.views.get(&viewid).unwrap();

    let html = ListTemplate {
        portail: unsafe {&(*plexic).portail},
        application: &app,
        table: &table,
        view: &view,
    }
    .render()
    .expect("template should be valid");

    Ok(Html(html))
}
