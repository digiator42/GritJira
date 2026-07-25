use crate::models::{IssueModel, WorkflowStepModel, issue, sprint, workflow};
use crate::repositories::issue::IssueRepository;
use crate::repositories::sprint::SprintRepository;
use crate::repositories::workflow::WorkflowRepository;
use crate::services::workflow_engine::WorkflowEngine;
use gritshield::GritComponent;
use sea_orm::DbErr;
use std::sync::Arc;
use sea_orm::ConnectionTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;

#[derive(Clone, GritComponent)]
pub struct BoardService {
    pub workflow_engine: Arc<WorkflowEngine>,
    pub workflow_repo: Arc<WorkflowRepository>,
    pub issue_repo: Arc<IssueRepository>,
    pub sprint_repo: Arc<SprintRepository>,
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
        let steps = self.workflow_repo.find_steps_by_project(project_id).await?;
        let sprint_issues = self.issue_repo.find_by_sprint(sprint_id).await?;

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
        self.issue_repo.find_unassigned_backlog().await
    }

    /// Fetch active sprints for a given project
    pub async fn get_active_sprints(&self, project_id: i32) -> Result<Vec<sprint::Model>, DbErr> {
        self.sprint_repo.find_active_by_project(project_id).await
    }

    /// Assign or remove an issue from a sprint
    pub async fn assign_sprint(
        &self,
        issue_id: i32,
        sprint_id: i32,
    ) -> Result<issue::Model, DbErr> {
        self.issue_repo.update_step(issue_id, sprint_id).await
    }

    /// Returns workflow steps paired with their respective sprint issues
    pub async fn get_kanban_columns(
        &self,
        sprint_id: i32,
    ) -> Result<Vec<(WorkflowStepModel, Vec<IssueModel>)>, DbErr> {
        // 1. Fetch all workflow steps ordered by position
        let steps = workflow::Entity::find()
            .order_by_asc(workflow::Column::Position)
            .all(&self.issue_repo.db)
            .await?;

        // 2. Fetch all issues assigned to the active sprint
        let sprint_issues = issue::Entity::find()
            .filter(issue::Column::SprintId.eq(sprint_id))
            .all(&self.issue_repo.db)
            .await?;

        // 3. Group issues by workflow step ID
        let mut columns = Vec::new();
        for step in steps {
            let step_issues: Vec<IssueModel> = sprint_issues
                .iter()
                .filter(|i| i.step_id == step.id)
                .cloned()
                .collect();

            columns.push((step, step_issues));
        }

        Ok(columns)
    }
}
