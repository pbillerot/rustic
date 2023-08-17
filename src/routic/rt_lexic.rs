
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use actix_web::{
    get,
    // delete,
    // post,
    HttpResponse,
    web,
    // web::ReqData,
    Responder,
    // Result,
};
// use log::info;
// use actix_session::Session;
// use actix_web_lab::respond::Html;
// use std::sync::atomic::Ordering;
// use crate::servic;
use crate::AppState;
use crate::lexic;

#[derive(Debug)]

#[derive(Deserialize, Serialize)]
struct Info {
    action: String,
}

#[get("/lexic/{action}")]
async fn lexicall(info: web::Path<Info>, data: web::Data<AppState>) -> impl Responder {
    println!("{:?}", info);
    let plexic = data.plexic.load(Ordering::Relaxed);
    unsafe { println!("{:?}", (*plexic).portail)}
    unsafe { println!("{:?}", (*plexic).applications)}

    let newlexic = Box::new(lexic::lex_lexic::Lexic::load());
    plexis = Box::into_raw(newlexic.clone());

    HttpResponse::Ok().body("Hello world!")
}