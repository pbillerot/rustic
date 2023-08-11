use actix_web::{
    get,
    delete,
    post,
    HttpResponse,
    web,
};
use log::info;

use crate::AppState;



/// list 50 last tweets `/tweets`
#[get("/load")]
pub async fn list(data: web::Data<AppState>) -> HttpResponse {
    // TODO find the last 50 tweets and return them


    HttpResponse::Ok().body("Hello world!");

}
