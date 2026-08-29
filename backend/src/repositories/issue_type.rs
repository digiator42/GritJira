use crate::models::{IssueTypeModel, issue_type};
use gritshield::database::GritRepository;
use gritshield::{GritAdmin, GritComponent};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder, Set};

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(entity = "crate::models::issue_type")]
pub struct IssueTypeRepository {
    pub db: DatabaseConnection,
}

impl IssueTypeRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_project(&self, project_id: i32) -> Result<Vec<IssueTypeModel>, DbErr> {
        issue_type::Entity::find()
            .filter(issue_type::Column::ProjectId.eq(project_id))
            .order_by_asc(issue_type::Column::Position)
            .order_by_asc(issue_type::Column::Id)
            .all(&self.db)
            .await
    }

    pub async fn find_one(&self, id: i32) -> Result<Option<IssueTypeModel>, DbErr> {
        issue_type::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn create(
        &self,
        project_id: i32,
        name: &str,
        icon_key: &str,
        color: &str,
    ) -> Result<IssueTypeModel, DbErr> {
        use crate::models::issue_type::Entity as IssueType;

        let max_pos = IssueType::find()
            .filter(issue_type::Column::ProjectId.eq(project_id))
            .order_by_desc(issue_type::Column::Position)
            .one(&self.db)
            .await?;

        let next_pos = max_pos.map(|t| t.position + 1).unwrap_or(0);

        let model = issue_type::ActiveModel {
            project_id: Set(project_id),
            name: Set(name.to_string()),
            icon_key: Set(if icon_key.trim().is_empty() {
                "task".to_string()
            } else {
                icon_key.to_string()
            }),
            color: Set(if color.trim().is_empty() {
                "#4bade9".to_string()
            } else {
                color.to_string()
            }),
            position: Set(next_pos),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        model.insert(&self.db).await
    }

    pub async fn update_type(
        &self,
        id: i32,
        name: Option<&str>,
        icon_key: Option<&str>,
        color: Option<&str>,
    ) -> Result<Option<IssueTypeModel>, DbErr> {
        let existing = issue_type::Entity::find_by_id(id).one(&self.db).await?;

        if let Some(existing) = existing {
            let mut active: issue_type::ActiveModel = existing.into();
            if let Some(name) = name {
                active.name = Set(name.to_string());
            }
            if let Some(icon_key) = icon_key {
                active.icon_key = Set(icon_key.to_string());
            }
            if let Some(color) = color {
                active.color = Set(color.to_string());
            }
            let updated = active.update(&self.db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_type(&self, id: i32) -> Result<bool, DbErr> {
        let result = issue_type::Entity::delete_by_id(id).exec(&self.db).await?;
        Ok(result.rows_affected > 0)
    }
}