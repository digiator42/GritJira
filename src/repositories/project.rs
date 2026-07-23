use gritshield::GritAdmin;

#[derive(Clone, GritAdmin)]
#[repository(
    searchable = [ "key", "name", "description", "created_at", ],
    read_only = ["created_at"],
)]
pub struct ProjectRepository {
    pub db: sea_orm::DatabaseConnection,
}