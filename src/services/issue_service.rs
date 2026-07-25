use gritshield::GritComponent;
use gritshield::GritJobExt;
use gritshield::routing::RequestContext;
use sea_orm::DbErr;
use std::sync::Arc;

use crate::dtos::{AddCommentPayload, CreateIssuePayload};
use crate::events::{CommentAdded, IssueCreated, IssueTransitioned};
use crate::jobs::SendIssueDigestJob;
use crate::models::IssueModel;
use crate::models::issue::GritRepositoryRecord;
use crate::models::{comment, issue};
use crate::repositories::comment::CommentRepository;
use crate::repositories::issue::IssueRepository;
use crate::repositories::sprint::SprintRepository;

#[derive(Clone, GritComponent)]
pub struct IssueService {
    pub issue_repo: Arc<IssueRepository>,
    pub comment_repo: Arc<CommentRepository>,
    pub sprint_repo: Arc<SprintRepository>,
}

impl IssueService {
    pub async fn create_issue(
        &self,
        payload: CreateIssuePayload,
        project_id: i32,
        reporter_id: i32,
        ctx: &gritshield::routing::RequestContext,
    ) -> Result<issue::Model, DbErr> {
        let active_sprint_id = self
            .sprint_repo
            .find_active_by_project(project_id)
            .await
            .ok()
            .and_then(|sprints| sprints.into_iter().next().map(|s| s.id));

        let issue = self
            .issue_repo
            .create(
                project_id,
                &payload.summary,
                &payload.description,
                &payload.issue_type,
                payload.priority,
                reporter_id,
                active_sprint_id,
            )
            .await?;

        // 1. Publish Event[cite: 36]
        ctx.event_bus.publish(IssueCreated {
            issue_id: issue.id,
            key: issue.key.clone(),
            summary: issue.summary.clone(),
            reporter_id,
        });

        // 2. Queue Email Digest Notification Job in Background
        let job = SendIssueDigestJob {
            issue_id: issue.id,
            recipient_emails: vec!["team-lead@gritshield.io".to_string()],
        };
        let _ = job.enqueue(&ctx.job_queue).await;

        Ok(issue)
    }

    pub async fn get_issue_by_id(&self, issue_id: i32) -> Result<Option<issue::Model>, DbErr> {
        let issue_record = self.issue_repo.find_one_by_id(issue_id).await?;
        Ok(issue_record.map(Into::into))
    }

    pub async fn move_issue_step(
        &self,
        issue_id: i32,
        target_step_id: i32,
        actor_id: i32,
        ctx: &RequestContext,
    ) -> Result<issue::Model, DbErr> {
        // Fetch original step for transition telemetry
        let from_step_id = self.issue_repo.find_one_by_step_id(issue_id).await?;
        let step_id = from_step_id.unwrap().id;

        let updated_issue = self
            .issue_repo
            .update_step(issue_id, target_step_id)
            .await?;

        // Publish IssueTransitioned (Triggers AuditLogHandler & SprintMetricHandler)
        ctx.event_bus.publish(IssueTransitioned {
            issue_id,
            key: updated_issue.key.clone(),
            from_step_id: step_id,
            to_step_id: target_step_id,
            actor_id,
        });

        Ok(updated_issue)
    }

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

        // Publish CommentAdded Event[cite: 33, 36]
        ctx.event_bus.publish(CommentAdded {
            comment_id: comment.id,
            issue_id,
            author_id,
        });

        Ok(comment)
    }
}
