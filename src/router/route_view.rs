//! Ouverture d'une view
//!
use crate::{
    // lexic::lex_table::{self, Element},
    AppState, cruder::list::crud_list, middler::{flash::FlashMessage, clear_flash, get_flash}
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
    Result,
};
use actix_web_lab::respond::Html;
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};

// cuerl http://0.0.0.0:8080/
// #[get("/view/{appid}/{tableid}/{viewid}")]
pub async fn view(
    path: Path<(String, String, String)>,
    data: web::Data<AppState>,
    session: Session,
) -> Result<impl Responder> {

    let (appid, tableid, viewid) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let app = apps.get(&appid).unwrap();
    let table = app.tables.get(&tableid).unwrap();
    let view = table.views.get(&viewid).unwrap();

    let mut context = tera::Context::new();
    let mut messages: Vec<FlashMessage> = Vec::new();

    match crud_list(
        &data.db,
        &data.dblite,
        app, &tableid, &viewid, "",
    ).await {
        Ok(records) => {
            context.insert("records", &records);
        },
        Err(e) => {
            messages.push(FlashMessage::error(format!("{e:?}").as_str()));
            context.insert("records", &vec![{}]);
        }
    };

    if let Some(flash) = get_flash(&session)? {
        messages.push(flash);
    }
    clear_flash(&session);

    context.insert("messages", &messages);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &app);
    context.insert("table", &table);
    context.insert("view", &view);
    context.insert("appid", &appid);
    context.insert("tableid", &tableid);
    context.insert("viewid", &viewid);
    context.insert("key", &table.setting.key);

    let html = data.template.render("tpl_view.html", &context).unwrap();

    Ok(Html(html))
}
