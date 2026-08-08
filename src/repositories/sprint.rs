use crate::models::{SprintModel, sprint};
use chrono::Utc;
use gritshield::database::GritRepository;
use gritshield::{GritAdmin, GritComponent};
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sprint::ActiveModel;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, DbErr};

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
    ) -> Result<sprint::Model, sea_orm::DbErr> {

        let new_sprint = ActiveModel {
            project_id: Set(project_id),
            name: Set(name.to_string()),
            goal: Set(goal),
            status: Set("Planning".to_string()),
            ..Default::default()
        };

        new_sprint.insert(&self.db).await
    }

    pub async fn start_sprint(&self, sprint_id: i32) -> Result<sprint::Model, DbErr> {
        self.update_column_value(sprint_id, "status", "active".into(), None)
            .await?;
        let now = Utc::now().naive_utc().to_string();
        self.update_column_value(sprint_id, "start_date", now, None)
            .await
    }

    pub async fn find_active_by_project(&self, project_id: i32) -> Result<Vec<SprintModel>, DbErr> {
        self.query()
            .where_eq(sprint::Column::ProjectId, project_id)
            .where_eq(sprint::Column::Status, "active")
            .fetch()
            .await
    }

    pub async fn complete_sprint(&self, sprint_id: i32) -> Result<sprint::Model, DbErr> {
        self.update_column_value(sprint_id, "status", "completed".into(), None)
            .await?;
        let now = Utc::now().naive_utc().to_string();
        self.update_column_value(sprint_id, "end_date", now, None)
            .await
    }

    pub async fn delete_sprint(&self, sprint_id: i32) -> Result<bool, DbErr> {
        use crate::models::sprint::Entity as Sprint;

        let result = Sprint::delete_by_id(sprint_id).exec(&self.db).await?;

        Ok(result.rows_affected > 0)
    }

    pub async fn update_sprint(
        &self,
        sprint_id: i32,
        name: Option<&str>,
        goal: Option<&str>,
    ) -> Result<Option<sprint::Model>, DbErr> {
        use crate::models::sprint::{self, Entity as Sprint};

        let sprint = Sprint::find_by_id(sprint_id).one(&self.db).await?;

        if let Some(s) = sprint {
            let mut active: sprint::ActiveModel = s.into();
            if let Some(name) = name {
                active.name = Set(name.to_string());
            }
            if let Some(goal) = goal {
                active.goal = Set(Some(goal.to_string()));
            }
            let updated = active.update(&self.db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }
}
