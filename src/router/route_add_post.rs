//! Ouverture d'une view
//!
//!
use super::get_back;
use crate::cruder::insert::crud_insert;
use crate::middler::set_flash;
use crate::AppState;
use crate::middler::flash::FlashMessage;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use actix_web::http::header::LOCATION;
use actix_web::web::Path;
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};

// #[post("/update/{appid}/{tableid}/{viewid}/{formid}/{id}")]
pub async fn add_post(
    path: Path<(String, String, String, String)>,
    web::Form(form_posted): web::Form<Vec<(String, String)>>,
    data: web::Data<AppState>,
    session: Session,
) -> HttpResponse {


    let (appid, tableid, viewid, formid,)= path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let application = apps.get(&appid).unwrap();
    let table = application.tables.get(&tableid).unwrap();
    // let view = table.views.get(&viewid).unwrap();
    let form = table.forms.get(&formid).unwrap();

    let mut location = String::new();
    match crud_insert(&data.db, &data.dblite, &table, &form.velements, &form_posted).await {
            Ok(s) => {
                set_flash(&session, FlashMessage::success(&s)).unwrap();
                location.push_str(get_back(&session).as_str())
            },
            Err(e) => {
                set_flash(&session, FlashMessage::error(format!("{e:?}").as_str())).unwrap();
                location.push_str(format!("/add/{appid}/{tableid}/{viewid}/{formid}").as_str());
            }
        };
        HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()

}

