//! Ouverture d'une view
//!
use crate::{
    // lexic::lex_table::{self, Element},
    AppState, cruder::list::crud_list
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
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};

// cuerl http://0.0.0.0:8080/
// #[get("/view/{appid}/{tableid}/{viewid}")]
pub async fn view(
    path: Path<(String, String, String)>,
    data: web::Data<AppState>,
    flash: IncomingFlashMessages
) -> Result<impl Responder> {

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
        ).await;

    let mut context = tera::Context::new();
    context.insert("messages", &flash);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &app);
    context.insert("table", &table);
    context.insert("view", &view);
    context.insert("appid", &appid);
    context.insert("tableid", &tableid);
    context.insert("viewid", &viewid);
    context.insert("records", &records);
    context.insert("key", &table.setting.key);

    let html = data.template.render("tpl_view.html", &context).unwrap();

    Ok(HttpResponse::Ok().body(html))
}
