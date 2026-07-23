use gritshield::GritAdmin;
use gritshield::GritComponent;
use sea_orm::DatabaseConnection;

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    searchable = ["id", "issue_id", "author_id", "body", "created_at",],
    read_only = ["created_at"],
)]
pub struct CommentRepository {
    pub db: DatabaseConnection,
}