//! Ouverture d'une view
//!
// use crate::sqlic::sql_utils::querlite;
use crate::{
    // lexic::lex_table::{self, Element},
    AppState, cruder::read::crud_read, middler::{flash::FlashMessage, clear_flash, get_flash},
};
use actix_session::Session;
use actix_web::{
    // get,
    // delete,
    // post,
    // HttpResponse,
    web,
    web::Path,
    Responder,
    Result, HttpRequest
};
use actix_web_lab::respond::Html;
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};

use super::get_back;

// #[get("/form/{appid}/{tableid}/{viewid}/{formid}/{id}")]
pub async fn form(
    path: Path<(String, String, String, String, String)>,
    data: web::Data<AppState>,
    session: Session,
    req: HttpRequest,
) -> Result<impl Responder> {

    let (appid, tableid, viewid, formid, id) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let application = apps.get(&appid).unwrap();
    let table = application.tables.get(&tableid).unwrap();
    let view = table.views.get(&viewid).unwrap();
    let form = table.forms.get(&formid).unwrap();

    let mut context = tera::Context::new();
    let mut messages: Vec<FlashMessage> = Vec::new();

    match crud_read(
        &data.db,
        &data.dblite,
        application, table, &form.velements, &id,
    ).await {
        Ok(mut records) => {
            context.insert("record", &records.pop());
        },
        Err(e) => {
            messages.push(FlashMessage::error(format!("{e:?}").as_str()));
            context.insert("record", &vec![{}]);
        }
    };

    if let Some(flash) = get_flash(&session)? {
        messages.push(flash);
    }
    clear_flash(&session);

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
    context.insert("back", &get_back(&req, &session));

    let html = data.template.render("tpl_form.html", &context).unwrap();

    Ok(Html(html))

}
