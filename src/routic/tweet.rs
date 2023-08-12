use actix_web::{
    get,
    delete,
    post,
    HttpResponse,
    web,
    web::{Json, Path},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use uuid::Uuid;
// extern crate sqlx;
// use sqlx::postgres::PgRow;
// use sqlx::postgres::Postgres;
// use sqlx::Decode;
// use sqlx::TypeInfo;
// use sqlx::Row;
// use sqlx::Column;
// use sqlx::ValueRef;
// use sqlx::Decode;

// use std::collections::HashMap;

// use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
// use std::collections::HashMap;

use crate::constants::APPLICATION_JSON;
// use crate::like::Like;
// use crate::response::Response;
use crate::AppState;
use crate::models::rows_to_vmap;
// use crate::models;

#[derive(Debug, Deserialize, Serialize)]
struct Response<T> {
    results: Vec<T>
}
type Tweets = Response<Tweet>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: String,
    // pub likes: Vec<Like>,
}

impl Tweet {
    pub fn new(message: String) -> Self {
        Self {
            id: "id".to_string(),
            created_at: Utc::now(),
            message,
            // likes: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}

impl TweetRequest {
    pub fn to_tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => Some(Tweet::new(message.to_string())),
            None => None,
        }
    }
}

/// list 50 last tweets `/tweets`
#[get("/tweets")]
pub async fn list(data: web::Data<AppState>) -> HttpResponse {
    // TODO find the last 50 tweets and return them

    let rows = sqlx::query(
        "SELECT * FROM tweets LIMIT 50",
    )
    .fetch_all(&data.db)
    .await.expect("query error");

    let vmap = rows_to_vmap(rows);
    log::info!("{:?}", vmap);
    let tweets = Tweets { results: vec![] };

    //let rowson = serde_json::to_string(&rows).expect("erreur convert json");

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(&tweets)

}

/// create a tweet `/tweets`
#[post("/tweets")]
pub async fn create(tweet_req: Json<TweetRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(tweet_req.to_tweet())
}

/// find a tweet by its id `/tweets/{id}`
#[get("/tweets/{id}")]
pub async fn get(path: Path<(String,)>) -> HttpResponse {
    // TODO find tweet a tweet by ID and return it
    println!("{:?}", path);
    let found_tweet: Option<Tweet> = None;

    match found_tweet {
        Some(tweet) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(tweet),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

/// delete a tweet by its id `/tweets/{id}`
#[delete("/tweets/{id}")]
pub async fn delete(path: Path<(String,)>) -> HttpResponse {
    // TODO delete tweet by ID
    // in any case return status 204
    println!("{:?}", path);
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}