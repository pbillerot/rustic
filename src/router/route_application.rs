//! Ouverture d'une application
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

use std::sync::atomic::Ordering;
use crate::{AppState, middler::{flash::FlashMessage, clear_flash, get_flash}};

// #[get("/app/{appid}")]
pub async fn application(
    path: Path<String>,
    data: web::Data<AppState>,
    session: Session,
) -> Result<impl Responder> {

    let appid = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe {&(*ptr).applications.clone()};
    let app = apps.get(&appid).unwrap();

    let mut messages: Vec<FlashMessage> = Vec::new();
    if let Some(flash) = get_flash(&session)? {
        messages.push(flash);
    }
    clear_flash(&session);

    let mut context = tera::Context::new();
    context.insert("messages", &messages);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &app);
    context.insert("appid", &appid);
    context.insert("back", "");
    let html = data.template.render("tpl_application.html", &context).unwrap();

    Ok(Html(html))
}
