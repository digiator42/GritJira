use gritshield::database::{DbConfig, DbManager};
use gritshield::middleware::AuthMiddleware;
use gritshield::prelude::*;

mod controllers;
mod repositories;
mod models;
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
        .mount_db(shared_db)
        .add_role_inheritance("Admin", vec!["Manager", "Operator", "Auditor"])
        .add_role_inheritance("Manager", vec!["Editor", "Viewer"])
        .add_role_inheritance("Editor", vec!["Contributor"]);

    ignite("127.0.0.1", "8080", router).await;
}
