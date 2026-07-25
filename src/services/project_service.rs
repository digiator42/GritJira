// src/services/project_service.rs
use crate::models::project::{
    self, ActiveModel as ProjectActiveModel, GritRepositoryRecord, Model as ProjectModel,
};
use crate::repositories::project::ProjectRepository;
use chrono::Utc;
use gritshield::GritComponent;
use gritshield::database::GritRepository;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::sync::Arc;

#[derive(GritComponent)]
pub struct ProjectService {
    pub repo: Arc<ProjectRepository>,
}

impl ProjectService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            repo: Arc::new(ProjectRepository { db }),
        }
    }

    /// List all projects
    pub async fn list_projects(&self) -> Result<Vec<ProjectModel>, sea_orm::DbErr> {
        self.repo
            .query()
            .order_asc(project::Column::Name)
            .fetch()
            .await
    }

    /// Get project by ID
    pub async fn get_project_by_id(
        &self,
        project_id: i32,
    ) -> Result<Option<GritRepositoryRecord>, sea_orm::DbErr> {
        self.repo.find_one_by_id(project_id).await
    }

    /// Get project by key
    pub async fn get_project_by_key(
        &self,
        key: &str,
    ) -> Result<Option<ProjectModel>, sea_orm::DbErr> {
        match self
            .repo
            .query()
            .where_eq(project::Column::Key, key)
            .fetch_one()
            .await
        {
            Ok(project) => Ok(Some(project.into())),
            Err(sea_orm::DbErr::RecordNotFound(_)) => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Create a new project
    pub async fn create_project(
        &self,
        key: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<ProjectModel, sea_orm::DbErr> {
        let new_project = ProjectActiveModel {
            key: Set(key.to_string().to_uppercase()),
            name: Set(name.to_string()),
            description: Set(description.map(|s| s.to_string())),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        new_project.insert(&self.repo.db).await
    }

    /// Update project
    pub async fn update_project(
        &self,
        project_id: i32,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<ProjectModel, sea_orm::DbErr> {
        match self
            .repo
            .query()
            .where_eq(project::Column::Id, project_id)
            .fetch_one()
            .await
        {
            Ok(existing) => {
                let mut active: ProjectActiveModel = existing.into();

                if let Some(name) = name {
                    active.name = Set(name.to_string());
                }
                if let Some(description) = description {
                    active.description = Set(Some(description.to_string()));
                }

                active.update(&self.repo.db).await
            }
            Err(sea_orm::DbErr::RecordNotFound(_)) => Err(sea_orm::DbErr::RecordNotFound(
                "Project not found".to_string(),
            )),
            Err(err) => Err(err),
        }
    }

    /// Delete project
    pub async fn delete_project(&self, project_id: i32) -> Result<bool, sea_orm::DbErr> {
        let result = self
            .repo
            .query()
            .where_eq(project::Column::Id, project_id)
            .delete()
            .await?;

        Ok(result.rows_affected > 0)
    }

    /// Get project with its issues (for backlog/board)
    pub async fn get_project_with_issues(
        &self,
        project_id: i32,
    ) -> Result<Option<(ProjectModel, Vec<crate::models::issue::Model>)>, sea_orm::DbErr> {
        let project = self.get_project_by_id(project_id).await?;

        if let Some(project) = project {
            let issues = crate::repositories::issue::IssueRepository {
                db: self.repo.db.clone(),
            }
            .query()
            .where_eq(crate::models::issue::Column::ProjectId, project_id)
            .order_desc(crate::models::issue::Column::CreatedAt)
            .fetch()
            .await?;

            Ok(Some((project.core, issues)))
        } else {
            Ok(None)
        }
    }

    /// Search projects by name or key
    pub async fn search_projects(&self, query: &str) -> Result<Vec<ProjectModel>, sea_orm::DbErr> {
        self.repo.search_admin_fields(query).await
    }
}
