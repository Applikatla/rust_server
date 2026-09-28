mod db;
mod handler;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};

use handler::{
    health::health,
    user::{create_user, user, user_by_id},
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db = db::create_db_connection().await;

    println!("Database connected successfully!");

    let app = Router::new()
        .route("/health", get(health))
        .route("/user", get(user))
        .route("/user/{id}", get(user_by_id))
        .route("/create-user", post(create_user)).with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:8080");

    axum::serve(listener, app)
        .await
        .unwrap();
}
