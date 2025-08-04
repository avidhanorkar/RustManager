use axum::{Router, routing::{get, post}};
use mongodb::{Database};

use crate::controller::auth_controller::*;

pub async fn create_router(db: Database) -> Router {
    Router::new()
        .route("/", get(|| async {"Hello World"}))
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(db)
}