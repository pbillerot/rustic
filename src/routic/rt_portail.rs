//! Page d'accueil de Silex
//! Ouverture du portail

use actix_web::{
    get,
    // delete,
    // post,
    // HttpResponse,
    web,
    // web::ReqData,
    Responder,
    Result,
};
// use log::info;
use actix_session::Session;
use actix_web_lab::respond::Html;
use askama::Template;
use std::sync::atomic::Ordering;
// use crate::servic;
use crate::AppState;

#[derive(Template)]
#[template(path = "tpl_portail.html")]
#[allow(dead_code)]
struct PortailTemplate {
    title: String,
    applications: Vec<String>,
    user_id: String,
}

// cuerl http://0.0.0.0:8080/
#[get("/")]
pub async fn portail(
    session: Session,
    data: web::Data<AppState>,
    // msg: Option<ReqData<servic::sr_data::Msg>>,
) -> Result<impl Responder> {
    log::info!("Session {:?} {:?}", session.status(), session.entries());
    let plexic = data.plexic.load(Ordering::Relaxed);
    // if let Some(msg_data) = msg {
    //     let servic::sr_data::Msg(message) = msg_data.into_inner();
    //     log::info!("Msg: {:?}", message);
    // } else {
    //     log::error!("no message found");
    // }

    let userid = if let Some(user_id) = session.get::<String>("user_id")? {
        user_id
    } else {
        "anonymous".to_string()
    };

    let html = PortailTemplate {
        title: unsafe {(*plexic).portail.title.clone()},
        applications: unsafe {(*plexic).portail.applications.clone()},
        user_id: userid,
    }
    .render()
    .expect("template should be valid");

    Ok(Html(html))
}
