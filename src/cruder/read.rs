///
/// CRUD sur les données
///
use sqlx::{Pool, Postgres, Sqlite, Error};

use crate::lexicer::lex_application::Application;
use crate::lexicer::lex_table::{Element, Table};
use std::collections::HashMap;

use super::records_elements;
///
/// - Lecture des données de la table
///
#[allow(unused_variables)]
/// Retourne une table d'éléments en fonction des éléments fournis dans le vecteur velements
pub async fn crud_read(
    pooldb: &Pool<Postgres>,
    poolite: &Pool<Sqlite>,
    application: &Application,
    table: &Table, // le lexique de l'application
    velements: &Vec<Element>,
    id: &str,
) -> Result<Vec<HashMap<String, Element>>, Error> {
    // construction de l'ordre sql
    let mut sql = "SELECT ".to_string();
    // on prend les colonnes définies dans la view.velements
    let mut bstart = true;
    let mut joins: Vec<String> = Vec::new();
    for element in velements {
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
            sql.push_str(
                format!("{}.{} as {}", &table.tableid, &element.elid, &element.elid).as_str(),
            );
        }
    }
    sql.push_str(format!(" FROM {}", &table.tableid).as_str());
    if !joins.is_empty() {
        for join in joins {
            sql.push_str(format!(" {}", &join).as_str());
        }
    }
    sql.push_str(format!(" WHERE ( {} = '{}' )", &table.setting.key, id).as_str());

    let records = records_elements(
        pooldb,
        poolite,
        &sql,
        &application,
        velements,
        table,
    ).await?;

    Ok(records)
}

