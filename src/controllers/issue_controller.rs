use crate::controllers::{get_project_context, get_project_key};
use crate::dtos::{AddCommentPayload, CreateIssuePayload, MoveIssuePayload, UpdateIssuePayload};
use crate::models::comment;
use crate::repositories::comment::CommentRepository;
use crate::security::caps::{IssueCreate, IssueEdit, ViewBoard};
use crate::services::JqlParser;
use crate::services::issue_service::IssueService;
use crate::services::project_service::ProjectService;
use gritshield::database::GritRepository;
use gritshield::http::response::HttpStatus;
use gritshield::{GritSanitizer, prelude::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

                Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: serde_json::json!({
                            "issue": issue,
                            "comments": comments,
                        }),
                    },
                )
            }
            Ok(None) => Response::not_found("Issue not found"),
            Err(e) => Response::internal_error(e.to_string()),
        }
    }

    /// POST /api/v1/issues?project_id=N - Create a new issue
    #[post("")]
    #[cap(IssueCreate)]
    pub async fn create_issue(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        project_service: Arc<ProjectService>,
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
            Ok(created) => Response::json(
                HttpStatus::Created,
                &ApiResponse {
                    success: true,
                    data: created,
                },
            ),
            Err(e) => Response::bad_request(format!("Failed to create issue: {}", e)),
        }
    }

    /// PATCH /api/v1/issues/:id/step - Move issue step (Kanban workflow transition)
    #[patch("/:id/step")]
    #[cap(IssueEdit)]
    pub async fn move_step(ctx: RequestContext, issue_service: Arc<IssueService>) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let payload = match ctx.json::<MoveIssuePayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid target step payload"),
        };

        match issue_service
            .move_issue_step(
                issue_id,
                payload.target_step_id,
                ctx.get_session_data("user_id")
                    .and_then(|id| id.parse().ok())
                    .unwrap_or(1),
                &ctx,
            )
            .await
        {
            Ok(updated) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: updated,
                },
            ),
            Err(e) => Response::bad_request(format!("Failed to move issue: {}", e)),
        }
    }

    /// POST /api/v1/issues/:id/comments - Add comment to issue
    #[post("/:id/comments")]
    #[cap(IssueEdit)]
    pub async fn add_comment(ctx: RequestContext, issue_service: Arc<IssueService>) -> Response {
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

        match issue_service
            .add_comment(issue_id, payload, author_id, &ctx)
            .await
        {
            Ok(comment) => Response::json(
                HttpStatus::Created,
                &ApiResponse {
                    success: true,
                    data: comment,
                },
            ),
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
    pub async fn assign_issue(ctx: RequestContext, issue_service: Arc<IssueService>) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let payload = match ctx.json::<AssignIssuePayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid assignee payload"),
        };

        match issue_service
            .assign_issue(issue_id, payload.assignee_id)
            .await
        {
            Ok(updated_issue) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: updated_issue,
                },
            ),
            Err(e) => Response::bad_request(format!("Failed to update assignee: {}", e)),
        }
    }

    /// DELETE /api/v1/issues/:id - Delete an issue
    #[delete("/:id")]
    #[cap(IssueEdit)]
    pub async fn delete_issue(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

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
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let payload = match ctx.json::<UpdateIssuePayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid issue payload"),
        };

        match issue_service
            .update_issue(
                issue_id,
                payload.summary.as_deref(),
                payload.description.as_deref(),
                payload.priority,
                payload.issue_type.as_deref(),
                payload.story_points,
            )
            .await
        {
            Ok(Some(issue)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: issue,
                },
            ),
            Ok(None) => Response::not_found("Issue not found"),
            Err(e) => Response::bad_request(format!("Failed to update issue: {}", e)),
        }
    }
}