//! Ouverture d'une view
//!
use crate::{
    cruder::{
        list::crud_list,
        sqler::{kerdata, kerlite},
    },
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
    Result
};
use actix_web_lab::respond::Html;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    // collections::HashMap,
    sync::atomic::Ordering,
};

use super::get_back;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tr {
    pub style: String,
    pub class: String,
    pub url_open: String,
    pub url_press: String,
    pub record: HashMap<String, Element>,
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

    let mut tx = tera::Context::new();
    let mut messages: Vec<FlashMessage> = Vec::new();

    // SORT ID DIRECTION
    let ctx_sortid = format!("{appid}-{tableid}-{viewid}-sortid");
    let sortid = match session.get::<String>(&ctx_sortid) {
        Ok(Some(s)) => s,
        Ok(None) => String::new(),
        Err(_) => String::new(),
    };
    tx.insert("sortid", &sortid);
    let ctx_sort_direction = format!("{appid}-{tableid}-{viewid}-sortdirection");
    let sortdirection = match session.get::<String>(&ctx_sort_direction) {
        Ok(Some(s)) => s,
        Ok(None) => String::new(),
        Err(_) => String::new(),
    };
    tx.insert("sortdirection", &sortdirection);

    // LECTURE DES DONNEES DE LA VUE
    let records = match crud_list(
        &session,
        &data.db,
        &data.dblite,
        app,
        &tableid,
        &viewid,
        &"",
    )
    .await
    {
        Ok(recs) => recs,
        Err(e) => {
            messages.push(FlashMessage::error(format!("{e:?}").as_str()));
            Vec::new()
        }
    };
    // CUMUL DES COLONNES
    tx.insert("sums", &records.last());

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
                if !view.form_view.is_empty() {
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
    tx.insert("trs", &trs);

    // FILTERS avec les ELEMENTS
    let mut filters: Vec<Element> = Vec::new();
    for key in &view.filters {
        let session_key = format!("{appid}-{tableid}-{viewid}-filter-{key}");
        // Duplication des éléments de la vue
        for vel in &view.velements {
            if key == &vel.elid {
                let mut element = vel.clone();
                if !element.items_sql.is_empty() {
                    match kerdata(&data.db, &element.items_sql).await {
                        Ok(r) => {
                            element.items = r;
                        }
                        Err(_) => {}
                    };
                }
                if let Some(filter_value) = session.get::<String>(&session_key).unwrap() {
                    element.value = filter_value.clone();
                }
                filters.push(element);
            }
        }
    }

    // SEARCH
    let search_key = format!("{appid}-{tableid}-{viewid}-search");
    if let Some(search) = session.get::<String>(&search_key).unwrap() {
        tx.insert("search", &search);
    } else {
        tx.insert("search", &"");
    }

    // FLASH
    if let Some(flash) = get_flash(&session)? {
        messages.push(flash);
    }
    clear_flash(&session);

    tx.insert("messages", &messages);
    tx.insert("portail", unsafe { &(*ptr).portail });
    tx.insert("application", &app);
    tx.insert("table", &table);
    tx.insert("view", &view);
    tx.insert("appid", &appid);
    tx.insert("tableid", &tableid);
    tx.insert("viewid", &viewid);
    tx.insert("key", &table.setting.key);
    tx.insert("filters", &filters);
    tx.insert("back", &get_back(&session));

    let html = data.template.render("tpl_view.html", &tx).unwrap();

    Ok(Html(html))
}
