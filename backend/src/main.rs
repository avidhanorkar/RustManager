use axum::{
    Router,
    routing::get
};

use std::env;
use dotenvy::dotenv;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port: u16 = env::var("PORT").expect("Port is not set").parse().expect("Must be a number");
    
    let addr= SocketAddr::from(([127, 0, 0, 1], port));
    let app = Router::new().route("/", get(|| async {"Hello World"}));
    println!("The server is up on address: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
