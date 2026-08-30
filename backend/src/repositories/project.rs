use chrono::Utc;
use gritshield::GritAdmin;
use gritshield::GritComponent;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use std::sync::Arc;

use crate::models::project;
use crate::models::project::Model as ProjectModel;

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    searchable = [ "key", "name", "description", "created_at", ],
    read_only = ["created_at"],
)]
pub struct ProjectRepository {
    pub db: DatabaseConnection,
}

impl ProjectRepository {
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

    /// List every project the user is a member of, ordered by name.
    pub async fn list_projects_for_user(
        &self,
        user_id: i32,
    ) -> Result<Vec<ProjectModel>, sea_orm::DbErr> {
        use crate::models::project::Entity as Project;
        use crate::models::project_member::{self, Entity as ProjectMember};
        use sea_orm::ColumnTrait;

        let member_projects = ProjectMember::find()
            .filter(project_member::Column::UserId.eq(user_id))
            .all(&self.db)
            .await?;

        let project_ids: Vec<i32> = member_projects.iter().map(|m| m.project_id).collect();
        if project_ids.is_empty() {
            return Ok(vec![]);
        }

        Project::find()
            .filter(project::Column::Id.is_in(project_ids))
            .order_by_asc(project::Column::Name)
            .all(&self.db)
            .await
    }

    /// Get the first project in the system
    pub async fn get_first_project(&self) -> Result<Option<i32>, sea_orm::DbErr> {
        use crate::models::project::Entity as Project;
        use sea_orm::ColumnTrait;

        let project = Project::find()
            .order_by_asc(project::Column::Id)
            .one(&self.db)
            .await?;

        Ok(project.map(|p| p.id))
    }

    /// Get project key by ID
    pub async fn get_project_key(&self, project_id: i32) -> Result<Option<String>, sea_orm::DbErr> {
        use crate::models::project::Entity as Project;
        use sea_orm::ColumnTrait;

        let project = Project::find()
            .filter(project::Column::Id.eq(project_id))
            .one(&self.db)
            .await?;

        Ok(project.map(|p| p.key))
    }

    pub async fn create_default_project(&self, username: &str) -> Result<i32, sea_orm::DbErr> {
        use crate::models::project::ActiveModel;
        use crate::models::project::Entity as Project;
        use crate::models::project_member::{self, ActiveModel as MemberActiveModel};
        use crate::models::workflow::{self, ActiveModel as WorkflowActiveModel};

        let key = format!(
            "{}{}",
            username.chars().take(4).collect::<String>().to_uppercase(),
            Utc::now().timestamp() % 10000
        );

        let name = format!("{}'s Project", username);

        // Create project
        let new_project = ActiveModel {
            key: Set(key),
            name: Set(name),
            description: Set(Some("Default project".to_string())),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };
        let project = new_project.insert(&self.db).await?;

        // Create default workflow steps
        let default_steps = vec![
            ("Backlog", 0),
            ("To Do", 1),
            ("In Progress", 2),
            ("In Review", 3),
            ("Done", 4),
        ];
        for (step_name, position) in default_steps {
            let step = WorkflowActiveModel {
                project_id: Set(project.id),
                name: Set(step_name.to_string()),
                position: Set(position),
                is_completed: Set(false),
                ..Default::default()
            };
            step.insert(&self.db).await?;
        }

        // Add user as Admin
        let member = MemberActiveModel {
            project_id: Set(project.id),
            user_id: Set(1), // You need to pass the user_id
            role: Set("Admin".to_string()),
            joined_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };
        member.insert(&self.db).await?;

        Ok(project.id)
    }
}
