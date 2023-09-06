//! Ouverture d'une application
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
use actix_web_flash_messages::IncomingFlashMessages;
use tera::Context;

use std::sync::atomic::Ordering;
use crate::AppState;

// #[get("/app/{appid}")]
pub async fn application(
    path: Path<String>,
    data: web::Data<AppState>,
    flash: IncomingFlashMessages
) -> Result<impl Responder> {

    let appid = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe {&(*ptr).applications.clone()};
    let app = apps.get(&appid).unwrap();

    let mut context = Context::new();
    context.insert("messages", &flash);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &app);
    context.insert("appid", &appid);
    let html = data.template.render("tpl_application.html", &context).unwrap();

    Ok(HttpResponse::Ok().body(html))
}
