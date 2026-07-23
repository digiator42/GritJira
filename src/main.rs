use gritshield::database::{DbConfig, DbManager};
use gritshield::middleware::AuthMiddleware;
use gritshield::{declare_security_caps, inject, prelude::*};
use sea_orm::DatabaseConnection;
use security::caps::*;

mod controllers;
mod database;
mod events;
mod jobs;
mod models;
mod repositories;
mod security;
mod services;
mod web;

declare_security_caps! {
    IssueEdit    => [Admin, Manager, Developer],
    IssueCreate  => [Admin, Manager, Developer, Tester],
    IssueDelete  => [Admin, Manager],
    ProjectAdmin => [Admin],
    ViewBoard    => [Admin, Manager, Developer, Tester, Viewer],
}
// db must be DatabaseConnection not
fn auto_wire(db: DatabaseConnection) {
    inject!(DatabaseConnection, db);
}

#[tokio::main]
async fn main() {
    // Initialize the engine configuration setup matrix
    let db_config = DbConfig::default();

    // Fire connection pool parameters and run pending dynamic migrations automatically!
    let shared_db: Arc<DatabaseConnection> = DbManager::connect(db_config).await.unwrap();

    auto_wire((*shared_db).clone());

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
