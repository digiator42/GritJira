use crate::dtos::{AddCommentPayload, CreateIssuePayload, MoveIssuePayload, UpdateIssuePayload};
use crate::security::caps::{IssueCreate, IssueEdit, ViewBoard};
use crate::services::JqlParser;
use crate::services::issue_service::IssueService;
use crate::web::views::helpers::get_project_context;
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

    #[post("")]
    #[cap(IssueCreate)]
    pub async fn create_issue(ctx: RequestContext, issue_service: Arc<IssueService>) -> Response {
        // Get project_id from session context
        let project_id = get_project_context(&ctx);

        let payload = match ctx.json::<CreateIssuePayload>().await {
            Ok(p) => p,
            Err(e) => {
                let error_msg = format!("Invalid request body: {:?}", e);
                if ctx.req.has_header("hx-request") {
                    let error_html = html! {
                        div class="bg-red-950/30 border border-red-800/60 rounded-lg p-4 text-center" {
                            div class="text-red-400 text-2xl mb-2" { "❌" }
                            p class="text-red-300 text-sm" { "Invalid form data" }
                            p class="text-gray-400 text-xxs mt-1" { (error_msg) }
                        }
                    };
                    return Response::bad_request(error_html.into_string());
                }
                return Response::bad_request(error_msg);
            }
        };

        let reporter_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        let is_htmx = ctx.req.has_header("hx-request");

        // Get the first workflow step for this project (to use as default)
        let default_step_id = match issue_service.get_first_workflow_step(project_id).await {
            Ok(Some(step)) => step.id,
            Ok(None) => {
                let error_msg = format!("Project {} has no workflow steps configured", project_id);
                if is_htmx {
                    let error_html = html! {
                        div class="bg-red-950/30 border border-red-800/60 rounded-lg p-4 text-center" {
                            div class="text-red-400 text-2xl mb-2" { "❌" }
                            p class="text-red-300 text-sm" { "Cannot create issue" }
                            p class="text-gray-400 text-xxs mt-1" { "Project has no workflow steps" }
                            p class="text-gray-500 text-xxs mt-1" { "Please configure workflow steps first" }
                            button
                                hx-get="/jira/projects"
                                hx-target="#main-content"
                                hx-swap="innerHTML"
                                class="mt-3 bg-gray-700 hover:bg-gray-600 text-white px-4 py-2 rounded-lg text-xs transition" {
                                "Back to Projects"
                            }
                        }
                    };
                    return Response::bad_request(error_html.into_string());
                }
                return Response::bad_request(error_msg);
            }
            Err(e) => {
                if is_htmx {
                    let error_html = html! {
                        div class="bg-red-950/30 border border-red-800/60 rounded-lg p-4 text-center" {
                            div class="text-red-400 text-2xl mb-2" { "❌" }
                            p class="text-red-300 text-sm" { "Failed to create issue" }
                            p class="text-gray-400 text-xxs mt-1" { (e.to_string()) }
                        }
                    };
                    return Response::bad_request(error_html.into_string());
                }
                return Response::bad_request(format!("Failed to get workflow step: {}", e));
            }
        };

        // Create the issue with the default step_id
        match issue_service
            .create_issue_with_step(payload, project_id, reporter_id, default_step_id, &ctx)
            .await
        {
            Ok(created) => {
                if is_htmx {
                    let success_html = html! {
                        div class="bg-green-950/30 border border-green-800/60 rounded-lg p-6 text-center" {
                            div class="text-green-400 text-4xl mb-3" { "✅" }
                            h3 class="text-lg font-bold text-white mb-1" { "Issue Created!" }
                            p class="text-gray-300 text-sm" {
                                (format!("{} - {}", created.key, created.summary))
                            }
                            p class="text-gray-500 text-xxs mt-2" {
                                "The issue has been added to the backlog."
                            }
                            div class="mt-4 flex items-center justify-center gap-3" {
                                button
                                    hx-get={"/jira/board?project_id=" (project_id)}
                                    hx-target="#main-content"
                                    hx-swap="innerHTML"
                                    hx-push-url="true"
                                    class="bg-blue-600 hover:bg-blue-500 text-white px-4 py-2 rounded-lg text-xs transition" {
                                    "View Board"
                                }
                                button
                                    hx-get={"/jira/backlog?project_id=" (project_id)}
                                    hx-target="#main-content"
                                    hx-swap="innerHTML"
                                    hx-push-url="true"
                                    class="bg-gray-700 hover:bg-gray-600 text-white px-4 py-2 rounded-lg text-xs transition" {
                                    "View Backlog"
                                }
                            }
                        }
                    };
                    Response::ok(success_html.into_string())
                } else {
                    Response::json(
                        HttpStatus::Created,
                        &ApiResponse {
                            success: true,
                            data: created,
                        },
                    )
                }
            }
            Err(e) => {
                if is_htmx {
                    let error_html = html! {
                        div class="bg-red-950/30 border border-red-800/60 rounded-lg p-4 text-center" {
                            div class="text-red-400 text-2xl mb-2" { "❌" }
                            p class="text-red-300 text-sm" { "Failed to create issue" }
                            p class="text-gray-400 text-xxs mt-1" { (e.to_string()) }
                        }
                    };
                    Response::bad_request(error_html.into_string())
                } else {
                    Response::bad_request(format!("Failed to create issue: {}", e))
                }
            }
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
            .and_then(|v| v.first())
            .map(|s| s.to_string())
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

        let is_htmx = ctx.req.has_header("hx-request");

        match issue_service.delete_issue(issue_id).await {
            Ok(true) => {
                if is_htmx {
                    Response::ok("")
                } else {
                    Response::json(
                        HttpStatus::Ok,
                        &ApiResponse {
                            success: true,
                            data: "Issue deleted successfully",
                        },
                    )
                }
            }
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

        let is_htmx = ctx.req.has_header("hx-request");

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
            Ok(Some(issue)) => {
                if is_htmx {
                    let issue_html = html! {
                        div class="bg-green-950/30 border border-green-800/60 rounded-lg p-3 flex items-center gap-2" {
                            span class="text-green-400" { "✅" }
                            span class="text-green-300 text-xs" { "Issue updated successfully" }
                        }
                    };
                    Response::ok(issue_html.into_string())
                } else {
                    Response::json(
                        HttpStatus::Ok,
                        &ApiResponse {
                            success: true,
                            data: issue,
                        },
                    )
                }
            }
            Ok(None) => Response::not_found("Issue not found"),
            Err(e) => Response::bad_request(format!("Failed to update issue: {}", e)),
        }
    }
}
