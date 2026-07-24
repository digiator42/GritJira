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
        Response::ok(create_issue_modal().into_string())
    }

    /// Handles issue creation with sanitization and re-renders the board view
    #[post("/create")]
    #[cap(IssueCreate)]
    pub async fn create_issue(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        board_service: Arc<BoardService>,
    ) -> ShieldResult<Response> {
        let payload = ctx.json::<CreateIssuePayload>().await?;

        let reporter_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(0);

        // 1. Persist issue & fire event
        issue_service
            .create_issue(payload, reporter_id, &ctx)
            .await
            .ok();

        // 2. Fetch fresh board data and re-render board
        let board_data = board_service
            .get_sprint_board_data(1, 1)
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
