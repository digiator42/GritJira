use crate::models::user;
use gritshield::GritAdmin;
use gritshield::GritComponent;
use gritshield::database::GritRepository;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use std::sync::Arc;

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    searchable = ["username", "email", "role", "avatar_url", "created_at",], 
    read_only = ["created_at"],
)]
pub struct UserRepository {
    pub db: DatabaseConnection,
}

impl UserRepository {
    pub async fn create(
        &self,
        username: &str,
        email: &str,
        password: &str,
        role: &str,
    ) -> Result<user::Model, DbErr> {
        let new_user = user::ActiveModel {
            username: Set(username.to_string()),
            email: Set(email.to_string()),
            password: Set(password.to_string()),
            role: Set(role.to_string()),
            ..Default::default()
        };

        new_user.insert(&self.db).await
    }


}
