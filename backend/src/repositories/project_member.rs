use chrono::Utc;
use gritshield::{GritAdmin, GritComponent};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set};

use crate::models::UserModel;
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
    /// Whether `user_id` is a member of `project_id`
    pub async fn is_member(&self, project_id: i32, user_id: i32) -> Result<bool, sea_orm::DbErr> {
        use crate::models::project_member::{self, Entity as ProjectMember};
        use sea_orm::ColumnTrait;

        let members = ProjectMember::find()
            .filter(
                project_member::Column::ProjectId
                    .eq(project_id)
                    .and(project_member::Column::UserId.eq(user_id)),
            )
            .all(&self.db)
            .await?;

        Ok(!members.is_empty())
    }

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

    /// Remove a member from a project
    pub async fn remove_member(&self, member_id: i32) -> Result<bool, sea_orm::DbErr> {
        use crate::models::project_member::{self, Entity as ProjectMember};
        use sea_orm::ColumnTrait;

        let result = ProjectMember::delete_by_id(member_id)
            .exec(&self.db)
            .await?;

        Ok(result.rows_affected > 0)
    }

    /// Update a member's role
    pub async fn update_member_role(
        &self,
        member_id: i32,
        role: &str,
    ) -> Result<Option<crate::models::project_member::Model>, sea_orm::DbErr> {
        use crate::models::project_member::{self, Entity as ProjectMember};
        use sea_orm::ColumnTrait;

        let member = ProjectMember::find_by_id(member_id).one(&self.db).await?;

        if let Some(m) = member {
            let mut active: project_member::ActiveModel = m.into();
            active.role = Set(role.to_string());
            let updated = active.update(&self.db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    /// Get all members of a project with user details (username from users table)
    pub async fn get_project_members_with_users(
        &self,
        project_id: i32,
    ) -> Result<
        Vec<(
            crate::models::project_member::Model,
            crate::models::user::Model,
        )>,
        sea_orm::DbErr,
    > {
        use crate::models::project_member::{self, Entity as ProjectMember};
        use crate::models::user::{self, Entity as User};
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

        let members = ProjectMember::find()
            .filter(project_member::Column::ProjectId.eq(project_id))
            .find_also_related(User)
            .all(&self.db)
            .await?;

        let members_with_users = members
            .into_iter()
            .filter_map(|(member, user)| user.map(|user| (member, user)))
            .collect();

        Ok(members_with_users)
    }
}
