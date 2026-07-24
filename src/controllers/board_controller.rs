use crate::events::IssueTransitioned;
use crate::jobs::{ExportProjectArchiveJob, GenerateSprintBurndownJob};
use crate::security::caps::{IssueEdit, ProjectAdmin, ViewBoard};
use crate::services::JqlParser;
use crate::services::board_service::BoardService;
use crate::web::partials::issue_card::issue_card;
use crate::web::render::MaudRender;
use crate::web::views::board_view::kanban_board_view;
use gritshield::GritJobExt;
use gritshield::prelude::*;
use std::sync::Arc;
use std::time::Duration;

pub struct BoardController;

#[controller("/jira")]
impl BoardController {
    #[get("/board")]
    #[cap(ViewBoard)] // Enforces that user has ViewBoard capability
    pub async fn view_board(ctx: RequestContext, board_service: Arc<BoardService>) -> Response {
        let issue_id = 1;
        let from_step_id = 0; // To Do
        let to_step_id = 1; // In Progress

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
    #[cap(IssueEdit)] // Enforces Admin, Manager, or Developer role
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
            Ok(updated_issue) => {
                let card_markup = issue_card(&updated_issue);
                Response::ok(card_markup.into_string())
            }
            Err(e) => Response::bad_request(format!("Move failed: {}", e)),
        }
    }

    #[get("/trigger-burndown")]
    #[cap(ProjectAdmin)] // Admin-only endpoint
    pub async fn trigger_burndown(ctx: RequestContext) -> Response {
        let job = GenerateSprintBurndownJob {
            sprint_id: 1,
            project_id: 10,
        };

        let _ = job.enqueue_in(&ctx.job_queue, Duration::from_secs(5)).await;

        Response::ok("Sprint burndown recalculation queued!")
    }

    #[get("/search")]
    #[cap(ViewBoard)]
    pub async fn search_issues(ctx: RequestContext, jql_parser: Arc<JqlParser>) -> Response {
        let sql_filter = jql_parser.parse_query("project = GRIT AND assignee = alex_dev");
        Response::ok(format!("Parsed SQL filter: {}", sql_filter))
    }

    #[post("/move")]
    #[cap(IssueEdit)]
    pub async fn move_card(ctx: RequestContext, board_service: Arc<BoardService>) -> Response {
        match board_service.transition_issue(0, 1).await {
            Ok(_) => Response::ok("Card moved successfully"),
            Err(err) => Response::bad_request(err),
        }
    }
}
