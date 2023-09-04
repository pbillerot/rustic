//! Ouverture d'une view
//!
use crate::{
    // lexic::lex_table::{self, Element},
    AppState, cruder::list::crud_list
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


// cuerl http://0.0.0.0:8080/
// #[get("/view/{appid}/{tableid}/{viewid}")]
pub async fn view(
    path: Path<(String, String, String)>,
    data: web::Data<AppState>,
    session: Session,
    // msg: Option<ReqData<servic::sr_data::Msg>>,
) -> Result<impl Responder> {

    let mut messages = Vec::new();

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
        &mut messages
        ).await;

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
    context.insert("application", &app);
    context.insert("table", &table);
    context.insert("view", &view);
    context.insert("appid", &appid);
    context.insert("tableid", &tableid);
    context.insert("viewid", &viewid);
    context.insert("records", &records);
    context.insert("key", &table.setting.key);

    let html = data.template.render("tpl_view.html", &context).unwrap();

    Ok(Html(html))
}
