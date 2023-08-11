use actix_web::{
    get,
    // delete,
    // post,
    // HttpResponse,
    web,
    web::ReqData,
    Responder,
    Result,
};
// use log::info;
use askama::Template;
use actix_web_lab::respond::Html;
use actix_session::Session;

use crate::AppState;
use crate::servic;


#[derive(Template)]
#[template(path = "tx_portail.html")]
#[allow(dead_code)]
struct PortailTemplate {
    title: String,
    applications: Vec<String>,
}

// cuerl http://0.0.0.0:8080/
#[get("/")]
pub async fn portail(session: Session, data: web::Data<AppState>, msg: Option<ReqData<servic::sx_data::Msg>>) -> Result<impl Responder> {
    log::info!("Session {:?} {:?}", session.status(), session.entries());

    if let Some(msg_data) = msg {
        let servic::sx_data::Msg(message) = msg_data.into_inner();
        log::info!("Msg: {:?}", message);
    } else {
        log::error!("no message found");
    }

    let html = PortailTemplate {
        title: data.portail.title.clone(),
        applications: data.portail.applications.clone(),
    }
    .render()
    .expect("template should be valid");

    Ok(Html(html))
}
