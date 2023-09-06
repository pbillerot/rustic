//! Ouverture d'une view
//!
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
    Result, HttpResponse,
};
use actix_web_flash_messages::{IncomingFlashMessages, FlashMessage};
// use actix_web_lab::respond::Html;
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};

// #[get("/edit/{appid}/{tableid}/{viewid}/{formid}/{id}")]
pub async fn edit(
    path: Path<(String, String, String, String, String)>,
    data: web::Data<AppState>,
    flash: IncomingFlashMessages
) -> Result<impl Responder> {

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
        ).await;

    FlashMessage::info("route_edit").send();
    for message in flash.iter() {
        println!("FLASHHHHHHHHHHH {} - {}", message.content(), message.level());
    }

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
    // context.insert("referer", &req.headers().get("referer").unwrap().to_str().ok());
    // println!("extension = {:?}", req.extensions().get::<String>());


    let html = data.template.render("tpl_edit.html", &context).unwrap();

    Ok(HttpResponse::Ok().body(html))

}

