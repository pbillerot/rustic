use actix_web::{
    get,
    // delete,
    post,
    HttpResponse,
    HttpRequest,
    web,
    Responder,
    Result,
    http::{
        // header::ContentType,
        header::LOCATION,
        // StatusCode,
    }
};
// use log::info;
use askama::Template;
use actix_web_lab::respond::Html;
use actix_session::{Session, SessionExt};
use serde::{Deserialize, Serialize};
// use actix_csrf::extractor::{Csrf, CsrfGuarded, CsrfToken};

use crate::AppState;

#[derive(Template)]
#[template(path = "tpl_login.html")]
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct TplLogin {
    user_id: String,
    password: String,
}

pub async fn login(_session: Session, _data: web::Data<AppState>, req: HttpRequest) -> Result<impl Responder> {
    let session = req.get_session();
    log::info!("Session {:?} {:?}", session.status(), session.entries());

    // if let Some(is_logged) = session.get::<bool>("is_logged")? {
    //     if !is_logged {
    //         log::info!("SESSION login...");
    //         session.insert("is_logged", true)?;
    //     }
    // } else {
    //     log::info!("SESSION new login...");
    //     session.insert("is_logged", true)?;
    // }
    // log::info!("Session {:?} {:?}", session.status(), session.entries());

    let html = TplLogin {
        user_id: "".to_string(),
        password: "".to_string(),
    }
    .render()
    .expect("template should be valid");

    Ok(Html(html))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginForm {
    user_id: String,
    password: String,
}
// impl CsrfGuarded for LoginForm {
//     fn csrf_token(&self) -> &CsrfToken {
//         &self.csrf_token
//     }
// }

pub async fn login_post(params: web::Form<LoginForm>,
    _session: Session,
    _data: web::Data<AppState>,
    req: HttpRequest)
     -> Result<HttpResponse> {
    let session = req.get_session();
    log::info!("Session {:?} {:?}", session.status(), session.entries());
    log::info!("Params {:?}", params);

    if let Some(is_logged) = session.get::<bool>("is_logged")? {
        if !is_logged {
            log::info!("SESSION login...");
            session.insert("is_logged", true)?;
        }
    } else {
        log::info!("SESSION new login...");
        session.insert("is_logged", true)?;
    }
    log::info!("Session {:?} {:?}", session.status(), session.entries());

    session.insert("user_id", params.user_id.clone())?;

    Ok(
        HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish()
    )

}

// cuerl http://0.0.0.0:8080/logout
#[get("/logout")]
pub async fn logout(_session: Session, _data: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse> {
    let session = req.get_session();

    session.clear();

    Ok(
        HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish()
    )
}
