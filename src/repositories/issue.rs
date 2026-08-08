use crate::models::{IssueModel, issue};
use gritshield::database::GritRepository;
use gritshield::{GritAdmin, GritComponent};
use sea_orm::FromQueryResult;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    searchable = [
        "project_id",
        "sprint_id",
        "step_id",
        "reporter_id",
        "assignee_id",
        "key",
        "summary",
        "description",
        "priority",
        "issue_type",
        "story_points",
        "created_at",
    ],
    read_only = ["created_at"]
)]
pub struct IssueRepository {
    pub db: DatabaseConnection,
}

impl IssueRepository {
    pub async fn create(
        &self,
        project_id: i32,
        summary: &str,
        description: &str,
        issue_type: &str,
        priority: i32,
        reporter_id: i32,
        sprint_id: Option<i32>,
    ) -> Result<issue::Model, DbErr> {
        let new_issue = issue::ActiveModel {
            project_id: Set(project_id),
            key: Set(format!("GRIT-{}", chrono::Utc::now().timestamp_millis())),
            summary: Set(summary.to_string()),
            description: Set(Some(description.to_string())),
            issue_type: Set(issue_type.to_string()),
            priority: Set(priority.to_string()),
            reporter_id: Set(reporter_id),
            sprint_id: Set(sprint_id),
            step_id: Set(1), // Default to initial step ("To Do")
            ..Default::default()
        };

        new_issue.insert(&self.db).await
    }

    /// Update assignee for an issue
    pub async fn update_assignee(
        &self,
        issue_id: i32,
        assignee_id: Option<i32>,
    ) -> Result<issue::Model, DbErr> {
        let assignee_val = assignee_id.map(|id| id.to_string()).unwrap_or_default();
        self.update_column_value(issue_id, "assignee_id", assignee_val, None)
            .await
    }

    /// Executes a compiled JQL SeaORM Statement directly against the DB
    pub async fn find_by_statement(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<Vec<IssueModel>, DbErr> {
        IssueModel::find_by_statement(stmt).all(&self.db).await
    }

    /// Fetch issues for a specific sprint using GritShield QueryBuilder
    pub async fn find_by_sprint(&self, sprint_id: i32) -> Result<Vec<IssueModel>, DbErr> {
        self.query()
            .where_eq(issue::Column::SprintId, sprint_id)
            .fetch()
            .await
    }

    /// Fetch issues for a specific step/column using GritShield QueryBuilder
    pub async fn find_by_step(&self, step_id: i32) -> Result<Vec<issue::Model>, DbErr> {
        self.query()
            .where_eq(issue::Column::StepId, step_id)
            .fetch()
            .await
    }

    /// Move an issue to a new step using GritShield's dynamic column updater
    pub async fn update_step(
        &self,
        issue_id: i32,
        target_step_id: i32,
    ) -> Result<issue::Model, DbErr> {
        self.update_column_value(issue_id, "step_id", target_step_id.to_string(), None)
            .await
    }
    // Add inside impl IssueRepository:

    pub async fn find_unassigned_backlog(&self) -> Result<Vec<issue::Model>, DbErr> {
        self.query()
            .where_null(issue::Column::SprintId)
            .fetch()
            .await
    }

    pub async fn update_sprint(
        &self,
        issue_id: i32,
        sprint_id: Option<i32>,
    ) -> Result<issue::Model, DbErr> {
        use crate::models::issue::{self, Entity as Issue};

        let issue = Issue::find_by_id(issue_id).one(&self.db).await?;

        if let Some(i) = issue {
            let mut active: issue::ActiveModel = i.into();
            active.sprint_id = Set(sprint_id);
            let updated = active.update(&self.db).await?;
            Ok(updated)
        } else {
            Err(DbErr::RecordNotFound("Issue not found".to_string()))
        }
    }

    pub async fn delete_issue(&self, issue_id: i32) -> Result<bool, DbErr> {
        use crate::models::issue::Entity as Issue;

        let result = Issue::delete_by_id(issue_id).exec(&self.db).await?;

        Ok(result.rows_affected > 0)
    }

    pub async fn update_issue(
        &self,
        issue_id: i32,
        summary: Option<&str>,
        description: Option<&str>,
        priority: Option<i32>,
        issue_type: Option<&str>,
        story_points: Option<i32>,
    ) -> Result<Option<issue::Model>, DbErr> {
        use crate::models::issue::{self, Entity as Issue};

        let issue = Issue::find_by_id(issue_id).one(&self.db).await?;

        if let Some(i) = issue {
            let mut active: issue::ActiveModel = i.into();
            if let Some(summary) = summary {
                active.summary = Set(summary.to_string());
            }
            if let Some(description) = description {
                active.description = Set(Some(description.to_string()));
            }
            if let Some(priority) = priority {
                active.priority = Set(priority.to_string());
            }
            if let Some(issue_type) = issue_type {
                active.issue_type = Set(issue_type.to_string());
            }
            if let Some(story_points) = story_points {
                active.story_points = Set(Some(story_points));
            }
            let updated = active.update(&self.db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }
}
