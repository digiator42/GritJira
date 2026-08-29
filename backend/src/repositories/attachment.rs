use crate::models::{AttachmentModel, attachment};
use gritshield::database::GritRepository;
use gritshield::{GritAdmin, GritComponent};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder, Set};

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(entity = "crate::models::attachment")]
pub struct AttachmentRepository {
    pub db: DatabaseConnection,
}

impl AttachmentRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_for_issue(&self, issue_id: i32) -> Result<Vec<attachment::Model>, DbErr> {
        attachment::Entity::find()
            .filter(attachment::Column::IssueId.eq(issue_id))
            .order_by_desc(attachment::Column::Id)
            .all(&self.db)
            .await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<attachment::Model>, DbErr> {
        attachment::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn create(
        &self,
        project_id: i32,
        issue_id: i32,
        uploader_id: i32,
        filename: String,
        mime_type: String,
        size_bytes: i32,
        storage_key: String,
    ) -> Result<attachment::Model, DbErr> {
        let model = attachment::ActiveModel {
            project_id: Set(project_id),
            issue_id: Set(issue_id),
            uploader_id: Set(uploader_id),
            filename: Set(filename),
            mime_type: Set(mime_type),
            size_bytes: Set(size_bytes),
            storage_key: Set(storage_key),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        model.insert(&self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), DbErr> {
        attachment::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }
}