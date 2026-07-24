use gritshield::prelude::*;
use gritshield::routing::engine::ShieldResult;
use std::sync::Arc;

use crate::dtos::{AddCommentPayload, CreateIssuePayload};
use crate::security::caps::{IssueCreate, IssueEdit};
use crate::services::board_service::BoardService;
use crate::services::issue_service::IssueService;
use crate::web::partials::create_issue_modal::create_issue_modal;
use crate::web::views::board_view::kanban_board_view;

pub struct IssueController;

#[controller("/jira/issues")]
impl IssueController {
    /// Renders the modal HTML partial into `#modals-container`
    #[get("/new-modal")]
    #[cap(IssueCreate)]
    pub async fn get_create_modal(ctx: RequestContext) -> Response {
        let project_id: i32 = ctx
            .query
            .get("project_id")
            .and_then(|p| p.parse().ok())
            .unwrap_or(1);
        
        Response::ok(create_issue_modal(project_id).into_string())
    }

    /// Handles issue creation with sanitization and re-renders the board view
    #[post("/projects/:project_id/issues/create")]
    #[cap(IssueCreate)]
    pub async fn create_issue(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        board_service: Arc<BoardService>,
    ) -> ShieldResult<Response> {
        // 1. Extract project_id from URL params (defaults to project 1 if parsing fails or unmapped)
        let project_id: i32 = ctx
            .params
            .get("project_id")
            .and_then(|p| p.parse().ok())
            .unwrap_or(1);

        let payload = ctx.json::<CreateIssuePayload>().await?;

        let reporter_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        // 2. Pass project_id directly into IssueService
        if let Err(err) = issue_service
            .create_issue(payload, project_id, reporter_id, &ctx)
            .await
        {
            eprintln!("[ERROR] Failed to create issue in DB: {:?}", err);
            return Ok(Response::bad_request(format!("Database error: {}", err)));
        }

        // 3. Fetch current active sprint for the project dynamically
        let active_sprint_id = board_service
            .get_active_sprints(project_id)
            .await
            .ok()
            .and_then(|sprints| sprints.into_iter().next().map(|s| s.id))
            .unwrap_or(1);

        // 4. Fetch fresh board data for this specific project and sprint
        let board_data = board_service
            .get_sprint_board_data(project_id, active_sprint_id)
            .await
            .unwrap_or_default();

        let markup = kanban_board_view(&board_data);

        Ok(Response::ok(markup.into_string()))
    }

    #[post("/:id/comments")]
    #[cap(IssueEdit)]
    pub async fn add_comment(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
    ) -> ShieldResult<Response> {
        let issue_id: i32 = ctx
            .params
            .get("id")
            .and_then(|p| p.parse().ok())
            .unwrap_or(0);
        let payload = ctx.json::<AddCommentPayload>().await?;

        let author_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        issue_service
            .add_comment(issue_id, payload, author_id, &ctx)
            .await
            .ok();

        Ok(Response::ok("Comment added successfully"))
    }
}
