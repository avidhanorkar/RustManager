use axum;
use mongodb::Database;
use std::env;
use dotenvy::dotenv;
use std::net::SocketAddr;
mod controller;
mod models;
mod utils;
use utils::db::db_connect;

mod routes;
use routes::router::create_router;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db: Database = db_connect().await;
    let port: u16 = env::var("PORT").expect("Port is not set").parse().expect("Must be a number");
    
    let addr= SocketAddr::from(([127, 0, 0, 1], port));
    let app = create_router(db).await;
    println!("The server is up on address: {}", addr);
    println!("Mongo DB is connected Successfully!!!");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
