//! Ouverture d'une view
//!
use crate::cruder::delete::crud_delete;
use crate::middler::set_flash;
use crate::AppState;
use crate::middler::flash::FlashMessage;
use actix_session::Session;
use actix_web::{HttpResponse, HttpRequest, web};
use actix_web::http::header::LOCATION;
use actix_web::web::Path;
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};

use super::get_back;

// #[post("/delete/{appid}/{tableid}/{viewid}/{id}")]
pub async fn delete_post(
    path: Path<(String, String, String, String)>,
    data: web::Data<AppState>,
    session: Session,
    req: HttpRequest,
) -> HttpResponse {


    let (appid, tableid, viewid, id)= path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let application = apps.get(&appid).unwrap();
    let table = application.tables.get(&tableid).unwrap();

    let mut location = String::new();
    match crud_delete(&data.db, &table, &id).await {
            Ok(s) => {
                set_flash(&session, FlashMessage::success(&s)).unwrap();
                location.push_str(get_back(&req, &session).as_str())
            },
            Err(e) => {
                set_flash(&session, FlashMessage::error(format!("{e:?}").as_str())).unwrap();
                location.push_str(format!("/view/{appid}/{tableid}/{viewid}").as_str());
            }
        };
    HttpResponse::SeeOther()
    .insert_header((LOCATION, location))
    .finish()

}

