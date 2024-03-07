#[macro_use]
extern crate serde;

mod api;
mod models;
mod repo;

use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;

use models::randint::RangeParameters;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new().route("/rand", get(handler))
}

async fn handler(Query(range): Query<RangeParameters>) -> Html<String> {
    // Generate a random number in range parsed from query.
    let seed = chrono::Utc::now().timestamp_millis();
    let random_number = (seed * 43 + seed * 923) as usize % (range.end - range.start + 1) + range.start;

    // Send response in html format.
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}
