use chrono::Utc;
use gritshield::GritComponent;
use gritshield::GritJobExt;
use gritshield::database::repository::JqlCompiler;
use gritshield::routing::RequestContext;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use serde::Deserialize;
use std::sync::Arc;

use crate::dtos::{AddCommentPayload, CreateIssuePayload};
use crate::events::{CommentAdded, IssueCreated, IssueTransitioned};
use crate::jobs::SendIssueDigestJob;
use crate::models::IssueModel;
use crate::models::SprintModel;
use crate::models::WorkflowStepModel;
use crate::models::issue::GritRepositoryRecord;
use crate::models::sprint;
use crate::models::workflow;
use crate::models::{comment, issue};
use crate::repositories::comment::CommentRepository;
use crate::repositories::issue::IssueRepository;
use crate::repositories::sprint::SprintRepository;
use crate::models::issue::{ActiveModel, Column, Entity as IssueEntity};
use crate::services::JqlParser;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;

#[derive(Deserialize)]
pub struct AssignIssuePayload {
    /// Pass `Some(id)` to assign, or `None` / `null` to unassign
    pub assignee_id: Option<i32>,
}

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
                payload.description.as_deref().unwrap_or_default(),
                &payload.issue_type,
                payload.priority,
                reporter_id,
                active_sprint_id,
            )
            .await?;

        // 1. Publish Event
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
        let _ = job.enqueue().await;

        Ok(issue)
    }

    /// Updates the assignee of an issue
    pub async fn assign_issue(
        &self,
        issue_id: i32,
        assignee_id: Option<i32>,
    ) -> Result<issue::Model, DbErr> {
        self.issue_repo.update_assignee(issue_id, assignee_id).await
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

    /// Parses JQL and executes it directly against SeaORM
    pub async fn search_issues(
        &self,
        raw_jql: &str,
        db_conn: &DatabaseConnection,
        jql_parser: &JqlParser,
    ) -> Result<Vec<IssueModel>, String> {
        // 1. Get database backend dialect (PostgreSQL, MySQL, SQLite)
        let backend = db_conn.get_database_backend();

        // 2. Compile JQL to SeaORM Statement
        let stmt = jql_parser.compile_jql(raw_jql, "issues", backend)?;

        // 3. Execute statement
        self.issue_repo
            .find_by_statement(stmt)
            .await
            .map_err(|e| format!("Database query error: {}", e))
    }

    /// Fetches all backlog issues for a project (where sprint_id is NULL)
    pub async fn get_backlog_issues(&self, project_id: i32) -> Result<Vec<IssueModel>, DbErr> {
        issue::Entity::find()
            .filter(issue::Column::ProjectId.eq(project_id))
            .filter(issue::Column::SprintId.is_null())
            .order_by_desc(issue::Column::CreatedAt)
            .all(&self.issue_repo.db)
            .await
    }

    /// Fetches all active/planned sprints for a project
    pub async fn get_project_sprints(&self, project_id: i32) -> Result<Vec<SprintModel>, DbErr> {
        sprint::Entity::find()
            .filter(sprint::Column::ProjectId.eq(project_id))
            .order_by_desc(sprint::Column::StartDate)
            .all(&self.issue_repo.db)
            .await
    }

    /// Create issue with explicit step_id
    pub async fn create_issue_with_step(
        &self,
        payload: CreateIssuePayload,
        project_id: i32,
        reporter_id: i32,
        step_id: i32,
        ctx: &RequestContext,
    ) -> Result<IssueModel, String> {
        // Generate issue key
        let project_key = ctx
            .get_session_data("current_project_key")
            .unwrap_or_else(|| "PROJ".to_string());

        // Use IssueEntity explicitly
        let count = IssueEntity::find()
            .filter(Column::ProjectId.eq(project_id))
            .count(&self.issue_repo.db)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let issue_key = format!("{}-{}", project_key, count + 1);

        let new_issue = ActiveModel {
            project_id: Set(project_id),
            key: Set(issue_key),
            summary: Set(payload.summary),
            description: Set(payload.description),
            issue_type: Set(payload.issue_type),
            priority: Set(payload.priority.to_string()),
            step_id: Set(step_id),
            reporter_id: Set(reporter_id),
            assignee_id: Set(payload.assignee_id),
            sprint_id: Set(payload.sprint_id),
            story_points: Set(payload.story_points),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        new_issue
            .insert(&self.issue_repo.db)
            .await
            .map_err(|e| format!("Database error: {}", e))
    }

    /// Get the first workflow step for a project (lowest position)
    pub async fn get_first_workflow_step(
        &self,
        project_id: i32,
    ) -> Result<Option<workflow::Model>, sea_orm::DbErr> {
        use crate::models::workflow::{self, Entity as Workflow};
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

        Workflow::find()
            .filter(workflow::Column::ProjectId.eq(project_id))
            .order_by_asc(workflow::Column::Position)
            .one(&self.issue_repo.db)
            .await
    }

    /// Update issue details
    pub async fn update_issue(
        &self,
        issue_id: i32,
        summary: Option<&str>,
        description: Option<&str>,
        priority: Option<i32>,
        issue_type: Option<&str>,
        story_points: Option<i32>,
    ) -> Result<Option<issue::Model>, DbErr> {
        self.issue_repo
            .update_issue(issue_id, summary, description, priority, issue_type, story_points)
            .await
    }

    /// Delete issue
    pub async fn delete_issue(&self, issue_id: i32) -> Result<bool, DbErr> {
        self.issue_repo.delete_issue(issue_id).await
    }
}
