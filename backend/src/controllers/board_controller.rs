use crate::controllers::get_project_context;
use crate::dtos::MoveIssuePayload;
use crate::events::IssueTransitioned;
use crate::jobs::GenerateSprintBurndownJob;
use crate::models::{sprint, workflow};
use crate::repositories::activity_log::ActivityLogRepository;
use crate::repositories::sprint::SprintRepository;
use crate::security::caps::{IssueEdit, ProjectAdmin, ViewBoard};
use crate::services::board_service::BoardService;
use crate::services::issue_service::IssueService;
use crate::services::webhook_service::WebhookService;
use chrono::{Duration as ChronoDuration, NaiveDateTime, Utc};
use gritshield::database::GritRepository;
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
    pub async fn move_issue(
        ctx: RequestContext,
        board_service: Arc<BoardService>,
        issue_service: Arc<IssueService>,
        activity_log_repo: Arc<ActivityLogRepository>,
        webhook_service: Arc<WebhookService>,
    ) -> Response {
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

        let actor_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        let issue_before = issue_service.get_issue_by_id(issue_id).await.ok().flatten();

        match board_service
            .move_issue(issue_id, target_step_id, position)
            .await {
            Ok(updated_issue) => {
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
                                iss.step_id, target_step_id
                            )),
                            None,
                        )
                        .await;

                    let _ = webhook_service
                        .fire(
                            iss.project_id,
                            "issue.moved",
                            &serde_json::json!({
                                "event": "issue.moved",
                                "issue_id": iss.id,
                                "key": iss.key,
                                "summary": iss.summary,
                                "from_step_id": iss.step_id,
                                "to_step_id": target_step_id,
                                "actor_id": actor_id,
                                "triggered_at": chrono::Utc::now().to_rfc3339(),
                            }),
                        )
                        .await;
                }
                Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: updated_issue,
                    },
                )
            }
            Err(e) => Response::bad_request(format!("Move failed: {}", e)),
        }
    }

    /// GET /api/v1/board/sprints/:sprint_id/burndown?project_id=N
    ///
    /// Returns spring burndown data: totals, per-column breakdown and an ideal
    /// decay line computed from the sprint start/end dates.
    #[get("/sprints/:sprint_id/burndown")]
    #[cap(ViewBoard)]
    pub async fn get_burndown(
        ctx: RequestContext,
        board_service: Arc<BoardService>,
        sprint_repo: Arc<SprintRepository>,
    ) -> Response {
        let sprint_id: i32 = match ctx.params.get("sprint_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid or missing sprint ID"),
        };

        let project_id = get_project_context(&ctx);

        let sprint = match sprint_repo
            .query()
            .where_eq(sprint::Column::Id, sprint_id)
            .where_eq(sprint::Column::ProjectId, project_id)
            .fetch_one()
            .await
        {
            Ok(spr) => spr,
            Err(e) => return Response::bad_request(format!("Failed to load sprint: {}", e)),
        };

        let columns = match board_service
            .get_sprint_board_data(project_id, sprint_id)
            .await
        {
            Ok(c) => c,
            Err(e) => return Response::bad_request(format!("Failed to load board: {}", e)),
        };

        let total_points: i32 = columns
            .iter()
            .flat_map(|(_, issues)| issues.iter())
            .map(|i| i.story_points.unwrap_or(0))
            .sum();

        let done_step_id = columns
            .iter()
            .find(|(step, _)| step.is_completed)
            .or_else(|| columns.last())
            .map(|(step, _)| step.id);

        let done_points: i32 = match done_step_id {
            Some(did) => columns
                .iter()
                .flat_map(|(_, issues)| issues.iter())
                .filter(|i| i.step_id == did)
                .map(|i| i.story_points.unwrap_or(0))
                .sum(),
            None => 0,
        };

        let remaining_points = total_points - done_points;

        let column_breakdown: Vec<_> = columns
            .iter()
            .map(|(step, issues)| {
                serde_json::json!({
                    "id": step.id,
                    "name": step.name,
                    "is_completed": step.is_completed,
                    "count": issues.len(),
                    "points": issues.iter().map(|i| i.story_points.unwrap_or(0)).sum::<i32>(),
                })
            })
            .collect();

        // Ideal burndown line: linear decay from total to 0 over the sprint.
        let today = Utc::now().naive_utc().date();
        let start = sprint
            .start_date
            .map(|d| d.date())
            .unwrap_or_else(|| (today - ChronoDuration::days(7)));
        let deadline = sprint
            .end_date
            .map(|d| d.date())
            .unwrap_or_else(|| start + ChronoDuration::days(14));

        let total_days = (deadline - start).num_days().max(1) as f64;
        let mut ideal = Vec::new();
        let mut actual = Vec::new();
        let mut day = start;
        let mut idx = 0;
        while day <= today.max(start) {
            let frac = idx as f64 / total_days;
            let ideal_rem = (total_points as f64 * (1.0 - frac)).round();
            ideal.push(serde_json::json!({
                "date": day.format("%Y-%m-%d").to_string(),
                "remaining": ideal_rem as i32,
            }));
            let remaining = if day >= today { remaining_points } else { total_points };
            actual.push(serde_json::json!({
                "date": day.format("%Y-%m-%d").to_string(),
                "remaining": remaining,
            }));
            day += ChronoDuration::days(1);
            idx += 1;
        }

        let data = serde_json::json!({
            "sprint": {
                "id": sprint.id,
                "name": sprint.name,
                "status": sprint.status,
                "start_date": sprint.start_date.map(|d| d.format("%Y-%m-%d").to_string()),
                "end_date": sprint.end_date.map(|d| d.format("%Y-%m-%d").to_string()),
            },
            "total_points": total_points,
            "done_points": done_points,
            "remaining_points": remaining_points,
            "percent_done": if total_points > 0 { (done_points as f64 / total_points as f64 * 100.0).round() as i32 } else { 0 },
            "columns": column_breakdown,
            "ideal": ideal,
            "actual": actual,
        });

        Response::json(
            HttpStatus::Ok,
            &ApiResponse {
                success: true,
                data,
            },
        )
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