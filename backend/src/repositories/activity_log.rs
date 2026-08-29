use gritshield::{GritAdmin, GritComponent};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

use crate::models::{ActivityLogModel, activity_log};

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    entity = "crate::models::activity_log",
    searchable = ["project_id", "actor_id", "action", "issue_id", "target_user_id", "created_at"],
    read_only = ["created_at"],
)]
pub struct ActivityLogRepository {
    pub db: DatabaseConnection,
}

#[allow(clippy::too_many_arguments)]
impl ActivityLogRepository {
    pub async fn record(
        &self,
        project_id: i32,
        actor_id: i32,
        action: &str,
        issue_id: Option<i32>,
        issue_key: Option<&str>,
        summary: Option<&str>,
        detail: Option<&str>,
        target_user_id: Option<i32>,
    ) -> Result<activity_log::Model, DbErr> {
        let entry = activity_log::ActiveModel {
            project_id: Set(project_id),
            actor_id: Set(actor_id),
            action: Set(action.to_string()),
            issue_id: Set(issue_id),
            issue_key: Set(issue_key.map(str::to_string)),
            summary: Set(summary.map(str::to_string)),
            detail: Set(detail.map(str::to_string)),
            target_user_id: Set(target_user_id),
            is_read: Set(false),
            ..Default::default()
        };

        entry.insert(&self.db).await
    }

    /// Latest activity for a project (audit trail), most recent first.
    pub async fn list_by_project(
        &self,
        project_id: i32,
        limit: usize,
    ) -> Result<Vec<ActivityLogModel>, DbErr> {
        let mut entries = activity_log::Entity::find()
            .filter(activity_log::Column::ProjectId.eq(project_id))
            .order_by_desc(activity_log::Column::CreatedAt)
            .order_by_desc(activity_log::Column::Id)
            .all(&self.db)
            .await?;

        entries.truncate(limit);
        Ok(entries)
    }

    /// Activity targeted at a user (notifications feed), most recent first.
    pub async fn list_for_user(
        &self,
        user_id: i32,
        project_id: i32,
        limit: usize,
    ) -> Result<Vec<ActivityLogModel>, DbErr> {
        let mut entries = activity_log::Entity::find()
            .filter(activity_log::Column::TargetUserId.eq(user_id))
            .filter(activity_log::Column::ProjectId.eq(project_id))
            .order_by_desc(activity_log::Column::CreatedAt)
            .order_by_desc(activity_log::Column::Id)
            .all(&self.db)
            .await?;

        entries.truncate(limit);
        Ok(entries)
    }

    pub async fn unread_count_for_user(
        &self,
        user_id: i32,
        project_id: i32,
    ) -> Result<u64, DbErr> {
        activity_log::Entity::find()
            .filter(activity_log::Column::TargetUserId.eq(user_id))
            .filter(activity_log::Column::ProjectId.eq(project_id))
            .filter(activity_log::Column::IsRead.eq(false))
            .count(&self.db)
            .await
    }

    /// Marks all unread notifications for a user read. Returns rows affected.
    pub async fn mark_read_for_user(
        &self,
        user_id: i32,
        project_id: i32,
    ) -> Result<u64, DbErr> {
        let mut entries = activity_log::Entity::find()
            .filter(activity_log::Column::TargetUserId.eq(user_id))
            .filter(activity_log::Column::ProjectId.eq(project_id))
            .filter(activity_log::Column::IsRead.eq(false))
            .all(&self.db)
            .await?;

        let count = entries.len() as u64;
        for entry in entries.drain(..) {
            let mut active: activity_log::ActiveModel = entry.into();
            active.is_read = Set(true);
            active.update(&self.db).await?;
        }
        Ok(count)
    }
}