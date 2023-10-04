//! Ouverture d'une view
//!
// use crate::sqlic::sql_utils::querlite;
use crate::{
    cruder::read::crud_read,
    lexicer::{lex_table::Element, macelement},
    middler::{clear_flash, flash::FlashMessage, get_flash},
    // lexic::lex_table::{self, Element},
    AppState,
};
use actix_session::Session;
use actix_web::{
    // get,
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
    collections::HashMap,
    // collections::HashMap,
    sync::atomic::Ordering,
};

use super::{get_back, view_table::Tview};

// #[get("/form/{appid}/{tableid}/{viewid}/{formid}/{id}")]
pub async fn form(
    path: Path<(String, String, String, String, String)>,
    data: web::Data<AppState>,
    session: Session,
) -> Result<impl Responder> {
    let (appid, tableid, viewid, formid, id) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let application = apps.get(&appid).unwrap();
    let table = application.tables.get(&tableid).unwrap();
    let view = table.views.get(&viewid).unwrap();
    let form = table.forms.get(&formid).unwrap();

    let mut context = tera::Context::new();
    let mut messages: Vec<FlashMessage> = Vec::new();

    let record = match crud_read(
        &data.db,
        &data.dblite,
        application,
        table,
        &form.velements,
        &id,
    )
    .await
    {
        Ok(mut records) => match records.pop() {
            Some(record) => record,
            None => {
                let rec: HashMap<String, Element> = HashMap::new();
                rec
            }
        },
        Err(e) => {
            messages.push(FlashMessage::error(format!("{e:?}").as_str()));
            let rec: HashMap<String, Element> = HashMap::new();
            rec
        }
    };

    // chargement des éléments de type tview dans tvs
    let mut tvs: HashMap<String, Tview> = HashMap::new();
    for vel in &form.velements {
        if vel.type_element == "view" {
            let filter = macelement(&vel.params.where_sql, &record);
            match Tview::new(
              &application,
              &vel.params.tableid,
              &vel.params.viewid,
              &filter,
              &session,
              &data.db,
              &data.dblite,
          )
          .await {
            Ok(tv) => {
              tvs.insert(vel.params.viewid.clone(), tv);
            },
            Err(e) => {
              messages.push(FlashMessage::error(format!("{e:?}").as_str()));
            }
          };
        }
    }

    if let Some(flash) = get_flash(&session)? {
        messages.push(flash);
    }
    clear_flash(&session);

    context.insert("messages", &messages);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &application);
    context.insert("table", &table);
    context.insert("view", &view);
    context.insert("form", &form);
    context.insert("appid", &appid);
    context.insert("tableid", &tableid);
    context.insert("viewid", &viewid);
    context.insert("formid", &formid);
    context.insert("id", &id);
    context.insert("key", &table.setting.key);
    context.insert("back", &get_back(&session));
    context.insert("tvs", &tvs);
    context.insert("record", &record);

    let html = data.template.render("tpl_form.html", &context).unwrap();

    Ok(Html(html))
}
