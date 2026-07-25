use crate::dtos::{AddCommentPayload, CreateIssuePayload, MoveIssuePayload};
use crate::security::caps::{IssueCreate, IssueEdit, ViewBoard};
use crate::services::JqlParser;
use crate::services::issue_service::IssueService;
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
    /// GET /api/v1/issues/:id - Fetch issue details
    #[get("/:id")]
    #[cap(ViewBoard)]
    pub async fn get_issue(ctx: RequestContext, issue_service: Arc<IssueService>) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        match issue_service.get_issue_by_id(issue_id).await {
            Ok(Some(issue)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: issue,
                },
            ),
            Ok(None) => Response::not_found("Issue not found"),
            Err(e) => Response::internal_error(e.to_string()),
        }
    }

    /// POST /api/v1/issues/projects/:project_id - Create a new issue
    #[post("/projects/:project_id")]
    #[cap(IssueCreate)]
    pub async fn create_issue(ctx: RequestContext, issue_service: Arc<IssueService>) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let payload = match ctx.json::<CreateIssuePayload>().await {
            Ok(p) => p,
            Err(e) => return Response::bad_request(format!("Invalid request body: {:?}", e)),
        };

        let reporter_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        match issue_service
            .create_issue(payload, project_id, reporter_id, &ctx)
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
            .move_issue_step(issue_id, payload.target_step_id, 3, &ctx)
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
    // src/controllers/issue_controller.rs
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
            .cloned()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "project_id = 1".to_string());

        let is_htmx = ctx.req.has_header("hx-request");

        match issue_service
            .search_issues(&jql_query, &issue_service.issue_repo.db, &jql_parser)
            .await
        {
            Ok(issues) => {
                if is_htmx {
                    // For HTMX requests, return just the issues array
                    // The client-side-templates extension will render it
                    Response::ok(serde_json::to_string(&issues).unwrap_or_default())
                } else {
                    Response::json(
                        HttpStatus::Ok,
                        &ApiResponse {
                            success: true,
                            data: issues,
                        },
                    )
                }
            }
            Err(err_msg) => {
                if is_htmx {
                    Response::bad_request(format!("JQL execution failed: {}", err_msg))
                } else {
                    Response::bad_request(format!("JQL execution failed: {}", err_msg))
                }
            }
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
}
