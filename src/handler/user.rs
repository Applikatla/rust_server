use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::json;
use sea_orm::DatabaseConnection;

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

pub async fn create_user(State(db): State<DatabaseConnection>, Json(payload): Json<User>) -> Json<serde_json::Value> {
    println!("Database connection received!");
    Json(json!({
        "message": format!("User created with ID: {}, Name: {}, Email: {}", payload.id, payload.name, payload.email)
    }))
}
