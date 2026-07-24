use gritshield::prelude::*;
use std::sync::Arc;

use crate::security::caps::ViewBoard;
use crate::services::board_service::BoardService;
use crate::web::render::MaudRender;
use crate::web::views::backlog_view::backlog_view;

pub struct BacklogController;

#[controller("/jira")]
impl BacklogController {
    #[get("/backlog")]
    #[cap(ViewBoard)]
    pub async fn view_backlog(ctx: RequestContext, board_service: Arc<BoardService>) -> Response {
        let backlog_issues = board_service.get_backlog_issues().await.unwrap_or_default();
        let sprints = board_service
            .get_active_sprints(1)
            .await
            .unwrap_or_default();

        let markup = backlog_view(&backlog_issues, &sprints);
        markup.render(ctx, false, "Backlog - GritJira")
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

        // Fix: Use .as_str() on UntrustedString to compare with &str
        let sprint_id: i32 = ctx
            .form
            .fields
            .get("sprint_id")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap();

        match board_service.assign_sprint(issue_id, sprint_id).await {
            Ok(_) => Response::ok("Sprint updated"),
            Err(e) => Response::bad_request(format!("Assignment failed: {}", e)),
        }
    }
}
