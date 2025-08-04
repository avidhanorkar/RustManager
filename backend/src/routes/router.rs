use axum::{middleware, routing::{get, patch, post}, Router};
use mongodb::{Database};

use crate::controller::{auth_controller::*, task_controller::*};
use crate::middleware::auth_middleware::auth_middleware;

pub async fn create_router(db: Database) -> Router {
    Router::new()
        .route("/", get(|| async {"Hello World"}))
        .route("/user/register", post(register))
        .route("/user/login", post(login))

        .route("/protected", get(|| async { "Protected Route" }))
        .layer(middleware::from_fn(auth_middleware))

        .route("/user", get(get_user_data))
        .layer(middleware::from_fn(auth_middleware))

        .route("/task/create", post(create_task))
        .layer(middleware::from_fn(auth_middleware))

        .route("/task/update/{task_id}", patch(update_task))
        .layer(middleware::from_fn(auth_middleware))

        .route("/task/getAll", get(all_for_user))
        .layer(middleware::from_fn(auth_middleware))
        
        .with_state(db)
}