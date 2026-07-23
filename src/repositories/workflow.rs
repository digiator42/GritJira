use crate::models::{WorkflowStepModel, workflow};
use gritshield::GritAdmin;
use gritshield::database::GritRepository;
use sea_orm::{DatabaseConnection, DbErr};
use gritshield::GritComponent;

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    searchable = ["id", "project_id", "name", "position", "is_completed",],
    read_only = ["is_completed"],
)]
pub struct WorkflowRepository {
    pub db: DatabaseConnection,
}

impl WorkflowRepository {
    /// Fetch steps ordered by board position using GritShield QueryBuilder
    pub async fn find_steps_by_project(
        &self,
        project_id: i32,
    ) -> Result<Vec<WorkflowStepModel>, DbErr> {
        self.query()
            .where_eq(workflow::Column::ProjectId, project_id)
            .order_asc(workflow::Column::Position)
            .fetch()
            .await
    }
}
