use crate::models::{WorkflowStepModel, workflow};
use gritshield::GritAdmin;
use gritshield::database::GritRepository;
use sea_orm::{DatabaseConnection, DbErr};

#[derive(Clone, GritAdmin)]
#[repository(
    // searchable = ["id", "post_id", "created_at", "content", "user_id",],
    // grid_columns = ["id", "post_id", "user_id", "content", "created_at"],
    read_only = ["created_at"],
)]
pub struct WorkflowRepository {
    pub db: DatabaseConnection,
}

impl WorkflowRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

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
