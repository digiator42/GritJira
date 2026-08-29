use crate::controllers::{get_project_context, get_project_key};
use crate::controllers::attachment_controller::UploadAttachmentPayload;
use crate::dtos::{
    AddCommentPayload, CreateIssuePayload, LogTimePayload, MoveIssuePayload, UpdateIssuePayload,
};
use crate::services::webhook_service::WebhookService;
use crate::models::comment;
use crate::repositories::activity_log::ActivityLogRepository;
use crate::repositories::attachment::AttachmentRepository;
use crate::repositories::comment::CommentRepository;
use crate::security::caps::{IssueCreate, IssueEdit, ViewBoard};
use crate::services::JqlParser;
use crate::services::attachment_service::AttachmentService;
use crate::services::issue_service::IssueService;
use crate::services::project_service::ProjectService;
use gritshield::database::GritRepository;
use gritshield::http::response::HttpStatus;
use gritshield::{GritSanitizer, prelude::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use base64::Engine as _;

const MAX_ATTACHMENT_BYTES: usize = 8 * 1024 * 1024;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Deserialize, GritSanitizer)]
pub struct AssignIssuePayload {
    /// Pass `Some(id)` to assign, or `None` / `null` to unassign
    pub assignee_id: Option<i32>,
}

pub struct IssueController;

#[controller("/api/v1/issues")]
impl IssueController {
    /// GET /api/v1/issues/:id - Fetch issue details and its comments
    #[get("/:id")]
    #[cap(ViewBoard)]
    pub async fn get_issue(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        comment_repo: Arc<CommentRepository>,
        attachment_repo: Arc<AttachmentRepository>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        match issue_service.get_issue_by_id(issue_id).await {
            Ok(Some(issue)) => {
                let comments = comment_repo
                    .query()
                    .where_eq(comment::Column::IssueId, issue_id)
                    .order_asc(comment::Column::CreatedAt)
                    .fetch()
                    .await
                    .unwrap_or_default();

                let attachments = attachment_repo
                    .list_for_issue(issue_id)
                    .await
                    .unwrap_or_default();

                Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: serde_json::json!({
                            "issue": issue,
                            "comments": comments,
                            "attachments": attachments,
                        }),
                    },
                )
            }
            Ok(None) => Response::not_found("Issue not found"),
            Err(e) => Response::internal_error(e.to_string()),
        }
    }

    /// GET /api/v1/issues/:id/attachments - List attachments on an issue
    #[get("/:id/attachments")]
    #[cap(ViewBoard)]
    pub async fn get_issue_attachments(
        ctx: RequestContext,
        attachment_repo: Arc<AttachmentRepository>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };
        match attachment_repo.list_for_issue(issue_id).await {
            Ok(list) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: list,
                },
            ),
            Err(e) => Response::internal_error(e.to_string()),
        }
    }

    /// POST /api/v1/issues/:id/attachments - Upload an attachment (base64 in JSON)
    #[post("/:id/attachments")]
    #[cap(IssueEdit)]
    pub async fn upload_attachment(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        attachment_repo: Arc<AttachmentRepository>,
        attachment_service: Arc<AttachmentService>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };
        let payload = match ctx.json::<UploadAttachmentPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid attachment payload"),
        };
        if payload.filename.trim().is_empty() {
            return Response::bad_request("filename is required");
        }
        let issue = match issue_service.get_issue_by_id(issue_id).await {
            Ok(Some(i)) => i,
            Ok(None) => return Response::not_found("Issue not found"),
            Err(e) => return Response::internal_error(e.to_string()),
        };

        let bytes = match base64::engine::general_purpose::STANDARD.decode(&payload.data_base64) {
            Ok(b) => b,
            Err(e) => return Response::bad_request(format!("Invalid base64 data: {}", e)),
        };
        if bytes.is_empty() {
            return Response::bad_request("Empty file");
        }
        if bytes.len() > MAX_ATTACHMENT_BYTES {
            return Response::bad_request(format!("File too large (max {} bytes)", MAX_ATTACHMENT_BYTES));
        }

        let uploader_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        let storage_key = attachment_service.new_key(&payload.filename);
        if let Err(e) = attachment_service.write_bytes(&storage_key, &bytes) {
            return Response::internal_error(format!("Failed to store file: {}", e));
        }

        let mime = if payload.mime.trim().is_empty() {
            "application/octet-stream".to_string()
        } else {
            payload.mime.clone()
        };
        let stored = attachment_repo
            .create(
                issue.project_id,
                issue_id,
                uploader_id,
                payload.filename.clone(),
                mime,
                bytes.len() as i32,
storage_key.clone(),
            )
            .await;
        match stored {
            Ok(record) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: record,
                },
            ),
            Err(e) => {
                attachment_service.remove_bytes(&storage_key);
                Response::internal_error(e.to_string())
            }
        }
    }

    /// POST /api/v1/issues?project_id=N - Create a new issue
    #[post("")]
    #[cap(IssueCreate)]
    pub async fn create_issue(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        project_service: Arc<ProjectService>,
        activity_log_repo: Arc<ActivityLogRepository>,
        webhook_service: Arc<WebhookService>,
    ) -> Response {
        let project_id = get_project_context(&ctx);

        let payload = match ctx.json::<CreateIssuePayload>().await {
            Ok(p) => p,
            Err(e) => {
                let error_msg = format!("Invalid request body: {:?}", e);
                return Response::bad_request(error_msg);
            }
        };

        let reporter_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        let default_step_id = match issue_service.get_first_workflow_step(project_id).await {
            Ok(Some(step)) => step.id,
            Ok(None) => {
                return Response::bad_request(format!(
                    "Project {} has no workflow steps configured",
                    project_id
                ))
            }
            Err(e) => {
                return Response::bad_request(format!("Failed to get workflow step: {}", e))
            }
        };

        // Issue keys are generated from the project key (e.g. GRIT-1). Resolve it
        // from the actual project record so issues created in any project get
        // correctly prefixed keys regardless of the session's current project.
        let project_key = match project_service.get_project_by_id(project_id).await {
            Ok(Some(project)) => project.core.key.clone(),
            _ => get_project_key(&ctx),
        };

        match issue_service
            .create_issue_with_step(payload, project_id, reporter_id, default_step_id, &project_key, &ctx)
            .await
        {
            Ok(created) => {
                let _ = activity_log_repo
                    .record(
                        created.project_id,
                        reporter_id,
                        "created",
                        Some(created.id),
                        Some(&created.key),
                        Some(&created.summary),
                        Some("Issue created"),
                        created.assignee_id.filter(|a| *a != reporter_id),
                    )
                    .await;
                let _ = webhook_service
                    .fire(
                        created.project_id,
                        "issue.created",
                        &serde_json::json!({
                            "event": "issue.created",
                            "issue_id": created.id,
                            "key": created.key,
                            "summary": created.summary,
                            "issue_type": created.issue_type,
                            "priority": created.priority,
                            "reporter_id": reporter_id,
                            "assignee_id": created.assignee_id,
                            "triggered_at": chrono::Utc::now().to_rfc3339(),
                        }),
                    )
                    .await;
                Response::json(
                    HttpStatus::Created,
                    &ApiResponse {
                        success: true,
                        data: created,
                    },
                )
            }
            Err(e) => Response::bad_request(format!("Failed to create issue: {}", e)),
        }
    }

    /// PATCH /api/v1/issues/:id/step - Move issue step (Kanban workflow transition)
    #[patch("/:id/step")]
    #[cap(IssueEdit)]
    pub async fn move_step(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        activity_log_repo: Arc<ActivityLogRepository>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let payload = match ctx.json::<MoveIssuePayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid target step payload"),
        };

        let actor_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        let issue_before = issue_service.get_issue_by_id(issue_id).await.ok().flatten();

        match issue_service
            .move_issue_step(issue_id, payload.target_step_id, actor_id, &ctx)
            .await
        {
            Ok(updated) => {
                if let Some(iss) = &issue_before {
                    let _ = activity_log_repo
                        .record(
                            iss.project_id,
                            actor_id,
                            "moved",
                            Some(iss.id),
                            Some(&iss.key),
                            Some(&iss.summary),
                            Some(&format!(
                                "from step {} to step {}",
                                iss.step_id, payload.target_step_id
                            )),
                            None,
                        )
                        .await;
                }
                Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: updated,
                    },
                )
            }
            Err(e) => Response::bad_request(format!("Failed to move issue: {}", e)),
        }
    }

    /// POST /api/v1/issues/:id/comments - Add comment to issue
    #[post("/:id/comments")]
    #[cap(IssueEdit)]
    pub async fn add_comment(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        activity_log_repo: Arc<ActivityLogRepository>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let payload = match ctx.json::<AddCommentPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid comment body"),
        };

        let author_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        let issue = issue_service.get_issue_by_id(issue_id).await.ok().flatten();

        match issue_service
            .add_comment(issue_id, payload, author_id, &ctx)
            .await
        {
            Ok(comment) => {
                if let Some(iss) = &issue {
                    let mut detail = comment.body.clone();
                    if detail.chars().count() > 60 {
                        detail = detail.chars().take(60).collect::<String>() + "…";
                    }
                    let target = iss
                        .assignee_id
                        .filter(|a| *a != author_id)
                        .or_else(|| (iss.reporter_id != author_id).then_some(iss.reporter_id));
                    let _ = activity_log_repo
                        .record(
                            iss.project_id,
                            author_id,
                            "commented",
                            Some(iss.id),
                            Some(&iss.key),
                            Some(&iss.summary),
                            Some(&detail),
                            target,
                        )
                        .await;
                }
                Response::json(
                    HttpStatus::Created,
                    &ApiResponse {
                        success: true,
                        data: comment,
                    },
                )
            }
            Err(e) => Response::bad_request(format!("Failed to add comment: {}", e)),
        }
    }

    /// GET /api/v1/issues/search?jql=project_id = 1 AND priority = 1
    #[get("/search")]
    #[cap(ViewBoard)]
    pub async fn search_issues(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        jql_parser: Arc<JqlParser>,
    ) -> Response {
        let jql_query = ctx
            .query
            .get("jql")
            .and_then(|v| v.first())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "project_id = 1".to_string());

        match issue_service
            .search_issues(&jql_query, &issue_service.issue_repo.db, &jql_parser)
            .await
        {
            Ok(issues) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: issues,
                },
            ),
            Err(err_msg) => Response::bad_request(format!("JQL execution failed: {}", err_msg)),
        }
    }

    /// PATCH /api/v1/issues/:id/assignee - Assign or unassign an issue
    #[patch("/:id/assignee")]
    #[cap(IssueEdit)]
    pub async fn assign_issue(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        activity_log_repo: Arc<ActivityLogRepository>,
        webhook_service: Arc<WebhookService>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let payload = match ctx.json::<AssignIssuePayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid assignee payload"),
        };

        let actor_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        match issue_service
            .assign_issue(issue_id, payload.assignee_id)
            .await
        {
            Ok(updated_issue) => {
                let assign_detail = match updated_issue.assignee_id {
                    Some(aid) => format!("Assigned to user {}", aid),
                    None => "Assignee removed".to_string(),
                };
                let _ = activity_log_repo
                    .record(
                        updated_issue.project_id,
                        actor_id,
                        "assigned",
                        Some(updated_issue.id),
                        Some(&updated_issue.key),
                        Some(&updated_issue.summary),
                        Some(&assign_detail),
                        updated_issue.assignee_id,
                    )
                    .await;
                let _ = webhook_service
                    .fire(
                        updated_issue.project_id,
                        "issue.assigned",
                        &serde_json::json!({
                            "event": "issue.assigned",
                            "issue_id": updated_issue.id,
                            "key": updated_issue.key,
                            "summary": updated_issue.summary,
                            "assignee_id": updated_issue.assignee_id,
                            "actor_id": actor_id,
                            "triggered_at": chrono::Utc::now().to_rfc3339(),
                        }),
                    )
                    .await;
                Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: updated_issue,
                    },
                )
            }
            Err(e) => Response::bad_request(format!("Failed to update assignee: {}", e)),
        }
    }

    /// POST /api/v1/issues/:id/time - Log time against an issue
    #[post("/:id/time")]
    #[cap(IssueEdit)]
    pub async fn log_time(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        activity_log_repo: Arc<ActivityLogRepository>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let payload = match ctx.json::<LogTimePayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid log-time payload"),
        };

        let actor_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        match issue_service.log_time(issue_id, payload.minutes).await {
            Ok(Some(issue)) => {
                let _ = activity_log_repo
                    .record(
                        issue.project_id,
                        actor_id,
                        "time.logged",
                        Some(issue.id),
                        Some(&issue.key),
                        Some(&issue.summary),
                        Some(&format!("{}m logged", payload.minutes)),
                        None,
                    )
                    .await;
                Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: issue,
                    },
                )
            }
            Ok(None) => Response::not_found("Issue not found"),
            Err(e) => Response::bad_request(format!("Failed to log time: {}", e)),
        }
    }

    /// DELETE /api/v1/issues/:id - Delete an issue
    #[delete("/:id")]
    #[cap(IssueEdit)]
    pub async fn delete_issue(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        activity_log_repo: Arc<ActivityLogRepository>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let actor_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        let issue_before = issue_service.get_issue_by_id(issue_id).await.ok().flatten();

        if let Some(iss) = &issue_before {
            let _ = activity_log_repo
                .record(
                    iss.project_id,
                    actor_id,
                    "deleted",
                    Some(iss.id),
                    Some(&iss.key),
                    Some(&iss.summary),
                    Some("Issue deleted"),
                    None,
                )
                .await;
        }

        match issue_service.delete_issue(issue_id).await {
            Ok(true) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: "Issue deleted successfully",
                },
            ),
            Ok(false) => Response::not_found("Issue not found"),
            Err(e) => Response::bad_request(format!("Failed to delete issue: {}", e)),
        }
    }

    /// PATCH /api/v1/issues/:id - Update issue details
    #[patch("/:id")]
    #[cap(IssueEdit)]
    pub async fn update_issue(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        activity_log_repo: Arc<ActivityLogRepository>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let payload = match ctx.json::<UpdateIssuePayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid issue payload"),
        };

        let actor_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        match issue_service
            .update_issue(
                issue_id,
                payload.summary.as_deref(),
                payload.description.as_deref(),
                payload.priority,
                payload.issue_type.as_deref(),
                payload.story_points,
                payload.time_estimate_minutes,
            )
            .await
        {
            Ok(Some(issue)) => {
                let changed = payload
                    .summary
                    .clone()
                    .map(|s| !s.is_empty())
                    .unwrap_or(false);
                let _ = activity_log_repo
                    .record(
                        issue.project_id,
                        actor_id,
                        "updated",
                        Some(issue.id),
                        Some(&issue.key),
                        Some(&issue.summary),
                        Some(if changed { "Fields updated" } else { "Issue updated" }),
                        None,
                    )
                    .await;
                Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: issue,
                    },
                )
            }
            Ok(None) => Response::not_found("Issue not found"),
            Err(e) => Response::bad_request(format!("Failed to update issue: {}", e)),
        }
    }
}