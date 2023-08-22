//! Ouverture d'une view
//!
use actix_web::{
    get,
    // delete,
    // post,
    // HttpResponse,
    web,
    web::Path,
    Responder,
    Result,
};
use actix_session::Session;
use actix_web_lab::respond::Html;
use std::sync::atomic::Ordering;
use crate::AppState;

// cuerl http://0.0.0.0:8080/
#[get("/list/{appid}/{tableid}/{viewid}")]
pub async fn list(
    path: Path<(String, String, String)>,
    _session: Session,
    data: web::Data<AppState>,
    // msg: Option<ReqData<servic::sr_data::Msg>>,
) -> Result<impl Responder> {
    // log::info!("Session {:?} {:?} {:?}", session.status(), session.entries(), path);
    let (appid, tableid, viewid) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe {&(*ptr).applications.clone()};
    let app = apps.get(&appid).unwrap();

    let mut context = tera::Context::new();
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &app);
    context.insert("tableid", &tableid);
    context.insert("viewid", &viewid);
    let html = data.template.render("tpl_list.html", &context).unwrap();

    Ok(Html(html))
}
