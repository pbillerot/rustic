///
/// CRUD sur les données
///
use sqlx::{Pool, Postgres};

use crate::lexicer::lex_table::Table;
///
/// - Suppression d'un article
///
pub async fn crud_delete(
    pooldb: &Pool<Postgres>,
    table: &Table, // le lexique de la table
    id: &str,
) -> Result<String, String> {

    // construction de l'order sql
    // construction de l'ordre sql de mise à jour
    let mut sql = "DELETE FROM ".to_string();
    sql.push_str(&table.tableid);
    sql.push_str(format!(" WHERE ( {} = '{}' )", &table.setting.key, &id).as_str());

    let result = match sqlx::query(&sql).execute(pooldb).await {
        Ok(r) => r,
        Err(e) => {
            let msg = format!("{sql:?} : {e:?}");
            log::error!("{msg}");
            return Err(msg)
        }
    };

    Ok(format!("{sql:?} {result:?}"))

}

