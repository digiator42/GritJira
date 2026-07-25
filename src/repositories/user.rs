use gritshield::GritAdmin;
use gritshield::GritComponent;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    searchable = ["username", "email", "role", "avatar_url", "created_at",], 
    read_only = ["created_at"],
)]
pub struct UserRepository {
    pub db: DatabaseConnection,
}
