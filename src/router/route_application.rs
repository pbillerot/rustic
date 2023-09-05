//! Ouverture d'une application
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
// use log::info;
// use actix_session::Session;
use actix_web_lab::respond::Html;
use tera::Context;

use std::sync::atomic::Ordering;
use crate::AppState;

use super::Messages;

// #[get("/app/{appid}")]
pub async fn application(
    path: Path<String>,
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder> {

    let messages = Messages::get_from_request(&req);


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
