//! Ouverture d'une view
//!
use crate::AppState;
use crate::cruder::update::crud_update;
use crate::middler::flash::FlashMessage;
use crate::middler::set_flash;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use actix_web::http::header::LOCATION;
use actix_web::web::Path;
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};
// #[post("/update/{appid}/{tableid}/{viewid}/{formid}/{id}")]
pub async fn edit_post(
    path: Path<(String, String, String, String, String)>,
    web::Form(form_posted): web::Form<Vec<(String, String)>>,
    data: web::Data<AppState>,
    session: Session,
) -> HttpResponse {

    let (appid, tableid, viewid, formid, id) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let application = apps.get(&appid).unwrap();
    let table = application.tables.get(&tableid).unwrap();
    // let view = table.views.get(&viewid).unwrap();
    let form = table.forms.get(&formid).unwrap();

    let mut location = String::new();
    match crud_update(&data.db, &data.dblite, &table, &form.velements, &id, &form_posted).await {
            Ok(s) => {
                set_flash(&session, FlashMessage::success(&s)).unwrap();
                location.push_str(format!("/form/{appid}/{tableid}/{viewid}/{formid}/{id}").as_str());
            },
            Err(e) => {
                set_flash(&session, FlashMessage::success(format!("{e:?}").as_str())).unwrap();
                location.push_str(format!("/edit/{appid}/{tableid}/{viewid}/{formid}/{id}").as_str());
            }
        };
        HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

