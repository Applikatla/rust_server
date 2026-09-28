mod handler;
mod utils;

use axum::{
    Router,
    routing::get,
    routing::post,
};

use handler::{
    health::health,
    user::{user, user_by_id, create_user},
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app: Router<()> = Router::new().route("/health", get(health)).
    route("/user", get(user)).
    route("/user/{id}", get(user_by_id)).
    route("/create-user", post(create_user));

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server running on http://127.0.0.1:8080");

    axum::serve(listner, app).await.unwrap();
}
