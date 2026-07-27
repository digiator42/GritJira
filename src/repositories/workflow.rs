use crate::models::{WorkflowStepModel, workflow};
use gritshield::GritAdmin;
use gritshield::GritComponent;
use gritshield::database::GritRepository;
use crate::models::WorkflowStep;
use sea_orm::{ActiveModelTrait, DbErr, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set};


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

    pub async fn create_step(&self, project_id: i32) -> Result<WorkflowStepModel, sea_orm::DbErr> {
        use crate::models::workflow::{self, ActiveModel};
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

        // Get max position
        let max_pos = WorkflowStep::find()
            .filter(workflow::Column::ProjectId.eq(project_id))
            .order_by_desc(workflow::Column::Position)
            .one(&self.db)
            .await?;

        let next_pos = max_pos.map(|s| s.position + 1).unwrap_or(0);

        let new_step = ActiveModel {
            project_id: Set(project_id),
            name: Set("New Step".to_string()),
            position: Set(next_pos),
            is_completed: Set(false),
            ..Default::default()
        };

        new_step.insert(&self.db).await
    }

    pub async fn update_step(
        &self,
        step_id: i32,
        name: Option<&str>,
    ) -> Result<Option<WorkflowStepModel>, sea_orm::DbErr> {
        use crate::models::workflow::{self, Entity as WorkflowStep};

        let step = WorkflowStep::find_by_id(step_id).one(&self.db).await?;

        if let Some(s) = step {
            let mut active: workflow::ActiveModel = s.into();
            if let Some(name) = name {
                active.name = Set(name.to_string());
            }
            let updated = active.update(&self.db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    pub async fn toggle_completed(
        &self,
        step_id: i32,
    ) -> Result<Option<WorkflowStepModel>, sea_orm::DbErr> {
        use crate::models::workflow::{self, Entity as WorkflowStep};

        let step = WorkflowStep::find_by_id(step_id).one(&self.db).await?;

        if let Some(s) = step {
            let mut active: workflow::ActiveModel = s.into();
            active.is_completed = Set(!active.is_completed.unwrap());
            let updated = active.update(&self.db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_step(&self, step_id: i32) -> Result<bool, sea_orm::DbErr> {
        use crate::models::workflow::Entity as WorkflowStep;

        let result = WorkflowStep::delete_by_id(step_id).exec(&self.db).await?;

        Ok(result.rows_affected > 0)
    }
}
