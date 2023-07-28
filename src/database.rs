use mongodb::{Client, Database, bson::doc};
use serde::{Deserialize, Serialize};

use crate::log::{log_error, log_info, log_debug};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub commands: u64,
    pub known_servers: u8
}

pub async fn connect(uri: &str) -> Database {
    log_info("Connecting to the DB");
    log_debug(&format!("Connecting to DB using URI - {}", uri));

    let connection = Client::with_uri_str(uri).await.unwrap();
    let logged = connection.database("admin").run_command(doc! {"ping": 1}, None).await;

    match logged {
        Ok(_) => {
           log_info("Successfully connected to the DB!")
        }
        Err (e) => {
           log_error("Failed to connet to the DB!");
           log_error(&e.to_string());
           std::process::exit(0);
        }
    }

    connection.database("orbital")
}