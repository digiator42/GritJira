use crate::controllers::get_project_context;
use crate::dtos::MoveIssuePayload;
use crate::events::IssueTransitioned;
use crate::jobs::GenerateSprintBurndownJob;
use crate::security::caps::{IssueEdit, ProjectAdmin, ViewBoard};
use crate::services::board_service::BoardService;
use gritshield::GritJobExt;
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

pub struct BoardController;

#[controller("/api/v1/board")]
impl BoardController {
    /// GET /api/v1/board/sprints/:sprint_id?project_id=N
    ///
    /// Returns the board as structured Kanban columns:
    /// `{ sprint_id, project_id, columns: [{ step, issues }] }`
    #[get("/sprints/:sprint_id")]
    #[cap(ViewBoard)]
    pub async fn get_board(ctx: RequestContext, board_service: Arc<BoardService>) -> Response {
        let sprint_id: i32 = match ctx.params.get("sprint_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid or missing sprint ID"),
        };

        // Resolve project from query param; fall back to session/default
        let project_id = get_project_context(&ctx);

        match board_service
            .get_sprint_board_data(project_id, sprint_id)
            .await
        {
            Ok(board_data) => {
                let columns: Vec<_> = board_data
                    .into_iter()
                    .map(|(step, issues)| serde_json::json!({ "step": step, "issues": issues }))
                    .collect();

                Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: serde_json::json!({
                            "sprint_id": sprint_id,
                            "project_id": project_id,
                            "columns": columns,
                        }),
                    },
                )
            }
            Err(e) => Response::bad_request(format!("Failed to load board: {}", e)),
        }
    }

    /// POST /api/v1/board/issues/:id/move
    /// Body: `{ "target_step_id": N }` (form field `step_id` also accepted)
    #[post("/issues/:id/move")]
    #[cap(IssueEdit)]
    pub async fn move_issue(ctx: RequestContext, board_service: Arc<BoardService>) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let payload = ctx.json::<MoveIssuePayload>().await.ok();

        let target_step_id = if let Some(p) = &payload {
            p.target_step_id
        } else if let Some(step) = ctx
            .form
            .fields
            .get("step_id")
            .and_then(|v| v.first())
            .and_then(|s| s.parse::<i32>().ok())
        {
            step
        } else {
            return Response::bad_request("Missing target step_id");
        };

        let position = payload.as_ref().and_then(|p| p.position);

        match board_service
            .move_issue(issue_id, target_step_id, position)
            .await {
            Ok(updated_issue) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: updated_issue,
                },
            ),
            Err(e) => Response::bad_request(format!("Move failed: {}", e)),
        }
    }

    /// POST /api/v1/board/trigger-burndown?project_id=N&sprint_id=N
    #[post("/trigger-burndown")]
    #[cap(ProjectAdmin)]
    pub async fn trigger_burndown(ctx: RequestContext) -> Response {
        let sprint_id = ctx
            .query
            .get("sprint_id")
            .and_then(|v| v.first().and_then(|s| s.parse().ok()))
            .unwrap_or(1);
        let project_id = get_project_context(&ctx);

        let job = GenerateSprintBurndownJob {
            sprint_id,
            project_id,
        };

        let _ = job.enqueue_in(Duration::from_secs(5)).await;

        Response::json(
            HttpStatus::Ok,
            &ApiResponse {
                success: true,
                data: "Sprint burndown recalculation queued",
            },
        )
    }
}