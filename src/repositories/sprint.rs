use gritshield::database::GritRepository;
use gritshield::{GritAdmin, GritComponent};
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, DbErr};
use sea_orm::ActiveModelTrait;
use crate::models::{SprintModel, sprint};

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    searchable = ["project_id", "name", "goal", "status", "start_date", "end_date"]
)]
pub struct SprintRepository {
    pub db: DatabaseConnection,
}

impl SprintRepository {
    pub async fn create(
        &self,
        project_id: i32,
        name: &str,
        goal: Option<String>,
    ) -> Result<sprint::Model, DbErr> {
        let new_sprint = sprint::ActiveModel {
            project_id: Set(project_id),
            name: Set(name.to_string()),
            goal: Set(goal),
            status: Set("future".to_string()),
            ..Default::default()
        };

        new_sprint.insert(&self.db).await
    }

    pub async fn start_sprint(&self, sprint_id: i32) -> Result<sprint::Model, DbErr> {
        self.update_column_value(sprint_id, "status", "active".into(), None)
            .await
    }

    pub async fn find_active_by_project(&self, project_id: i32) -> Result<Vec<SprintModel>, DbErr> {
        self.query()
            .where_eq(sprint::Column::ProjectId, project_id)
            .where_eq(sprint::Column::Status, "active")
            .fetch()
            .await
    }
}
