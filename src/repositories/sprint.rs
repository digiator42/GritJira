use gritshield::database::GritRepository;
use gritshield::{GritAdmin, GritComponent};
use sea_orm::{DatabaseConnection, DbErr};

use crate::models::{SprintModel, sprint};

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    searchable = ["project_id", "name", "goal", "status", "start_date", "end_date"]
)]
pub struct SprintRepository {
    pub db: DatabaseConnection,
}

impl SprintRepository {
    pub async fn find_active_by_project(&self, project_id: i32) -> Result<Vec<SprintModel>, DbErr> {
        self.query()
            .where_eq(sprint::Column::ProjectId, project_id)
            .where_eq(sprint::Column::Status, "active")
            .fetch()
            .await
    }
}
