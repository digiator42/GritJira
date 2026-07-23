use gritshield::GritAdmin;
use gritshield::database::GritRepository;
use sea_orm::{DatabaseConnection, DbErr};

use crate::models::{IssueModel, issue};

#[derive(Clone, GritAdmin)]
#[repository(
    searchable = [ "project_id", "sprint_id", "step_id", "reporter_id", "assignee_id", "key", "summary", "description", "priority", "issue_type", "story_points", "created_at",],
     read_only = ["created_at"],
)]
pub struct IssueRepository {
    pub db: DatabaseConnection,
}

impl IssueRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
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
}
