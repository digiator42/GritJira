use gritshield::GritAdmin;

#[derive(Clone, GritAdmin)]
#[repository(
    searchable = [ "project_id", "name", "goal", "status", "start_date", "end_date",],
    read_only = ["created_at"],
)]
pub struct SprintRepository {
    pub db: sea_orm::DatabaseConnection,
}