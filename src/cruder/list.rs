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
    let table = application.tables.get(tableid).unwrap();
    let view = table.views.get(viewid).unwrap();
    let mut bstart = true;
    let mut joins: Vec<String> = Vec::new();
    // SORT SORTDIRECTION

    let ctx_sortid = format!("{}-{tableid}-{viewid}-sortid", &application.appid);
    let sortid = match session.get::<String>(&ctx_sortid) {
        Ok(Some(s)) => s,
        Ok(None) => String::new(),
        Err(_) => String::new(),
    };
    let ctx_sort_direction = format!("{}-{tableid}-{viewid}-sortdirection", &application.appid);
    let sortdirection = match session.get::<String>(&ctx_sort_direction) {
        Ok(Some(s)) => s,
        Ok(None) => String::new(),
        Err(_) => String::new(),
    };

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
    }
    sql.push_str(format!(" FROM {}", &tableid).as_str());
    if !joins.is_empty() {
        for join in joins {
            sql.push_str(format!(" {}", &join).as_str());
        }
    }
    // Cas id valorisé ou non
    if id.is_empty() {
        if !view.where_sql.is_empty() {
            sql.push_str(format!(" WHERE ( {} )", &view.where_sql).as_str());
        }
        if !view.where_sql.is_empty() {
            sql.push_str(format!(" WHERE ( {} )", &view.where_sql).as_str());
        }
        if sortid.is_empty() {
            if !view.order_by.is_empty() {
                sql.push_str(format!(" ORDER BY {}", &view.order_by).as_str());
            }
        } else {
            if sortdirection == "descending" {
                sql.push_str(format!(" ORDER BY {} DESC", &sortid).as_str());
            } else {
                sql.push_str(format!(" ORDER BY {}", &sortid ).as_str());
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

