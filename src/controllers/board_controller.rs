use crate::events::IssueTransitioned;
use crate::jobs::{ExportProjectArchiveJob, GenerateSprintBurndownJob};
use crate::services::board_service::BoardService;
use crate::web::partials::issue_card::issue_card;
use crate::web::render::MaudRender;
use crate::web::views::board_view::kanban_board_view;
use gritshield::GritJobExt;
use gritshield::prelude::*;
use std::time::Duration;

pub struct BoardController;

#[controller("/admin/jira")]
impl BoardController {
    #[get("/board")]
    pub async fn view_board(ctx: RequestContext) -> Response {
        let issue_id = 1;
        let from_step_id = 0; // To Do
        let to_step_id = 1; // In Progress

        let db = match ctx.db.as_deref() {
            Some(d) => d.clone(),
            None => return Response::bad_request("Database connection missing"),
        };

        let board_service = BoardService::new(db);

        // Fetch board data using GritShield QueryBuilder pipeline
        let board_data = match board_service.get_sprint_board_data(1, 1).await {
            Ok(data) => data,
            Err(e) => return Response::bad_request(format!("Failed to load board: {}", e)),
        };

        ctx.event_bus.publish(IssueTransitioned {
            issue_id,
            key: "GRIT-1".to_string(),
            from_step_id,
            to_step_id,
            actor_id: 42,
        });

        let markup = kanban_board_view(&board_data);
        markup.render(ctx.clone(), false, "Sprint Board")
    }

    #[post("/issues/:id/move")]
    pub async fn move_issue(ctx: RequestContext) -> Response {
        let db = match ctx.db.as_deref() {
            Some(d) => d.clone(),
            None => return Response::bad_request("Database connection missing"),
        };

        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        let target_step_id: i32 = match ctx.form.fields.get("step_id").and_then(|v| v.parse().ok())
        {
            Some(step) => step,
            None => return Response::bad_request("Missing target step_id"),
        };

        let board_service = BoardService::new(db);

        match board_service.move_issue(issue_id, target_step_id).await {
            Ok(updated_issue) => {
                let card_markup = issue_card(&updated_issue);
                Response::ok(card_markup.into_string())
            }
            Err(e) => Response::bad_request(format!("Move failed: {}", e)),
        }
    }

    // Inside BoardController / ProjectController
    pub async fn trigger_burndown(ctx: RequestContext) -> Response {
        let job = GenerateSprintBurndownJob {
            sprint_id: 1,
            project_id: 10,
        };

        // Enqueue job with optional delay
        let _ = job.enqueue_in(&ctx.job_queue, Duration::from_secs(5)).await;

        Response::ok("Sprint burndown recalculation queued!")
    }
}
