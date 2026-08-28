use gritshield::GritSanitizer;
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::{IssueModel, SprintModel};
use crate::repositories::issue::IssueRepository;
use crate::security::caps::ViewBoard;
use crate::services::issue_service::IssueService;

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

#[derive(Deserialize, GritSanitizer)]
pub struct AssignSprintPayload {
    /// Pass `Some(id)` to assign, or `null` to move the issue back to the backlog
    pub sprint_id: Option<i32>,
}

pub struct BacklogController;

#[controller("/api/v1/backlog")]
impl BacklogController {
    /// GET /api/v1/backlog/projects/:project_id
    ///
    /// Returns this project's unassigned issues (backlog) plus all of the
    /// project's sprints ordered by end date (newest first).
    #[get("/projects/:project_id")]
    #[cap(ViewBoard)]
    pub async fn get_backlog(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid or missing project ID"),
        };

        let backlog_issues = issue_service
            .get_backlog_issues(project_id)
            .await
            .unwrap_or_default();

        let sprints = issue_service
            .get_project_sprints(project_id)
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

    /// POST /api/v1/backlog/issues/:id/assign-sprint
    ///
    /// Body: `{ "sprint_id": 2 }` to assign, `{ "sprint_id": null }` to unassign.
    /// Form field `sprint_id` is also accepted.
    #[post("/issues/:id/assign-sprint")]
    #[cap(ViewBoard)]
    pub async fn assign_issue_sprint(
        ctx: RequestContext,
        issue_repo: Arc<IssueRepository>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let sprint_id = if let Ok(payload) = ctx.json::<AssignSprintPayload>().await {
            payload.sprint_id
        } else if let Some(id) = ctx
            .form
            .fields
            .get("sprint_id")
            .and_then(|v| v.first())
            .and_then(|s| s.parse::<i32>().ok())
        {
            Some(id)
        } else {
            return Response::bad_request("Missing or invalid sprint_id");
        };

        match issue_repo.update_sprint(issue_id, sprint_id).await {
            Ok(updated_issue) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: updated_issue,
                },
            ),
            Err(e) => Response::bad_request(format!("Assignment failed: {}", e)),
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