use axum::{
    Json,
    extract::Path,
};
use serde_json::json;

use crate::utils::utils::User;

pub async fn user() -> Json<serde_json::Value> {
    Json(json!({
        "message": "User endpoint"
    }))
}

pub async fn user_by_id(Path(id): Path<u32>) -> Json<serde_json::Value> {
    Json(json!({
        "message": format!("User endpoint with ID: {}", id)
    }))
}

pub async fn create_user(Json(payload): Json<User>) -> Json<serde_json::Value> {
    Json(json!({
        "message": format!("User created with ID: {}, Name: {}, Email: {}", payload.id, payload.name, payload.email)
    }))
}
