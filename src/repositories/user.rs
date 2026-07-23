use gritshield::GritAdmin;

#[derive(Clone, GritAdmin)]
#[repository(
    searchable = ["username", "email", "avatar_url", "created_at",], 
    read_only = ["created_at"],
)]
pub struct UserRepository {
    pub db: sea_orm::DatabaseConnection,
}
