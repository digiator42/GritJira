use gritshield::core::AutoWire;
use gritshield::core::shield::GritShield;
use gritshield::database::{DbConfig, DbManager};
use gritshield::middleware::AuthMiddleware;
use gritshield::{catch, inject, prelude::*};
use sea_orm::DatabaseConnection;
use security::caps::*;

mod controllers;
mod database;
mod dtos;
mod events;
mod jobs;
mod models;
mod repositories;
mod security;
mod services;

// db must be DatabaseConnection not
fn auto_wire(db: DatabaseConnection) {
    inject!(DatabaseConnection, db);
    AutoWire::boot_di_container();
}

#[catch(status = 404)]
pub async fn handle_not_found(ctx: RequestContext) -> Response {
    Response::json_not_found(&serde_json::json!({
        "error": "Not found",
        "message": format!("No route matches {}", ctx.req.path),
        "path": ctx.req.path,
    }))
}

#[launch]
async fn main() {
    // Initialize the engine configuration setup matrix
    let db_config = DbConfig::default();

    // Fire connection pool parameters and run pending dynamic migrations automatically!
    let shared_db: Arc<DatabaseConnection> = DbManager::connect(db_config).await.unwrap();

    auto_wire((*shared_db).clone());

    // Seed test data on launch
    if let Err(e) = database::seeder::seed_database(&shared_db).await {
        eprintln!("[SEEDER ERROR] {}", e);
    }

    let router = Router::new()
        .add_middleware(AuthMiddleware::new_session(
            vec![
                "/static/**".to_string(),
                "/api/v1/auth/**".to_string(),
                "/admin/**".to_string(),
            ],
            None,
        ))
        .mount_db(shared_db.clone());

    let dot_schema = AutoWire::export_dot();
    println!("[DI CONTAINER] Dependency Injection Graph:\n{}", dot_schema);

    GritShield::build().router(router).launch();
}
