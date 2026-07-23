use gritshield::GritAdmin;
use gritshield::GritComponent;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    searchable = [ "project_id", "name", "goal", "status", "start_date", "end_date",],
    read_only = ["created_at"],
)]
pub struct SprintRepository {
    pub db: DatabaseConnection,
}