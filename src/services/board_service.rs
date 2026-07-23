use sea_orm::{DatabaseConnection, DbErr};
use crate::models::{issue, workflow};
use crate::repositories::issue::IssueRepository;
use crate::repositories::workflow::WorkflowRepository;

pub struct BoardService {
    issue_repo: IssueRepository,
    workflow_repo: WorkflowRepository,
}

impl BoardService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            issue_repo: IssueRepository::new(db.clone()),
            workflow_repo: WorkflowRepository::new(db),
        }
    }

    /// Builds the Kanban board matrix (Columns + Issues) using GritShield queries
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

    /// Moves issue across workflow steps
    pub async fn move_issue(&self, issue_id: i32, target_step_id: i32) -> Result<issue::Model, DbErr> {
        self.issue_repo.update_step(issue_id, target_step_id).await
    }
}