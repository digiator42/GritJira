use gritshield::GritComponent;
use sea_orm::DbErr;
use std::sync::Arc;

use crate::dtos::{AddCommentPayload, CreateIssuePayload};
use crate::events::{CommentAdded, IssueCreated};
use crate::models::{comment, issue};
use crate::repositories::comment::CommentRepository;
use crate::repositories::issue::IssueRepository;

#[derive(Clone, GritComponent)]
pub struct IssueService {
    pub issue_repo: Arc<IssueRepository>,
    pub comment_repo: Arc<CommentRepository>,
}

impl IssueService {
    /// Creates a new issue and dispatches the `IssueCreated` event
    pub async fn create_issue(
        &self,
        payload: CreateIssuePayload,
        reporter_id: i32,
        ctx: &gritshield::routing::RequestContext,
    ) -> Result<issue::Model, DbErr> {
        let issue = self
            .issue_repo
            .create(
                &payload.summary,
                &payload.description,
                &payload.issue_type,
                payload.priority,
                reporter_id,
            )
            .await?;

        // Publish event asynchronously via EventBus
        ctx.event_bus.publish(IssueCreated {
            issue_id: issue.id,
            key: issue.key.clone(),
            summary: issue.summary.clone(),
            reporter_id,
        });

        Ok(issue)
    }

    /// Adds a comment to an issue and dispatches `CommentAdded`
    pub async fn add_comment(
        &self,
        issue_id: i32,
        payload: AddCommentPayload,
        author_id: i32,
        ctx: &gritshield::routing::RequestContext,
    ) -> Result<comment::Model, DbErr> {
        let comment = self
            .comment_repo
            .create(issue_id, author_id, &payload.body)
            .await?;

        ctx.event_bus.publish(CommentAdded {
            comment_id: comment.id,
            issue_id,
            author_id,
        });

        Ok(comment)
    }
}
