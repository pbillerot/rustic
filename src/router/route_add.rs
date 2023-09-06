//! Ouverture d'une view
//!
use crate::{
    // lexic::lex_table::{self, Element},
    AppState, cruder::records_elements
};
use actix_web::{
    // get,
    // delete,
    // post,
    // HttpResponse,
    web,
    web::Path,
    Responder,
    Result, HttpResponse
};
use actix_web_flash_messages::IncomingFlashMessages;
// use actix_web_lab::respond::Html;
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};

// #[get("/add/{appid}/{tableid}/{viewid}/{formid}")]
pub async fn add(
    path: Path<(String, String, String, String)>,
    data: web::Data<AppState>,
    flash: IncomingFlashMessages
) -> Result<impl Responder> {

    let (appid, tableid, viewid, formid) = path.into_inner();
    let id = ""; // c'est un ajout
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let application = apps.get(&appid).unwrap();
    let table = application.tables.get(&tableid).unwrap();
    let view = table.views.get(&viewid).unwrap();
    let form = table.forms.get(&formid).unwrap();

    let mut records = records_elements(
        &data.db,
        &data.dblite,
        &"", // pas de lecture dans la bd
        &application,
        &form.velements,
        table,
    )
    .await;

    let mut context = tera::Context::new();
    context.insert("messages", &flash);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &application);
    context.insert("table", &table);
    context.insert("view", &view);
    context.insert("form", &form);
    context.insert("appid", &appid);
    context.insert("tableid", &tableid);
    context.insert("viewid", &viewid);
    context.insert("formid", &formid);
    context.insert("id", &id);
    context.insert("key", &table.setting.key);
    context.insert("record", &records.pop());

    let html = data.template.render("tpl_add.html", &context).unwrap();

    Ok(HttpResponse::Ok().body(html))
}

