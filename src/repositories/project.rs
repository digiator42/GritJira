use gritshield::GritAdmin;

#[derive(Clone, GritAdmin)]
#[repository(
    // searchable = ["id", "post_id", "created_at", "content", "user_id",],
    // grid_columns = ["id", "post_id", "user_id", "content", "created_at"],
    read_only = ["created_at"],
)]
pub struct ProjectRepository {
    pub db: sea_orm::DatabaseConnection,
}