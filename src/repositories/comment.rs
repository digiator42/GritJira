use gritshield::GritAdmin;

#[derive(Clone, GritAdmin)]
#[repository(
    searchable = ["id", "issue_id", "author_id", "body", "created_at",],
    read_only = ["created_at"],
)]
pub struct CommentRepository {
    pub db: sea_orm::DatabaseConnection,
}