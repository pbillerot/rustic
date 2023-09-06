//! Ouverture d'une view
//!
use crate::AppState;
use crate::cruder::update::crud_update;
use actix_web::{HttpResponse, web};
use actix_web::http::header::LOCATION;
use actix_web::web::Path;
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};
// #[post("/update/{appid}/{tableid}/{viewid}/{formid}/{id}")]
pub async fn edit_post(
    path: Path<(String, String, String, String, String)>,
    web::Form(form_posted): web::Form<Vec<(String, String)>>,
    data: web::Data<AppState>,
    _flash: IncomingFlashMessages
) -> HttpResponse {

    let (appid, tableid, viewid, formid, id) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let application = apps.get(&appid).unwrap();
    let table = application.tables.get(&tableid).unwrap();
    // let view = table.views.get(&viewid).unwrap();
    let form = table.forms.get(&formid).unwrap();

    let result = crud_update(&data.db, &data.dblite, &table, &form.velements, &id, &form_posted).await;

    FlashMessage::info("route_edit_post").send();
    FlashMessage::error("ya un bug!!").send();

    if result {
        HttpResponse::SeeOther()
        .insert_header((LOCATION, format!("/form/{appid}/{tableid}/{viewid}/{formid}/{id}")))
        .finish()
    } else {
        HttpResponse::SeeOther()
        .insert_header((LOCATION, format!("/edit/{appid}/{tableid}/{viewid}/{formid}/{id}")))
        .finish()
    }
}

