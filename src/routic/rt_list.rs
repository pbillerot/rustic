//! Ouverture d'une view
//!
use crate::lexic::lex_table::Element;
use crate::sqlic::sql_utils::querlite;
use crate::sqlic::sql_utils::rows_to_vmap;
use crate::{
    // lexic::lex_table::{self, Element},
    AppState,
};
use actix_session::Session;
use actix_web::{
    get,
    // delete,
    // post,
    // HttpResponse,
    web,
    web::Path,
    Responder,
    Result,
};
use actix_web_lab::respond::Html;
use std::{collections::HashMap, sync::atomic::Ordering};

// cuerl http://0.0.0.0:8080/
#[get("/list/{appid}/{tableid}/{viewid}")]
pub async fn list(
    path: Path<(String, String, String)>,
    _session: Session,
    data: web::Data<AppState>,
    // msg: Option<ReqData<servic::sr_data::Msg>>,
) -> Result<impl Responder> {
    // log::info!("Session {:?} {:?} {:?}", session.status(), session.entries(), path);
    let (appid, tableid, viewid) = path.into_inner();
    let ptr = data.plexic.load(Ordering::Relaxed);
    let apps = unsafe { &(*ptr).applications.clone() };
    let app = apps.get(&appid).unwrap();

    // lecture des données de la tableid
    // construction de l'ordre sql
    let mut sql = "SELECT ".to_string();
    // on prend les colonnes définies dans la view
    let table = app.tables.get(&tableid).unwrap();
    let view = table.views.get(&viewid).unwrap();
    let mut bstart = true;
    for element in &view.velements {
        if element.hide {
            continue;
        }
        if bstart {
            sql.push_str("");
            bstart = false;
        } else {
            sql.push_str(", ");
        }
        sql.push_str(&element.elid);
    }
    sql.push_str(format!(" FROM {}", &tableid).as_str());
    if !view.order_by.is_empty() {
        sql.push_str(format!(" ORDER BY {}", &view.order_by).as_str());
    }
    let rows = match sqlx::query(&sql).fetch_all(&data.db).await {
        Ok(t) => t,
        Err(e) => {
            log::error!("{:?}", e);
            Vec::new()
        }
    };

    // TESTS SQLITE
    let _reslite: HashMap<String, String> = match querlite(&data.dblite,
        "
        select 'text' as mytext,
        122 as myinteger,
        12.23 as myfloat,
        datetime('now','localtime') as localtime,
        CURRENT_TIMESTAMP as current,
        datetime(CURRENT_TIMESTAMP,'localtime') as created_at,
        strftime('%s','now') as strftime
        "
    ).await {
        Ok(t) => t,
        Err(e) => {
            log::error!("{:?}", e);
            HashMap::new()
        }
    };

    let vrows = rows_to_vmap(rows);
    // Construction d'un tableau d'éléments par cellule retournée par la requête
    let mut rowsel:Vec<HashMap<String, Element>> = Vec::new();
    for hrow in vrows {
        let mut hel: HashMap<String, Element> = HashMap::new();
        for vel in &view.velements {
            let mut element = view.elements.get(&vel.elid).unwrap().clone();
            element.compute(&data.db, &data.dblite, vel, &hrow);
            element.value = hrow.get(&vel.elid).unwrap().clone();
            hel.insert(vel.elid.clone(), element);
        }
        rowsel.push(hel);
    }

    let mut context = tera::Context::new();
    context.insert("portail", unsafe { &(*ptr).portail });
    context.insert("application", &app);
    context.insert("tableid", &tableid);
    context.insert("viewid", &viewid);
    context.insert("rowsel", &rowsel);
    let html = data.template.render("tpl_list.html", &context).unwrap();

    Ok(Html(html))
}
