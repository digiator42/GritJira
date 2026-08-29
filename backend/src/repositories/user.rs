use crate::models::user;
use gritshield::GritAdmin;
use gritshield::GritComponent;
use gritshield::database::GritRepository;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
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

    /// Update profile fields (username / email) for a user
    pub async fn update_profile(
        &self,
        user_id: i32,
        username: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<user::Model>, DbErr> {
        let existing = user::Entity::find_by_id(user_id).one(&self.db).await?;

        if let Some(existing) = existing {
            let mut active: user::ActiveModel = existing.into();
            if let Some(username) = username {
                active.username = Set(username.to_string());
            }
            if let Some(email) = email {
                active.email = Set(email.to_string());
            }
            let updated = active.update(&self.db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    /// Change password if current password matches. Returns Ok(None) if the
    /// user is unknown, Ok(Some(false)) when the current password is wrong.
    pub async fn change_password(
        &self,
        user_id: i32,
        current_password: &str,
        new_password: &str,
    ) -> Result<Option<bool>, DbErr> {
        use crate::models::user::Entity as User;

        let existing = User::find_by_id(user_id).one(&self.db).await?;

        match existing {
            Some(existing) => {
                if existing.password != current_password {
                    return Ok(Some(false));
                }
                let mut active: user::ActiveModel = existing.into();
                active.password = Set(new_password.to_string());
                active.update(&self.db).await?;
                Ok(Some(true))
            }
            None => Ok(None),
        }
    }
}
