use gritshield::GritAdmin;
use gritshield::GritComponent;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    searchable = [ "key", "name", "description", "created_at", ],
    read_only = ["created_at"],
)]
pub struct ProjectRepository {
    pub db: DatabaseConnection,
}