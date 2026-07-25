// src/services/board_service.rs
use crate::models::{IssueModel, SprintModel, WorkflowStepModel, issue, sprint, workflow};
use crate::repositories::issue::IssueRepository;
use crate::repositories::sprint::SprintRepository;
use crate::repositories::workflow::WorkflowRepository;
use crate::services::workflow_engine::WorkflowEngine;
use gritshield::GritComponent;
use sea_orm::{DatabaseConnection, DbErr};
use std::sync::Arc;
use gritshield::database::GritRepository;

#[derive(Clone, GritComponent)]
pub struct BoardService {
    pub workflow_engine: Arc<WorkflowEngine>,
    pub workflow_repo: Arc<WorkflowRepository>,
    pub issue_repo: Arc<IssueRepository>,
    pub sprint_repo: Arc<SprintRepository>,
    pub db: DatabaseConnection,
}

impl BoardService {
    pub async fn transition_issue(
        &self,
        current_step: i32,
        target_step: i32,
    ) -> Result<bool, String> {
        if !self
            .workflow_engine
            .can_transition(current_step, target_step)
        {
            return Err("Invalid transition: Workflow step transition blocked".to_string());
        }

        Ok(true)
    }

    pub async fn get_sprint_board_data(
        &self,
        project_id: i32,
        sprint_id: i32,
    ) -> Result<Vec<(workflow::Model, Vec<issue::Model>)>, DbErr> {
        // Get workflow steps using query builder
        let steps = self
            .workflow_repo
            .query()
            .where_eq(workflow::Column::ProjectId, project_id)
            .order_asc(workflow::Column::Position)
            .fetch()
            .await?;

        // Get sprint issues using query builder
        let sprint_issues = self
            .issue_repo
            .query()
            .where_eq(issue::Column::SprintId, sprint_id)
            .fetch()
            .await?;

        // Build the board structure
        let board = steps
            .into_iter()
            .map(|step| {
                let column_issues = sprint_issues
                    .iter()
                    .filter(|issue| issue.step_id == step.id)
                    .cloned()
                    .collect();
                (step, column_issues)
            })
            .collect();

        Ok(board)
    }

    pub async fn move_issue(
        &self,
        issue_id: i32,
        target_step_id: i32,
    ) -> Result<issue::Model, DbErr> {
        self.issue_repo.update_step(issue_id, target_step_id).await
    }

    /// Fetch all issues where sprint_id is None
    pub async fn get_backlog_issues(&self) -> Result<Vec<issue::Model>, DbErr> {
        self.issue_repo
            .query()
            .where_null(issue::Column::SprintId)
            .fetch()
            .await
    }

    /// Fetch active sprints for a given project
    pub async fn get_active_sprints(&self, project_id: i32) -> Result<Vec<sprint::Model>, DbErr> {
        self.sprint_repo
            .query()
            .where_eq(sprint::Column::ProjectId, project_id)
            .where_eq(sprint::Column::Status, "Active")
            .fetch()
            .await
    }

    /// Assign or remove an issue from a sprint
    pub async fn assign_sprint(
        &self,
        issue_id: i32,
        sprint_id: i32,
    ) -> Result<issue::Model, DbErr> {
        self.issue_repo.update_step(issue_id, sprint_id).await
    }

    /// Get the active sprint for a project
    pub async fn get_active_sprint(&self, project_id: i32) -> Result<SprintModel, DbErr> {
        self.sprint_repo
            .query()
            .where_eq(sprint::Column::ProjectId, project_id)
            .where_eq(sprint::Column::Status, "Active")
            .fetch_one()
            .await
    }

    /// Get the first sprint for a project (fallback)
    pub async fn get_first_sprint(&self, project_id: i32) -> Result<SprintModel, DbErr> {
        self.sprint_repo
            .query()
            .where_eq(sprint::Column::ProjectId, project_id)
            .order_asc(sprint::Column::Id)
            .fetch_one()
            .await
    }

    /// Get kanban columns for a specific project and sprint
    pub async fn get_kanban_columns(
        &self,
        project_id: i32,
        sprint_id: i32,
    ) -> Result<Vec<(WorkflowStepModel, Vec<IssueModel>)>, DbErr> {
        // Get all workflow steps for this project
        let steps = self
            .workflow_repo
            .query()
            .where_eq(workflow::Column::ProjectId, project_id)
            .order_asc(workflow::Column::Position)
            .fetch()
            .await?;

        // For each step, get the issues in that step for this sprint
        let mut result = Vec::new();
        for step in steps {
            let issues = self
                .issue_repo
                .query()
                .where_eq(issue::Column::SprintId, sprint_id)
                .where_eq(issue::Column::StepId, step.id)
                .order_asc(issue::Column::Id)
                .fetch()
                .await?;

            result.push((step, issues));
        }

        Ok(result)
    }

    /// Get board with eager loaded relations (using find_by_* with with_*)
    pub async fn get_board_with_relations(
        &self,
        project_id: i32,
        sprint_id: i32,
    ) -> Result<Vec<(WorkflowStepModel, Vec<IssueModel>)>, DbErr> {
        // Get workflow steps with their issues loaded eagerly
        // Using the repository's find_with_* pattern
        let workflow_repo = self.workflow_repo.clone();

        // First get all steps
        let steps = workflow_repo
            .query()
            .where_eq(workflow::Column::ProjectId, project_id)
            .order_asc(workflow::Column::Position)
            .fetch()
            .await?;

        // For each step, find its issues using the issue repository with with_step
        let mut result = Vec::new();
        for step in steps {
            // Using find_by_* with with_* relation
            let issues = self
                .issue_repo
                .query()
                .where_eq(issue::Column::StepId, step.id)
                .where_eq(issue::Column::SprintId, sprint_id)
                .order_asc(issue::Column::Id)
                .fetch()
                .await?;

            result.push((step, issues));
        }

        Ok(result)
    }
}
