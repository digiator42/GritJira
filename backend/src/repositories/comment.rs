use gritshield::database::GritRepository;
use gritshield::{GritAdmin, GritComponent};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

use crate::models::{CommentModel, comment};

#[derive(Clone, GritAdmin, GritComponent)]
#[repository(
    searchable = ["issue_id", "author_id", "body", "created_at"],
    read_only = ["created_at"]
)]
pub struct CommentRepository {
    pub db: DatabaseConnection,
}

impl CommentRepository {
    pub async fn create(
        &self,
        issue_id: i32,
        author_id: i32,
        body: &str,
    ) -> Result<comment::Model, DbErr> {
        let new_comment = comment::ActiveModel {
            issue_id: Set(issue_id),
            author_id: Set(author_id),
            body: Set(body.to_string()),
            ..Default::default()
        };

        new_comment.insert(&self.db).await
    }

    pub async fn find_by_issue(&self, issue_id: i32) -> Result<Vec<CommentModel>, DbErr> {
        self.query()
            .where_eq(comment::Column::IssueId, issue_id)
            .fetch()
            .await
    }
}
