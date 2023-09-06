use std::collections::HashMap;

use sqlx::{Postgres, Pool, Sqlite, Error};

use crate::lexicer::{lex_application::Application, lex_table::{Element, Table}};

use self::sqler::rows_to_vmap;

pub mod sqler;

pub mod insert;
pub mod list;
pub mod read;
pub mod update;

/// chargement d'un tableau d'éléments correspondant à la requête sql
/// ou d'un tableau minimum (cas d'un ajout)
pub async fn records_elements(
    pooldb: &Pool<Postgres>,
    poolite: &Pool<Sqlite>,
    sql: &str,
    application: &Application,
    velements: &Vec<Element>,
    table: &Table,
) -> Result<Vec<HashMap<String, Element>>, Error> {

    let mut records = Vec::new();
    let mut vrows: Vec<HashMap<String, String>> = Vec::new();
    if sql.is_empty() {
        // construction d'un vrows vide
        for vel in velements {
            let mut hvalue: HashMap<String, String> = HashMap::new();
            hvalue.insert(vel.elid.clone(), vel.value.clone());
            vrows.push(hvalue);
        }
    } else {
        let rows = sqlx::query(&sql).fetch_all(pooldb).await?;
        // Chargement des enregistrements dans un tableau de valeur
        vrows = rows_to_vmap(rows);
    }

    for hvalue in vrows {
        // récup de la valeur de la clé de l'enregistrement
        let default = "".to_string();
        let key_value = match hvalue.get(&table.setting.key) {
            Some(v) => v,
            None => &default, // key_value non définie dans le cas d'un ajout
        };
        // init de la table des éléments
        let mut hel: HashMap<String, Element> = HashMap::new();
        // 1er tour pour calculer la value
        // TODO
        for vel in velements {
            let mut element = vel.clone();
            element.compute_value(poolite, &hvalue).await?;
            element.key_value = key_value.clone();
            hel.insert(vel.elid.clone(), element);
        }
        // 2ème tour pour calculer les propriétés
        // on reconstruit un hvalue actualisé avec les values
        let mut hvalue_computed = HashMap::new();
        for vel in velements {
            hvalue_computed.insert(vel.elid.clone(), hel.get(&vel.elid).unwrap().value.clone());
        }
        // ajout des paramètres de l'application pour macrolex
        for (key, param) in &application.parameters {
            hvalue_computed.insert(key.to_string(), param.to_string());
        }
        // calcul des autres propriétés
        for vel in velements {
            let mut element = hel.get(&vel.elid).unwrap().clone();
            element.compute_prop(pooldb, poolite, &hvalue_computed).await?;
            if vel.elid == table.setting.key {
                element.read_only = true;
            }
            hel.insert(element.elid.clone(), element);
        }
        records.push(hel);
    }

    Ok(records)
}

