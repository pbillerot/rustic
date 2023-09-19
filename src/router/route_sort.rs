//! Ouverture d'une view
//!
// use std::collections::HashMap;

use actix_session::Session;
use actix_web::{ web, HttpResponse };
use actix_web::web::Path;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Formdata {
    sortid: String,
    sortdirection: String,
}

// #[post("/sort/{appid}/{tableid}/{viewid}")]
pub async fn sort(
    path: Path<(String, String, String)>,
    form: web::Json<Formdata>,
    // data: web::Data<AppState>,
    session: Session,
) -> HttpResponse {

    let (appid, tableid, viewid) = path.into_inner();

    // enregistrement du tri dans la session
    let ctx_sortid = format!("{appid}-{tableid}-{viewid}-sortid");
    if !form.sortid.is_empty() {
        session.insert(&ctx_sortid, &form.sortid).unwrap();
    } else {
        session.remove(&ctx_sortid);
    }
    let ctx_sort_direction = format!("{appid}-{tableid}-{viewid}-sortdirection");
    if !form.sortdirection.is_empty() {
        session.insert(&ctx_sort_direction, &form.sortdirection).unwrap();
    } else {
        session.remove(&ctx_sort_direction);
    }

    let rest = Rest {
        response: "ok".to_string(),
        message: format!("{} {}", &form.sortid, &form.sortdirection),
    };
    log::info!("{:?}", session.entries());
    HttpResponse::Ok().json(rest)

}

#[derive(Serialize)]
struct Rest {
    response: String,
    message: String,
}