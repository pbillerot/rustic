use actix_web::{
    get,
    // delete,
    // post,
    // HttpResponse,
    HttpRequest,
    web,
    Responder,
    Result,
};
// use log::info;
use askama::Template;
use actix_web_lab::respond::Html;
use actix_session::{Session, SessionExt};

use crate::AppState;

#[derive(Template)]
#[template(path = "tx_login.html")]
#[allow(dead_code)]
struct MaPage {
    title: &'static str,
}

// cuerl http://0.0.0.0:8080/login
#[get("/login")]
pub async fn login(_session: Session, _data: web::Data<AppState>, req: HttpRequest) -> Result<impl Responder> {
    let session = req.get_session();
    log::info!("Session {:?} {:?}", session.status(), session.entries());

    if let Some(is_logged) = session.get::<bool>("is_logged")? {
        if !is_logged {
            log::info!("SESSION login...");
            session.insert("is_logged", true)?;
        }
    } else {
        log::info!("SESSION new login...");
        session.insert("is_logged", true)?;
    }
    // session.renew();
    log::info!("Session {:?} {:?}", session.status(), session.entries());

    let html = MaPage {
        title: "Veuillez vous identifier",
    }
    .render()
    .expect("template should be valid");

    Ok(Html(html))
}
// cuerl http://0.0.0.0:8080/logout
#[get("/logout")]
pub async fn logout(_session: Session, _data: web::Data<AppState>, req: HttpRequest) -> Result<impl Responder> {
    let session = req.get_session();
    log::info!("Session {:?} {:?}", session.status(), session.entries());

    session.renew();

    log::info!("Session {:?} {:?}", session.status(), session.entries());

    let html = MaPage {
        title: "Veuillez vous identifier",
    }
    .render()
    .expect("template should be valid");

    Ok(Html(html))
}
