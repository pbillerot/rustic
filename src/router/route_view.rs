//! Ouverture d'une view
//!
use std::sync::atomic::Ordering;

use crate::{
    middler::{clear_flash, flash::FlashMessage, get_flash},
    // lexic::lex_table::{self, Element},
    AppState,
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

use super::{get_back, view_table::Tview};

// "/view/{appid}/{tableid}/{viewid}
pub async fn view(
    path: Path<(String, String, String)>,
    data: web::Data<AppState>,
    session: Session,
) -> Result<impl Responder> {
    let (appid, tableid, viewid) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let application = apps.get(&appid).unwrap();
    let table = application.tables.get(&tableid).unwrap();
    let view = table.views.get(&viewid).unwrap();

    let mut context = tera::Context::new();
    let mut messages: Vec<FlashMessage> = Vec::new();

    let mut tvs: Vec<Tview> = Vec::new();
    match Tview::new(
        &application,
        &tableid,
        &viewid,
        "",
        &session,
        &data.db,
        &data.dblite,
    )
    .await
    {
        Ok(tv) => {
          context.insert("view", &tv.view);
          tvs.push(tv);
        }
        Err(e) => {
          context.insert("view", &view);
          messages.push(FlashMessage::error(format!("{e:?}").as_str()));
        }
    };
    context.insert("tvs", &tvs);

    // FLASH
    if let Some(flash) = get_flash(&session)? {
        messages.push(flash);
    }
    clear_flash(&session);

    context.insert("messages", &messages);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &application);
    context.insert("table", &table);
    context.insert("appid", &appid);
    context.insert("tableid", &tableid);
    context.insert("viewid", &viewid);
    context.insert("key", &table.setting.key);
    context.insert("back", &get_back(&session));

    let tpl = match view.type_view.as_str() {
        "table" => "tpl_view_table.html",
        "card" => "tpl_view_card.html",
        "dashboard" => "tpl_view_dash.html",
        _ => "tpl_view_table.html",
    };

    let html = data.template.render(tpl, &context).unwrap();

    Ok(Html(html.to_string()))
}
