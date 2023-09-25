use std::collections::HashMap;

use tera::try_get_value;
use tera::Result;
use tera::Value;

// tera.register_filter("format_amount", format_amount);
pub fn format_amount(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let amount_str = try_get_value!("format_amount", "value", String, value);
    if amount_str.is_empty() {
        Ok(String::new().into())
    } else {
        let amount = amount_str.parse::<f64>().unwrap();
        // let mut f: Formatter;
        // f = "[.2n/ ] €".parse().unwrap();
        // Ok(f.fmt2(amount).into())
        Ok(format!("{:.2} €", amount).as_str().into())
    }
}

// use markdown::to_html;
// use std::collections::HashMap;
// use tera::{try_get_value, Result, Value};
// pub fn markdown_to_html(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
//     let markdown_string = try_get_value!("markdown", "value", String, value);
//     Ok(to_html(markdown_string.as_str()).into())
// }
// tera.register_filter("markdown", markdown_to_html);
// {{ content | markdown | safe }}

// {{ maclite(sql=view.style_sqlite, record=record) }}
// pub async fn maclite(
//     args: &HashMap<String, tera::Value>,
//     // poolsqlite: Pool<Sqlite>,
// ) -> tera::Result<tera::Value> {
//     let poolsqlite = match args.get("sqlite") {
//         Some(val) => tera::try_get_value!("maclite", "sqlite", &Pool<Sqlite>, val),
//         None => {
//             tera::Error::msg(format!("maclite error [pool sqlite]"));
//             return Ok("".into());
//         }
//     };
//     let record = match args.get("record") {
//         Some(val) => tera::try_get_value!("maclite", "record", HashMap<String, String>, val),
//         None => {
//             tera::Error::msg(format!("maclite : args record not defined"));
//             HashMap::new()
//         }
//     };

//     let source = match args.get("sql") {
//         Some(val) => tera::try_get_value!("maclite", "attribute", String, val),
//         None => String::new(),
//     };
//     if source.is_empty() {
//         return Ok("".into());
//     }
//     // remplacement des macro {var} par leur valeur
//     let sql = macrolex(&source, &record);
//     // exec du sql
//     if sql.is_empty() {
//         return Ok("".into());
//     }
//     match kerlite(&poolsqlite, &sql).await {
//         Ok(result) => return Ok(tera::Value::String(result.into())),
//         Err(e) => {
//             tera::Error::msg(format!("maclite error {}:[{}]", sql, e));
//             return Ok("".into());
//         }
//     }
// }
// fn make_url_for(urls: HashMap<String, String>) -> impl tera::Function {
//     Box::new(
//         move |args: &HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
//             match args.get("name") {
//                 Some(val) => match tera::from_value::<String>(val.clone()) {
//                     Ok(v) => Ok(tera::to_value(urls.get(&v).unwrap()).unwrap()),
//                     Err(_) => Err("oops".into()),
//                 },
//                 None => Err("oops".into()),
//             }
//         },
//     )
// }
// pub async fn acolite(poolsqlite: Pool<Sqlite>) -> impl tera::Function {
//     // let mut rt = Runtime::new().unwrap();
//     Box::new(
//         move |args: &HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
//             let record = match args.get("record") {
//                 Some(val) => {
//                     tera::try_get_value!("maclite", "record", HashMap<String, String>, val)
//                 }
//                 None => {
//                     tera::Error::msg(format!("maclite : args record not defined"));
//                     HashMap::new()
//                 }
//             };

//             let source = match args.get("sql") {
//                 Some(val) => tera::try_get_value!("maclite", "attribute", String, val),
//                 None => String::new(),
//             };
//             if source.is_empty() {
//                 return Ok("".into());
//             }
//             // remplacement des macro {var} par leur valeur
//             let sql = macrolex(&source, &record);
//             // exec du sql
//             if sql.is_empty() {
//                 return Ok("".into());
//             }

//             let result = match kerlite(&poolsqlite, &sql).await {
//                 Ok(res) => res,
//                 Err(e) => {
//                     tera::Error::msg(format!("maclite error {}:[{}]", sql, e));
//                     String::new()
//                 }
//             };
//             Ok(tera::Value::String(result.into()))
//         },
//     )
// }
