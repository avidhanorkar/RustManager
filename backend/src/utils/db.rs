use mongodb::{options::ClientOptions, Client, Database};
use std::env;

pub async fn db_connect() -> Database {
    let uri = env::var("MongoDB").expect("MongoDB uri is not set");
    let client_options= ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client.database("RustManager")
}