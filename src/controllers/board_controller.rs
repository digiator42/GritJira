use gritshield::GritJobExt;
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;

use crate::events::IssueTransitioned;
use crate::jobs::GenerateSprintBurndownJob;
use crate::security::caps::{IssueEdit, ProjectAdmin, ViewBoard};
use crate::services::JqlParser;
use crate::services::board_service::BoardService;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

pub struct BoardController;

#[controller("/api/v1/board")]
impl BoardController {
    #[get("/sprints/:sprint_id")]
    #[cap(ViewBoard)]
    pub async fn get_board(ctx: RequestContext, board_service: Arc<BoardService>) -> Response {
        let sprint_id: i32 = match ctx.params.get("sprint_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid or missing sprint ID"),
        };

        let project_id = 1;

        match board_service
            .get_sprint_board_data(project_id, sprint_id)
            .await
        {
            Ok(board_data) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: board_data,
                },
            ),
            Err(e) => Response::bad_request(format!("Failed to load board: {}", e)),
        }
    }

    #[post("/issues/:id/move")]
    #[cap(IssueEdit)]
    pub async fn move_issue(ctx: RequestContext, board_service: Arc<BoardService>) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let target_step_id: i32 = match ctx.form.fields.get("step_id").and_then(|v| v.parse().ok())
        {
            Some(step) => step,
            None => return Response::bad_request("Missing target step_id"),
        };

        match board_service.move_issue(issue_id, target_step_id).await {
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

    #[get("/trigger-burndown")]
    #[cap(ProjectAdmin)]
    pub async fn trigger_burndown(ctx: RequestContext) -> Response {
        let job = GenerateSprintBurndownJob {
            sprint_id: 1,
            project_id: 10,
        };

        let _ = job.enqueue_in(&ctx.job_queue, Duration::from_secs(5)).await;

        Response::json(
            HttpStatus::Ok,
            &ApiResponse {
                success: true,
                data: "Sprint burndown recalculation queued",
            },
        )
    }

    #[get("/search")]
    #[cap(ViewBoard)]
    pub async fn search_issues(ctx: RequestContext, jql_parser: Arc<JqlParser>) -> Response {
        let sql_filter = jql_parser.parse_query("project = GRIT AND assignee = alex_dev");
        Response::json(
            HttpStatus::Ok,
            &ApiResponse {
                success: true,
                data: sql_filter,
            },
        )
    }
}
