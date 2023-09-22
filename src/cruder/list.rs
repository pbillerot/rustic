use actix_session::Session;
///
/// CRUD sur les données
///
use sqlx::{Pool, Postgres, Sqlite};

use crate::lexicer::lex_application::Application;
use crate::lexicer::lex_table::Element;
use std::collections::HashMap;

use super::record::records_elements;

///
/// - Lecture des données de la table
///
pub async fn crud_list(
    session: &Session,
    pooldb: &Pool<Postgres>,
    poolite: &Pool<Sqlite>,
    application: &Application, // le lexique de l'application
    tableid: &str,
    viewid: &str,
    id: &str,
) -> Result<Vec<HashMap<String, Element>>, String> {
    // construction de l'ordre sql
    let mut sql = "SELECT ".to_string();
    // on prend les colonnes définies dans la view.velements
    let appid = &application.appid;
    let table = application.tables.get(tableid).unwrap();
    let view = table.views.get(viewid).unwrap();
    let mut bstart = true;
    let mut joins: Vec<String> = Vec::new();

    // SORT ID DIRECTION
    let ctx_sortid = format!("{appid}-{tableid}-{viewid}-sortid");
    let sortid = match session.get::<String>(&ctx_sortid) {
        Ok(Some(s)) => s,
        Ok(None) => String::new(),
        Err(_) => String::new()
    };
    let ctx_sort_direction = format!("{appid}-{tableid}-{viewid}-sortdirection");
    let sortdirection = match session.get::<String>(&ctx_sort_direction) {
        Ok(Some(s)) => s,
        Ok(None) => String::new(),
        Err(_) => String::new()
    };

    let mut where_filter = "".to_string();

    for element in &view.velements {
        if element.hide {
            continue;
        }
        if element.elid.starts_with("_") {
            continue;
        }
        if bstart {
            bstart = false;
        } else {
            sql.push_str(", ");
        }
        if !element.jointure.column.is_empty() {
            sql.push_str(format!("{} as {}", &element.jointure.column, &element.elid).as_str());
            joins.push(element.jointure.join.clone());
        } else {
            sql.push_str(format!("{}.{} as {}", &tableid, &element.elid, &element.elid).as_str());
        }
        // FILTRES DANS LA VUE
        // les valeurs ont été mémorisée dans la session
        if id.is_empty() {
            for key in &view.filters {
                if key == &element.elid {
                    let key_filter = format!("{appid}-{tableid}-{viewid}-filter-{key}");
                    if let Some(filter_value) = session.get::<String>(&key_filter).unwrap() {
                        if filter_value.is_empty() {
                            continue;
                        }
                        if ! where_filter.is_empty() {
                            where_filter.push_str(" AND ");
                        }
                        if element.type_element == "list" || element.type_element == "radio" || element.type_element == "tag" {
                            // where_filter.push_str(format!("'{filter_value}' IN {tableid}.{key}").as_str());
                            where_filter.push_str(format!("lower({tableid}.{key}) LIKE '%{}%'",
                            filter_value.to_lowercase()).as_str());
                } else {
                            if element.jointure.column.is_empty() {
                                if element.type_element == "date" || element.type_element == "number" || element.type_element == "amount" || element.type_element == "checkbox" {
                                    where_filter.push_str(format!("cast({tableid}.{key} as varchar) LIKE '%{}%'",
                                    filter_value.to_lowercase()).as_str());
                                } else {
                                    where_filter.push_str(format!("lower({}) LIKE '%{}%'",
                                    element.jointure.column,
                                    filter_value.to_lowercase()).as_str());
                                }
                            } else {
                                where_filter.push_str(format!("lower({}) LIKE '%{}%'",
                                    element.jointure.column,
                                    filter_value.to_lowercase()).as_str());
                            }
                        }
                    }
                }
            }
        }
    }
    sql.push_str(format!(" FROM {}", &tableid).as_str());
    if !joins.is_empty() {
        for join in joins {
            sql.push_str(format!(" {}", &join).as_str());
        }
    }
    // Cas id valorisé ou non
    if id.is_empty() {
        let mut w = String::new();
        if !view.where_sql.is_empty() {
            w.push_str(format!(" WHERE ( {} )", &view.where_sql).as_str());
        }
        if !view.where_sql.is_empty() {
            w.push_str(format!(" WHERE ( {} )", &view.where_sql).as_str());
        }
        if !where_filter.is_empty() {
            if w.is_empty() {
                w.push_str(" WHERE ");
            }
            w.push_str(format!("{}", &where_filter).as_str());
        }
        if !w.is_empty() {
            sql.push_str(&w);
        }

        if sortid.is_empty() {
            if !view.order_by.is_empty() {
                sql.push_str(format!(" ORDER BY {}", &view.order_by).as_str());
            }
        } else {
            if sortdirection == "descending" {
                sql.push_str(format!(" ORDER BY {} DESC", sortid).as_str());
            } else {
                sql.push_str(format!(" ORDER BY {}", sortid ).as_str());
            }
        }
        if !application.limit_sql.is_empty() {
            sql.push_str(format!(" LIMIT {}", &application.limit_sql).as_str());
        } else {
            sql.push_str(" LIMIT 50");
        }
    } else {
        sql.push_str(format!(" WHERE ( {} = '{}' )", &table.setting.key, id).as_str());
    }

    let records = match records_elements(
        pooldb,
        poolite,
        &sql,
        &application,
        &view.velements,
        table,
    ).await {
        Ok(r) => r,
        Err(e) => {
            let msg = format!("{sql:?} : {e:?}");
            log::error!("{msg}");
            return Err(msg)
        }
    };
    Ok(records)
}

