use chrono::Utc;
use gritshield::{GritAdmin, GritComponent};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set};
#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    entity = "crate::models::project_member",
    searchable = [ "project_id", "user_id", "username", "role", "joined_at" ],
    read_only = ["joined_at"],
)]
pub struct ProjectMemberRepository {
    pub db: DatabaseConnection,
}

impl ProjectMemberRepository {
    /// Get the user's default project (first project they are a member of)
    pub async fn get_user_default_project(
        &self,
        user_id: i32,
    ) -> Result<Option<i32>, sea_orm::DbErr> {
        use crate::models::project_member::{self, Entity as ProjectMember};
        use sea_orm::ColumnTrait;

        let member = ProjectMember::find()
            .filter(project_member::Column::UserId.eq(user_id))
            .order_by_asc(project_member::Column::JoinedAt)
            .one(&self.db)
            .await?;

        Ok(member.map(|m| m.project_id))
    }

    /// Add user to project
    pub async fn add_user_to_project(
        &self,
        project_id: i32,
        user_id: i32,
        username: &str,
        role: &str,
    ) -> Result<(), sea_orm::DbErr> {
        use crate::models::project_member::{self, ActiveModel};

        let member = ActiveModel {
            project_id: Set(project_id),
            user_id: Set(user_id),
            username: Set(username.to_string()),
            role: Set(role.to_string()),
            joined_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        member.insert(&self.db).await?;
        Ok(())
    }
}
