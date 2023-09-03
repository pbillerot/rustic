///
/// CRUD sur les données
///
use sqlx::{Pool, Postgres, Sqlite};

use crate::cruder::sqler::rows_to_vmap;
use crate::lexicer::lex_application::Application;
use crate::lexicer::lex_table::{Element, Table};
use crate::router::{Message, MESSAGE_LEVEL_ERROR, MESSAGE_LEVEL_INFO};
use std::collections::HashMap;
///
/// - Lecture des données de la table
///
#[allow(unused_variables)]
pub async fn crud_list(
    pooldb: &Pool<Postgres>,
    poolite: &Pool<Sqlite>,
    application: &Application, // le lexique de l'application
    tableid: &str,
    viewid: &str,
    id: &str,
    filter: &str, // TODO: voir si utile
    messages: &mut Vec<Message>,
) -> Vec<HashMap<String, Element>> {
    // construction de l'ordre sql
    let mut sql = "SELECT ".to_string();
    // on prend les colonnes définies dans la view.velements
    let table = application.tables.get(tableid).unwrap();
    let view = table.views.get(viewid).unwrap();
    let mut bstart = true;
    let mut joins: Vec<String> = Vec::new();
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
        if !filter.is_empty() {
            sql.push_str(format!(" AND ( {} )", &filter).as_str());
        }
        if !view.order_by.is_empty() {
            sql.push_str(format!(" ORDER BY {}", &view.order_by).as_str());
        }
        if !application.limit_sql.is_empty() {
            sql.push_str(format!(" LIMIT {}", &application.limit_sql).as_str());
        } else {
            sql.push_str(" LIMIT 50");
        }
    } else {
        sql.push_str(format!(" WHERE ( {} = '{}' )", &table.setting.key, id).as_str());
    }

    let records = records_elements(
        pooldb,
        poolite,
        &sql,
        &application,
        &view.velements,
        table,
        messages,
    )
    .await;
    records
}

/// Retourne une table d'éléments en fonction des éléments fournis dans le vecteur velements
pub async fn crud_read(
    pooldb: &Pool<Postgres>,
    poolite: &Pool<Sqlite>,
    application: &Application,
    table: &Table, // le lexique de l'application
    velements: &Vec<Element>,
    id: &str,
    messages: &mut Vec<Message>,
) -> Vec<HashMap<String, Element>> {
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
        messages,
    )
    .await;
    records
}

/// Retourne une table d'éléments en fonction des éléments fournis dans le vecteur velements
pub async fn crud_update(
    pooldb: &Pool<Postgres>,
    poolite: &Pool<Sqlite>,
    table: &Table, // le lexique de l'application
    velements: &Vec<Element>,
    id: &str,
    form_posted: &Vec<(String, String)>,
    messages: &mut Vec<Message>,
) -> bool {


    // Transformation de form_posted Vec(key, value) en Hashtable
    // sachant key ne sera unique pour les "select multiple" === tag
    let mut hvalue: HashMap<String, String> = HashMap::new();
    let mut key = String::new();
    let mut val = String::new();
    for (k, v) in form_posted {
        if key.is_empty() {
            key = k.clone();
        }
        if &key == k {
            if !val.is_empty() {
                val.push_str(", ");
            }
            val.push_str(v);
        } else {
            hvalue.insert(key.clone(), val.clone());
            key = k.clone();
            val.clear();
            val.push_str(v);
        }
    }
    hvalue.insert(key, val.clone());

    // valorisation des éléments du formulaire avec les champs du formulaire
    // construction de l'order sql
    // construction de l'ordre sql de mise à jour
    let mut sql = "UPDATE ".to_string();
    sql.push_str(&table.tableid);
    sql.push_str(" SET ");
    // on prend les colonnes définies dans la view.velements
    let mut bstart = true;
    for vel in velements {
        let mut element = vel.clone();
        match vel.type_element.as_str() {
            "checkbox" => { // 'on' si coché et rien si non coché
                element.value = match hvalue.get(&vel.elid) {
                    Some(_) => "1".to_string(),
                    None => "0".to_string(),
                };
            }
            _ => {
                element.value = match hvalue.get(&vel.elid) {
                    Some(v) => v.to_string(),
                    None => String::new(),
                };
            }
        }
        element.compute_prop(pooldb, poolite, &hvalue, messages).await;
        element.key_value = id.to_string();
        // construction du sql
        if element.elid == table.setting.key {
            continue;
        }
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
        sql.push_str(format!("{} = '{}'", &element.elid, element.value).as_str());
    }
    sql.push_str(format!(" WHERE ( {} = '{}' )", &table.setting.key, &id).as_str());

    let result = match sqlx::query(&sql).execute(pooldb).await {
        Ok(_) => {
            messages.push(Message::new(
                format!("Mise à jour ok").as_str(),
                MESSAGE_LEVEL_INFO,
            ));
            true
        }
        Err(e) => {
            messages.push(Message::new(
                format!("{:?}", &e).as_str(),
                MESSAGE_LEVEL_ERROR,
            ));
            false
        }
    };

    result
}

/// chargement d'un tableau d'éléments correspondant à la requête sql
pub async fn records_elements(
    pooldb: &Pool<Postgres>,
    poolite: &Pool<Sqlite>,
    sql: &str,
    application: &Application,
    velements: &Vec<Element>,
    table: &Table,
    messages: &mut Vec<Message>,
) -> Vec<HashMap<String, Element>> {

    let mut records = Vec::new();

    let rows = match sqlx::query(&sql).fetch_all(pooldb).await {
        Ok(t) => t,
        Err(e) => {
            messages.push(Message::new(
                format!("{:?}", &e).as_str(),
                MESSAGE_LEVEL_ERROR,
            ));
            Vec::new()
        }
    };
    // Chargement des enregistrements dans un tableau de valeur
    let vrows = rows_to_vmap(rows);
    for hvalue in vrows {
        // récup de la valeur de la clé de l'enregistrement
        let key_value = hvalue.get(&table.setting.key).unwrap();
        // init de la table des éléments
        let mut hel: HashMap<String, Element> = HashMap::new();
        // 1er tour pour calculer la value
        // TODO
        for vel in velements {
            let mut element = vel.clone();
            element.compute_value(poolite, &hvalue, messages).await;
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
            element
                .compute_prop(pooldb, poolite, &hvalue_computed, messages)
                .await;
            hel.insert(element.elid.clone(), element);
        }
        records.push(hel);
    }

    records
}
