//! Ouverture d'une view
//!
use crate::{
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
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};

use super::Message;

// #[get("/edit/{appid}/{tableid}/{viewid}/{formid}")]
pub async fn add(
    path: Path<(String, String, String, String)>,
    data: web::Data<AppState>,
    session: Session,
    // msg: Option<ReqData<servic::sr_data::Msg>>,
) -> Result<impl Responder> {

    let mut messages = Vec::new();

    let (appid, tableid, viewid, formid) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let application = apps.get(&appid).unwrap();
    let table = application.tables.get(&tableid).unwrap();
    let view = table.views.get(&viewid).unwrap();
    let form = table.forms.get(&formid).unwrap();

    let records = records_elements(
        pooldb,
        poolite,
        &"",
        &application,
        velements,
        table,
        messages,
    )
    .await;


    let mut context = tera::Context::new();

    if let Some(mut messages_session) = session.get::<Vec<Message>>("messages").unwrap() {
        // ajout des messages du contrôleur à ceux de la session
        for message in messages {
            messages_session.push(message.clone());
        }
        // copie des messages dans le contexte du template
        context.insert("messages", &messages_session);
        // suppression des messages de la session car il seront consommés (affichés) dans le template
        session.remove("messages").unwrap();
    } else {
        context.insert("messages", &messages);
    }
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

    let html = data.template.render("tpl_edit.html", &context).unwrap();

    Ok(Html(html))
}

