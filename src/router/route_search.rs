//! Ouverture d'une view
//!
// use std::collections::HashMap;

use actix_session::Session;
use actix_web::{ web, HttpResponse };
use actix_web::web::Path;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Formdata {
    search: String,
}

// #[post("/search/{appid}/{tableid}/{viewid}")]
pub async fn search(
    path: Path<(String, String, String)>,
    form_json: web::Json<Formdata>,
    session: Session,
) -> HttpResponse {

    let (appid, tableid, viewid) = path.into_inner();

    // enregistrement de la recherche dans la session
    let ctx_search = format!("{appid}-{tableid}-{viewid}-search");
    if form_json.search.is_empty() {
        session.remove(&ctx_search);
    } else {
        session.insert(&ctx_search, &form_json.search).unwrap();
    }

    let rest = Rest {
        response: "ok".to_string(),
        message: format!("Search {}", &form_json.search),
    };
    HttpResponse::Ok().json(rest)

}

#[derive(Serialize)]
struct Rest {
    response: String,
    message: String,
}