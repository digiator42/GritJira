use gritshield::database::{DbConfig, DbManager};
use gritshield::middleware::AuthMiddleware;
use gritshield::prelude::*;

mod controllers;
mod database;
mod models;
mod repositories;
mod services;
mod web;

#[tokio::main]
async fn main() {
    // Initialize the engine configuration setup matrix
    let db_config = DbConfig::default();

    // Fire connection pool parameters and run pending dynamic migrations automatically!
    let db_connection = DbManager::connect(db_config).await.unwrap();
    let shared_db = db_connection;

    let router = Router::new()
        .add_middleware(AuthMiddleware::new_session(
            vec![
                "/auth/login".to_string(),
                "/api/**".to_string(),
                "/admin/**".to_string(),
            ],
            Some("/api/info/sea-orm"),
        ))
        .mount_db(shared_db.clone());

    // Seed test data on launch
    if let Err(e) = database::seeder::seed_database(&shared_db).await {
        eprintln!("[SEEDER ERROR] {}", e);
    }

    ignite("127.0.0.1", "8080", router).await;
}
