use crate::models::{webhook, WebhookModel};
use gritshield::database::GritRepository;
use gritshield::{GritAdmin, GritComponent};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder};

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(entity = "crate::models::webhook")]
pub struct WebhookRepository {
    pub db: DatabaseConnection,
}

impl WebhookRepository {
    pub async fn list_by_project(&self, project_id: i32) -> Result<Vec<WebhookModel>, DbErr> {
        use crate::models::webhook::{self, Entity as Webhook};
        Webhook::find()
            .filter(webhook::Column::ProjectId.eq(project_id))
            .order_by_asc(webhook::Column::Id)
            .all(&self.db)
            .await
    }

    pub async fn list_for_event(
        &self,
        project_id: i32,
        event: &str,
    ) -> Result<Vec<WebhookModel>, DbErr> {
        use crate::models::webhook::{self, Entity as Webhook};
        Webhook::find()
            .filter(webhook::Column::ProjectId.eq(project_id))
            .filter(webhook::Column::IsActive.eq(true))
            .filter(webhook::Column::Event.is_in([event.to_string(), "*".to_string()]))
            .all(&self.db)
            .await
    }

    pub async fn create(
        &self,
        project_id: i32,
        name: &str,
        url: &str,
        event: &str,
    ) -> Result<WebhookModel, DbErr> {
        let m = webhook::ActiveModel {
            project_id: Set(project_id),
            name: Set(name.to_string()),
            url: Set(url.to_string()),
            event: Set(event.to_string()),
            is_active: Set(true),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        m.insert(&self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<bool, DbErr> {
        use crate::models::webhook::Entity as Webhook;
        let res = Webhook::delete_by_id(id).exec(&self.db).await?;
        Ok(res.rows_affected > 0)
    }
}