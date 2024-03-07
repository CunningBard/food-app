#[macro_use]
extern crate serde;

mod api;
mod models;
mod repo;
mod test;

use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;
use std::sync::Arc;

use models::randint::RangeParameters;
use crate::api::get_user_info;
use crate::repo::db::DB;

#[tokio::main]
async fn main() {
    let mock_database = Arc::new(repo::db::DB::new("example secret password".to_string()));

    // route("/", get(move || handler(Arc::clone(&data_for_route))));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app(Arc::clone(&mock_database))).await.unwrap();
}

fn app(db: Arc<DB>) -> Router {
    Router::new()
        .route("/rand", get(handler))
        .route("/user", get(get_user_info).with_state(Arc::clone(&db)))
}

async fn handler(Query(range): Query<RangeParameters>) -> Html<String> {
    // Generate a random number in range parsed from query.
    let seed = chrono::Utc::now().timestamp_millis();
    let random_number =
        (seed * 43 + seed * 923) as usize % (range.end - range.start + 1) + range.start;

    // Send response in html format.
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}
