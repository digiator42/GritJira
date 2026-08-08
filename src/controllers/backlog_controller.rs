use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use serde::Serialize;
use std::sync::Arc;

use crate::models::{IssueModel, SprintModel};
use crate::security::caps::ViewBoard;
use crate::services::board_service::BoardService;

#[derive(Serialize)]
pub struct BacklogResponse {
    pub backlog_issues: Vec<IssueModel>,
    pub sprints: Vec<SprintModel>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

pub struct BacklogController;

#[controller("/api/v1/backlog")]
impl BacklogController {
    #[get("/projects/:project_id")]
    #[cap(ViewBoard)]
    pub async fn get_backlog(ctx: RequestContext, board_service: Arc<BoardService>) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid or missing project ID"),
        };

        let backlog_issues = board_service.get_backlog_issues().await.unwrap_or_default();
        let sprints = board_service
            .get_active_sprints(project_id)
            .await
            .unwrap_or_default();

        Response::json(
            HttpStatus::Ok,
            &ApiResponse {
                success: true,
                data: BacklogResponse {
                    backlog_issues,
                    sprints,
                },
            },
        )
    }

    #[post("/issues/:id/assign-sprint")]
    #[cap(ViewBoard)]
    pub async fn assign_issue_sprint(
        ctx: RequestContext,
        board_service: Arc<BoardService>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let sprint_id: i32 = match ctx
            .form
            .fields
            .get("sprint_id")
            .and_then(|v| v.first())
            .and_then(|s| s.parse().ok())
        {
            Some(id) => id,
            None => return Response::bad_request("Missing or invalid sprint_id"),
        };

        let is_htmx = ctx.req.has_header("hx-request");

        match board_service.assign_sprint(issue_id, sprint_id).await {
            Ok(updated_issue) => {
                if is_htmx {
                    // Return success message or updated issue row
                    let success_html = html! {
                        div class="bg-green-950/30 border border-green-800/60 rounded-lg p-3 flex items-center gap-2" {
                            span class="text-green-400" { "✅" }
                            span class="text-green-300 text-xs" { "Assigned to sprint!" }
                        }
                    };
                    Response::ok(success_html.into_string())
                } else {
                    Response::json(
                        HttpStatus::Ok,
                        &ApiResponse {
                            success: true,
                            data: updated_issue,
                        },
                    )
                }
            }
            Err(e) => {
                if is_htmx {
                    Response::bad_request(format!("Assignment failed: {}", e))
                } else {
                    Response::bad_request(format!("Assignment failed: {}", e))
                }
            }
        }
    }
    #[get("/api/version")]
    pub fn get_version() -> Response {
        Response::json_ok(&serde_json::json!({
            "version": "1.0.0",
            "name": "MyApp"
        }))
    }
}
