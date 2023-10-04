use std::collections::HashMap;

use actix_session::Session;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Sqlite};

use crate::{
    cruder::{
        list::crud_list,
        sqler::{kerdata, kerlite},
    },
    lexicer::{
        lex_application::Application,
        lex_table::{Element, View},
        macelement,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tr {
    pub style: String,
    pub class: String,
    pub url_open: String,
    pub url_press: String,
    pub record: HashMap<String, Element>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tview {
    pub search: String,
    pub sortid: String,
    pub sortdirection: String,
    pub view: View,
    pub trs: Vec<Tr>,
    pub filters: Vec<Element>,
}

impl Tview {
    pub async fn new(
        application: &Application,
        tableid: &str,
        viewid: &str,
        filter: &str,
        session: &Session,
        pooldb: &Pool<Postgres>,
        poolite: &Pool<Sqlite>,
    ) -> Result<Tview, String> {
        let appid = &application.appid;
        let ctx_sortid = format!("{appid}-{tableid}-{viewid}-sortid");
        let ctx_sort_direction = format!("{appid}-{tableid}-{viewid}-sortdirection");
        let search_key = format!("{appid}-{tableid}-{viewid}-search");

        let table = application.tables.get(tableid).unwrap();
        let view = table.views.get(viewid).unwrap();

        // LECTURE DES DONNEES DE LA VUE
        let records = match crud_list(
            &session,
            &pooldb,
            &poolite,
            &application,
            &tableid,
            &viewid,
            &"",
            &filter,
        )
        .await
        {
            Ok(recs) => recs,
            Err(e) => {
              return Err(e)
            }
        };

        let mut tv = Tview {
            search: {
                match session.get::<String>(&search_key) {
                    Ok(Some(s)) => s,
                    Ok(None) => String::new(),
                    Err(_) => String::new(),
                }
            },
            sortid: {
                match session.get::<String>(&ctx_sortid) {
                    Ok(Some(s)) => s,
                    Ok(None) => String::new(),
                    Err(_) => String::new(),
                }
            },
            sortdirection: {
                match session.get::<String>(&ctx_sort_direction) {
                    Ok(Some(s)) => s,
                    Ok(None) => String::new(),
                    Err(_) => String::new(),
                }
            },
            view: view.clone(),
            trs: Vec::new(),
            filters: Vec::new(),
        };
        // TRS alimentation de la structure Trs avec les éléments et les données lues dans la table
        for record in records {
            let tr = Tr {
                style: {
                    if !view.style_sqlite.is_empty() {
                        match kerlite(&poolite, &macelement(&view.style_sqlite, &record)).await {
                            Ok(r) => r,
                            Err(e) => {
                                return Err(e)
                            }
                        }
                    } else {
                        String::new()
                    }
                },
                class: {
                    if !view.class_sqlite.is_empty() {
                        match kerlite(&poolite, &macelement(&view.class_sqlite, &record)).await {
                            Ok(r) => r,
                            Err(e) => {
                              return Err(e)
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
            tv.trs.push(tr);
        }
        // FILTERS avec les ELEMENTS
        for key in &view.filters {
            let session_key = format!("{appid}-{tableid}-{viewid}-filter-{key}");
            // Duplication des éléments de la vue
            for vel in &view.velements {
                if key == &vel.elid {
                    let mut element = vel.clone();
                    if !element.items_sql.is_empty() {
                        match kerdata(&pooldb, &element.items_sql).await {
                            Ok(r) => {
                                element.items = r;
                            }
                            Err(_) => {}
                        };
                    }
                    if let Some(filter_value) = session.get::<String>(&session_key).unwrap() {
                        element.value = filter_value.clone();
                    }
                    tv.filters.push(element);
                }
            }
        }

        Ok(tv)
    }
}
