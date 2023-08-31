//! Ouverture d'une view
//!
use crate::cruder::sql_crud::crud_list;
// use crate::sqlic::sql_utils::querlite;
use crate::{
    // lexic::lex_table::{self, Element},
    AppState,
};
use actix_web::{
    // get,
    // delete,
    // post,
    // HttpResponse,
    web,
    web::Path,
    Responder,
    Result,
};
use actix_web_lab::respond::Html;
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};
use crate::service;

// cuerl http://0.0.0.0:8080/
// #[get("/list/{appid}/{tableid}/{viewid}")]
pub async fn list(
    path: Path<(String, String, String)>,
    data: web::Data<AppState>,
    // msg: Option<ReqData<servic::sr_data::Msg>>,
) -> Result<impl Responder> {
    // log::info!("Session {:?} {:?} {:?}", session.status(), session.entries(), path);
    let mut messages = Vec::new();
    messages.push(service::Message::new("list:Tout va bien", service::MESSAGE_LEVEL_INFO));

    let (appid, tableid, viewid) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let app = apps.get(&appid).unwrap();
    let table = app.tables.get(&tableid).unwrap();
    let view = table.views.get(&viewid).unwrap();

    let records = crud_list(
        &data.db,
        &data.dblite,
        app, &tableid, &viewid, "",
        "",
         &mut messages
        ).await;

    let mut context = tera::Context::new();
    context.insert("messages", &messages);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &app);
    context.insert("table", &table);
    context.insert("view", &view);
    context.insert("appid", &appid);
    context.insert("tableid", &tableid);
    context.insert("viewid", &viewid);
    context.insert("records", &records);

    let html = data.template.render("tpl_list.html", &context).unwrap();

    Ok(Html(html))
}
