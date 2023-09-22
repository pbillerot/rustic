//! Ouverture d'une view
//!
// use std::collections::HashMap;

use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{ web, HttpResponse };
use actix_web::web::Path;
use serde::Serialize;

// #[post("/filter/{appid}/{tableid}/{viewid}")]
pub async fn filter(
    path: Path<(String, String, String)>,
    web::Form(form_posted): web::Form<Vec<(String, String)>>,
    session: Session,
) -> HttpResponse {

    let (appid, tableid, viewid) = path.into_inner();

    // un 1er tour pour savoir si un reset sur les filtres est demandé
    let mut resetfilter = false;
    for (k, v) in &form_posted {
        if k == "resetfilter" && v == "reset" {
            resetfilter = true;
        }
    }
    // enregistrement des filtres dans la session
    for (k, v) in &form_posted {
        let ctx_filter = format!("{appid}-{tableid}-{viewid}-filter-{k}");
        if resetfilter {
            session.remove(&ctx_filter);
        } else {
            session.insert(&ctx_filter, &v).unwrap();
        }
    }

    HttpResponse::SeeOther()
    .insert_header((LOCATION, format!("/view/{appid}/{tableid}/{viewid}")))
    .finish()

}

#[derive(Serialize)]
struct Rest {
    response: String,
    message: String,
}