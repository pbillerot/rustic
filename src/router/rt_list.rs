//! Ouverture d'une view
//!
use crate::cruder::sql_crud::crud_read_all;
// use crate::sqlic::sql_utils::querlite;
use crate::{
    // lexic::lex_table::{self, Element},
    AppState,
};
use actix_session::Session;
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
use actix_web_lab::respond::Html;
use std::{
    // collections::HashMap,
    sync::atomic::Ordering
};
use crate::service;

// cuerl http://0.0.0.0:8080/
#[get("/list/{appid}/{tableid}/{viewid}")]
pub async fn list(
    path: Path<(String, String, String)>,
    session: Session,
    data: web::Data<AppState>,
    // msg: Option<ReqData<servic::sr_data::Msg>>,
) -> Result<impl Responder> {
    // log::info!("Session {:?} {:?} {:?}", session.status(), session.entries(), path);
    let mut messages = session.get::<Vec<service::Message>>("messages")?.unwrap();
    messages.push(service::Message::new("list:Tout va bien", service::MESSAGE_LEVEL_INFO));

    let (appid, tableid, viewid) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let app = apps.get(&appid).unwrap();

    let mut rowsel = Vec::new();

    crud_read_all(&data.db, &data.dblite, app, &tableid, &viewid ,&"".to_string(), &mut rowsel, &mut messages).await;

    // TESTS SQLITE
    // let _reslite: HashMap<String, String> = match querlite(&data.dblite,
    //     "
    //     select 'text' as mytext,
    //     122 as myinteger,
    //     12.23 as myfloat,
    //     datetime('now','localtime') as localtime,
    //     CURRENT_TIMESTAMP as current,
    //     datetime(CURRENT_TIMESTAMP,'localtime') as created_at,
    //     strftime('%s','now') as strftime
    //     "
    // ).await {
    //     Ok(t) => t,
    //     Err(e) => {
    //         log::error!("{:?}", e);
    //         HashMap::new()
    //     }
    // };
    // let messages = Vec::new();
    // for message in msg_flash.iter() {
    //     message.push(&message.level(),&message.content());
    // }

    let mut context = tera::Context::new();
    context.insert("messages", &messages);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &app);
    context.insert("tableid", &tableid);
    context.insert("viewid", &viewid);
    context.insert("rowsel", &rowsel);
    let html = data.template.render("tpl_list.html", &context).unwrap();

    Ok(Html(html))
}
