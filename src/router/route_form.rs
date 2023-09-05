//! Ouverture d'une view
//!
// use crate::sqlic::sql_utils::querlite;
use crate::{
    // lexic::lex_table::{self, Element},
    AppState, cruder::read::crud_read,
};
use actix_web::{
    // get,
    // delete,
    // post,
    // HttpResponse,
    web,
    web::Path,
    Responder,
    Result, HttpRequest,
};
use actix_web_lab::respond::Html;
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};

use super::Messages;

// #[get("/form/{appid}/{tableid}/{viewid}/{formid}/{id}")]
pub async fn form(
    path: Path<(String, String, String, String, String)>,
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder> {

    let mut messages = Messages::get_from_request(&req);

    let (appid, tableid, viewid, formid, id) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let application = apps.get(&appid).unwrap();
    let table = application.tables.get(&tableid).unwrap();
    let view = table.views.get(&viewid).unwrap();
    let form = table.forms.get(&formid).unwrap();

    let mut records = crud_read(
        &data.db,
        &data.dblite,
        application, table, &form.velements, &id,
        &mut messages).await;

    let mut context = tera::Context::new();
    context.insert("messages", &messages);
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

    let html = data.template.render("tpl_form.html", &context).unwrap();

    Ok(Html(html))
}
