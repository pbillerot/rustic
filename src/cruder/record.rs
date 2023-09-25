use std::collections::HashMap;

use sqlx::{Pool, Postgres, Sqlite};

use crate::lexicer::{lex_application::Application, lex_table::{Element, Table}};

use super::sqler::rows_to_vmap;


/// chargement d'un tableau d'éléments correspondant à la requête sql
/// ou d'un tableau minimum (cas d'un ajout)
pub async fn records_elements(
    pooldb: &Pool<Postgres>,
    poolite: &Pool<Sqlite>,
    sql: &str,
    application: &Application,
    velements: &Vec<Element>,
    table: &Table,
) -> Result<Vec<HashMap<String, Element>>, String> {

    let mut records = Vec::new();
    let mut vrows: Vec<HashMap<String, String>> = Vec::new();
    if sql.is_empty() {
        // construction d'un vrows vide
        let mut hcols: HashMap<String, String> = HashMap::new();
        for vel in velements {
            hcols.insert(vel.elid.clone(), "".to_string());
        }
        vrows.push(hcols);
    } else {
        // Chargement des enregistrements dans un tableau de valeur
        let rows = match sqlx::query(&sql).fetch_all(pooldb).await {
            Ok(r) => r,
            Err(e) => {
                let msg = format!("{sql:?} : {e:?}");
                log::error!("{msg}");
                return Err(msg)
            }
        };
        vrows = rows_to_vmap(rows);
    }
    // Initialisation de la somme des colonnes avec cumul
    let mut hsum: HashMap<String, f64> = HashMap::new();
    for vel in velements {
        hsum.insert(vel.elid.clone(), 0.0);
    }

    let mut irow = 0;
    for hvalue in vrows {
        irow = irow + 1;
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
            if vel.elid == table.setting.key && !sql.is_empty() {
                element.read_only = true;
                element.required = false;
            }
            if vel.type_element == "counter" {
                element.read_only = true;
                element.required = false;
            }
            // Calcul des colonnes avec cumul
            if !vel.hide && vel.with_sum {
                // let v = &element.value.clone();
                match &element.value.parse::<f64>() {
                    Ok(val) => {
                        let sum = hsum.get(&vel.elid).unwrap() + val;
                        hsum.insert(element.elid.clone(), sum);
                    },
                    Err(_) => {
                        // pas de cumul
                    }
                };
                element.sum = hsum.get(&vel.elid).unwrap().clone();
            }
            hel.insert(element.elid.clone(), element);

        }
        records.push(hel);
    }
    Ok(records)
}

