//! Ouverture d'une application
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
// use log::info;
// use actix_session::Session;
use actix_web_lab::respond::Html;
use tera::Context;

use std::sync::atomic::Ordering;
use crate::AppState;

use super::MESSAGE_LEVEL_INFO;
use super::Message;

// #[get("/app/{appid}")]
pub async fn application(
    path: Path<String>,
    data: web::Data<AppState>,
    // msg: Option<ReqData<servic::sr_data::Msg>>,
) -> Result<impl Responder> {
    // log::info!("Session {:?} {:?} {:?}", session.status(), session.entries(), path);
    let mut messages = Vec::new();
    messages.push(Message::new("app:Tout va bien", MESSAGE_LEVEL_INFO));


    let appid = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe {&(*ptr).applications.clone()};
    let app = apps.get(&appid).unwrap();

    let mut context = Context::new();
    context.insert("messages", &messages);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &app);
    context.insert("appid", &appid);
    let html = data.template.render("tpl_application.html", &context).unwrap();

    Ok(Html(html))
}
