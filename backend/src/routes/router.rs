use axum::{Router, routing::{get, post}, middleware};
use mongodb::{Database};

use crate::controller::auth_controller::*;
use crate::middleware::auth_middleware::auth_middleware;

pub async fn create_router(db: Database) -> Router {
    Router::new()
        .route("/", get(|| async {"Hello World"}))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/protected", get(|| async { "Protected Route" }))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(db)
}