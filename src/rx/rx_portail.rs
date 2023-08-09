use actix_web::{
    get,
    // delete,
    // post,
    // HttpResponse,
    web,
    Responder,
    Result,
};
// use log::info;
use askama::Template;
use actix_web_lab::respond::Html;

use crate::AppState;



#[derive(Template)]
#[template(path = "tx_portail.html")]
#[allow(dead_code)]
struct PortailTemplate {
    title: String,
    applications: Vec<String>,
}

// http://127.0.0.1:8080/portail
#[get("/")]
pub async fn portail(data: web::Data<AppState>) -> Result<impl Responder> {
    // let portail = dx::Portail::new();

    let html = PortailTemplate {
        title: data.portail.title.clone(),
        applications: data.portail.applications.clone(),
    }
    .render()
    .expect("template should be valid");

    Ok(Html(html))
}
