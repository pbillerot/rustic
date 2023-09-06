use sqlx::Error;
use sqlx::Postgres;
/**
 * Modèles de données
 */
use sqlx::postgres::PgRow;
// use sqlx::sqlite::SqliteRow;
// use sqlx::sqlite::SqliteTypeInfo;
use sqlx::TypeInfo;
use sqlx::Row;
use sqlx::Column;
// use sqlx::ValueRef;
use sqlx::types::chrono::Utc;
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;

/// Requête sqlite qui ne renvoie qu'une seule colonne et une seule ligne
#[allow(dead_code)]
pub async fn kerlite(poollite: &Pool<Sqlite>, sql: &str ) -> Result<String, Error> {
    let row = sqlx::query(sql).fetch_one(poollite).await?;
    let mut valcol = String::new();
    for col in row.columns() {
        // on ne renvoie que la 1ère colonne
        let val = match row.try_get_unchecked::<String, _>(col.ordinal()) {
            Ok(v) => v,
            Err(_) => {
                match row.try_get_unchecked::<i32, _>(col.ordinal()) {
                    Ok(v) => v.to_string(),
                    Err(_) => {
                        match row.try_get_unchecked::<f32, _>(col.ordinal()) {
                            Ok(v) => v.to_string(),
                            Err(_) => {
                                // FlashMessage::error(format!("{:?}", e).as_str());
                                "".to_string()
                            },
                        }
                    }
                }
            }
        };
        valcol.push_str(&val);
    }
    Ok(valcol)
}

/// Requête sur les données applicatives qui retourne une table de valeur
pub async fn kerdata(pooldb: &Pool<Postgres>, sql: &str ) -> Result<Vec<HashMap<String, String>>, Error> {
    // log::info!("{}", sql);
    let rows = sqlx::query(&sql).fetch_all(pooldb).await?;
    //     Ok(t) => t,
    //     Err(e) => {
    //         Err(FlashMessage::error(format!("{:?}", e).as_str()))
    //     }
    // };

    // Chargement des enregistrements lus dans un tableau de valeur
    let result = rows_to_vmap(rows);

    Ok(result)
}

pub fn rows_to_vmap(rows: Vec<PgRow>) -> Vec<HashMap<String, String>> {
    let mut vmap = vec![];
    for row in rows.iter() {
        let mut result = HashMap::new();
        for col in row.columns() {
            result.insert(
                col.name().to_string(),
                reflective_get(row, col.ordinal()),
            );
        }
        vmap.push(result);
    }
   vmap
}
fn _type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
/// The postgres-crate does not provide a default mapping to fallback to String for all
/// types: row.get is generic and without a type assignment the FromSql-Trait cannot be inferred.
/// This function matches over the current column-type and does a manual conversion
fn reflective_get(row: &PgRow, index: usize) -> String {
    let column_type = row.columns().get(index).map(|c| c.type_info().name()).unwrap();
    // println!("{} : {}", row.column(index).name(), column_type);
    // see https://docs.rs/sqlx/0.4.0-beta.1/sqlx/postgres/types/index.html
    let value = match column_type {
        "BOOL" => {
            let v: Option<bool> = row.get(index);
            v.map(|v| v.to_string())
        }
        "VARCHAR" | "CHAR(N)" | "NAME" => {
            let v: Option<String> = row.get(index);
            v
        }
        "TEXT" => {
            let v: Option<String>  = row.get(index);
            v.map(|v| v.to_string())
        }
        "UUID" => {
            let v: Option<sqlx::types::uuid::Uuid>  = row.get(index);
            v.map(|v| v.to_string())
        }
        "INT2" | "SMALLSERIAL" | "SMALLINT" => {
            let v: Option<i16> = row.get(index);
            v.map(|v| v.to_string())
        }
        "INT" | "INT4" | "INTEGER" | "SERIAL" => {
            let v: Option<i32> = row.get(index);
            v.map(|v| v.to_string())
        }
        "INT8" | "BIGSERIAL" | "BIGINT" => {
            let v: Option<i64> = row.get(index);
            v.map(|v| v.to_string())
        }
        "FLOAT4" | "REAL" => {
            let v: Option<f32> = row.get(index);
            v.map(|v| v.to_string())
        }
        "FLOAT8" | "DOUBLE PRECISION" => {
            let v: Option<f64> = row.get(index);
            v.map(|v| v.to_string())
        }
        "DECIMAL" | "NUMERIC" => {
            let v: Option<sqlx::types::BigDecimal> = row.get(index);
            v.map(|v| v.to_string())
        }
        "DATE" => {
            // with-chrono feature is needed for this
            let v: Option<chrono::NaiveDate> = row.get(index);
            v.map(|v| v.to_string())
        }
        "TIME" => {
            // with-chrono feature is needed for this
            let v: Option<chrono::NaiveTime> = row.get(index);
            v.map(|v| v.to_string())
        }
        "TIMESTAMPTZ" => {
            // with-chrono feature is needed for this
            let v: Option<chrono::DateTime<Utc>> = row.get(index);
            v.map(|v| v.to_string())
        }
        "TIMESTAMP" => {
            // with-chrono feature is needed for this
            let v: Option<chrono::NaiveDateTime> = row.get(index);
            v.map(|v| v.to_string())
        }
        &_ => Some("CANNOT PARSE".to_string()),
    };
    value.unwrap_or("".to_string())
}
