use crate::models::{GetUser, User};
use crate::repo::db::DB;
use axum::debug_handler;
use axum::Json;
use std::sync::Arc;
use axum::extract::State;

#[debug_handler]
pub async fn get_user_info(State(database): State<Arc<DB>>, Json(payload): Json<GetUser>) -> Result<Json<User>, String> {
    let user = database.find_user_by_id(payload.id);
    match user {
        Some(user) => Ok(Json(user)),
        None => Err("User not found".to_string()),
    }
}
