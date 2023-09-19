//! Ouverture d'une view
//!
use crate::{
    cruder::{list::crud_list, sqler::kerlite},
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
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    // collections::HashMap,
    sync::atomic::Ordering,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Tr {
    style: String,
    class: String,
    url_open: String,
    url_press: String,
    record: HashMap<String, Element>,
}

// cuerl http://0.0.0.0:8080/
// #[get("/view/{appid}/{tableid}/{viewid}")]
pub async fn view(
    path: Path<(String, String, String)>,
    data: web::Data<AppState>,
    session: Session,
) -> Result<impl Responder> {
    let (appid, tableid, viewid) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let app = apps.get(&appid).unwrap();
    let table = app.tables.get(&tableid).unwrap();
    let view = table.views.get(&viewid).unwrap();

    let mut context = tera::Context::new();
    let mut messages: Vec<FlashMessage> = Vec::new();

    // LECTURE DES DONNEES DE LA VUE
    let records = match crud_list(&session, &data.db, &data.dblite, app, &tableid, &viewid, "").await {
        Ok(recs) => recs,
        Err(e) => {
            messages.push(FlashMessage::error(format!("{e:?}").as_str()));
            Vec::new()
        }
    };
    // CUMUL DES COLONNES
    context.insert("sums", &records.last());

    // TRS alimentation de la structure Trs avec les éléments et les données lues dans la table
    let mut trs: Vec<Tr> = Vec::new();
    for record in records {
        let tr = Tr {
            style: {
                if !view.style_sqlite.is_empty() {
                    match kerlite(&data.dblite, &macelement(&view.style_sqlite, &record)).await {
                        Ok(r) => r,
                        Err(e) => {
                            messages.push(FlashMessage::error(format!("{e:?}").as_str()));
                            String::new()
                        }
                    }
                } else {
                    String::new()
                }
            },
            class: {
                if !view.class_sqlite.is_empty() {
                    match kerlite(&data.dblite, &macelement(&view.class_sqlite, &record)).await {
                        Ok(r) => r,
                        Err(e) => {
                            messages.push(FlashMessage::error(format!("{e:?}").as_str()));
                            String::new()
                        }
                    }
                } else {
                    String::new()
                }
            },
            url_open: {
                if !view.form_view.is_empty() || view.deletable {
                    format!(
                        "/form/{appid}/{tableid}/{viewid}/{}/{}",
                        view.form_view, record[&table.setting.key].key_value
                    )
                } else if !view.form_edit.is_empty() {
                    format!(
                        "/edit/{appid}/{tableid}/{viewid}/{}/{}",
                        view.form_edit, record[&table.setting.key].key_value
                    )
                } else {
                    String::new()
                }
            },
            url_press: {
                if !view.action_press.sql.is_empty() {
                    format!(
                        "/action/{appid}/{tableid}/{viewid}/{}",
                        record[&table.setting.key].key_value
                    )
                } else {
                    String::new()
                }
            },
            record: record,
        };
        trs.push(tr);
        if messages.len() > 0 {
            break;
        }
    }
    context.insert("trs", &trs);

    // SORT ID DIRECTION
    let ctx_sortid = format!("{appid}-{tableid}-{viewid}-sortid");
    if let Some(sortid) = session.get::<String>(&ctx_sortid).unwrap() {
        context.insert("sortid", &sortid);
    } else {
        context.insert("sortid", &"");
    }
    let ctx_sort_direction = format!("{appid}-{tableid}-{viewid}-sortdirection");
    if let Some(sortdirection) = session.get::<String>(&ctx_sort_direction).unwrap() {
        context.insert("sortdirection", &sortdirection);
    } else {
        context.insert("sortdirection", &"");
    }

    // FLASH
    if let Some(flash) = get_flash(&session)? {
        messages.push(flash);
    }
    clear_flash(&session);

    context.insert("messages", &messages);
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &app);
    context.insert("table", &table);
    context.insert("view", &view);
    context.insert("appid", &appid);
    context.insert("tableid", &tableid);
    context.insert("viewid", &viewid);
    context.insert("key", &table.setting.key);
    context.insert("search", &""); // TODO

    let html = data.template.render("tpl_view.html", &context).unwrap();

    Ok(Html(html))
}
