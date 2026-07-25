use gritshield::database::GritRepository;
use gritshield::{GritAdmin, GritComponent};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use sea_orm::FromQueryResult;
use crate::models::{IssueModel, issue};

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
    pub async fn find_by_statement(&self, stmt: sea_orm::Statement) -> Result<Vec<IssueModel>, DbErr> {
        IssueModel::find_by_statement(stmt)
            .all(&self.db)
            .await
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
        let sprint_value = sprint_id.map(|id| id.to_string()).unwrap_or_default();
        self.update_column_value(issue_id, "sprint_id", sprint_value, None)
            .await
    }
}
