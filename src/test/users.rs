use crate::models::{User, UserID};
use crate::repo::db::DB;
use std::sync::Arc;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::body::Body;
use serde_json::{json, Value};
use http_body_util::BodyExt;
use tower::{Service, ServiceExt};
use crate::app;

#[tokio::test]
async fn user_get() {
    let db = Arc::new(DB::new("secret".to_string()));

    let id = db.create_user(
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
        "+1234567890".to_string(),
    );

    let mut app = app(Arc::clone(&db)).into_service();

    let json_body = json!({"id": id.0});
    let string_body = serde_json::to_string(&json_body).unwrap();
    let request = Request::builder()
        .uri("/user")
        .method("GET")
        .header("content-type", "application/json")
        .body(Body::from(string_body))
        .unwrap();

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let user_bytes = response.into_body().collect().await.unwrap().to_bytes();

    let user: User = serde_json::from_slice(&user_bytes).unwrap();

    assert_eq!(user.id, id);

    println!("User: {:?}", user);
}
