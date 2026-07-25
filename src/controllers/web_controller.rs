// src/controllers/jira_web_controller.rs
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use std::sync::Arc;

use crate::services::JqlParser;
use crate::services::{board_service::BoardService, issue_service::IssueService};
use crate::web::partials::create_issue_modal::create_issue_modal;
use crate::web::partials::create_sprint_modal::create_sprint_modal;
use crate::web::partials::issue_detail_modal::{self, issue_detail_modal};
use crate::web::render::MaudRender;
use crate::web::views::auth_view::login_page_view;
use crate::web::views::projects_view::projects_view;
use crate::web::views::search_view::search_page;
use crate::web::views::{backlog_view::backlog_view, board_view::kanban_board_view};

pub struct WebController;

#[controller("/jira")]
impl WebController {
    /// GET /jira/board
    #[get("/board")]
    pub async fn board_page(ctx: RequestContext, board_service: Arc<BoardService>) -> Response {
        let sprint_id = 1; // Default or active sprint ID
        let columns = match board_service.get_kanban_columns(sprint_id).await {
            Ok(cols) => cols,
            Err(_) => vec![],
        };

        kanban_board_view(&columns).render(ctx, "Sprint Board")
    }

    #[get("/login")]
    pub async fn login_page(ctx: RequestContext) -> Response {
        login_page_view().render(ctx, "Login Page")
    }

    /// GET /jira/backlog
    #[get("/backlog")]
    pub async fn backlog_page(ctx: RequestContext, issue_service: Arc<IssueService>) -> Response {
        let project_id = 1;
        let backlog = issue_service
            .get_backlog_issues(project_id)
            .await
            .unwrap_or_default();
        let sprints = issue_service
            .get_project_sprints(project_id)
            .await
            .unwrap_or_default();

        backlog_view(&backlog, &sprints).render(ctx, "Backlog")
    }

    /// GET /jira/issues/new-modal (Target: #modals-container)
    #[get("/issues/new-modal")]
    pub async fn new_issue_modal(ctx: RequestContext) -> Response {
        // Returns only the modal partial directly to #modals-container
        let modal_markup = create_issue_modal(1, Some("GRIT"));
        Response::ok(modal_markup.into_string())
    }

    #[get("/projects")]
    pub async fn projects_page(
        ctx: RequestContext,
        // project_service: Arc<ProjectService>,
    ) -> Response {
        // let projects = project_service.list_projects().await.unwrap_or_default();
        projects_view(&[]).render(ctx, "Projects")
    }

    #[get("/sprints/new-modal")]
    pub async fn new_sprint_modal(ctx: RequestContext) -> Response {
        let project_id = ctx
            .query
            .get("project_id")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);
        Response::ok(create_sprint_modal(project_id).into_string())
    }

    #[get("/issues/:id/detail-modal")]
    pub async fn issue_detail_modal(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
    ) -> Response {
        let issue_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid issue ID"),
        };

        match issue_service.get_issue_by_id(issue_id).await {
            Ok(Some(issue)) => {
                let markup = issue_detail_modal(&issue);
                Response::ok(markup.into_string())
            }
            Ok(None) => Response::not_found("Issue not found"),
            Err(e) => Response::internal_error(e.to_string()),
        }
    }

    #[get("/search")]
    pub async fn search_page(ctx: RequestContext) -> Response {
        search_page().render(ctx, "Search Issues")
    }

    // Add a new endpoint that returns HTML
    #[get("/search/results")]
    pub async fn search_results(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        jql_parser: Arc<JqlParser>,
    ) -> Response {
        let jql = ctx.query.get("jql").map(|v| v.to_string()).unwrap_or("".to_string());

        let issues = issue_service
            .search_issues(&jql, &issue_service.issue_repo.db, &jql_parser)
            .await
            .unwrap_or_default();

        // Render HTML directly on the server
        let markup = html! {
            @if issues.is_empty() {
                p class="text-gray-500 italic" { "No issues found." }
            } @else {
                @for issue in issues {
                    div class="bg-gray-900 border border-gray-800 rounded-lg p-3 flex justify-between items-center hover:border-gray-700 transition" {
                        div {
                            span class="text-blue-400 font-bold" { (issue.key) }
                            span class="text-gray-300 ml-2" { (issue.summary) }
                        }
                        div class="flex items-center gap-2" {
                            span class="text-xxs bg-gray-800 text-gray-400 px-2 py-0.5 rounded" { (issue.priority) }
                            span class="text-xxs text-gray-500" { (issue.issue_type) }
                        }
                    }
                }
            }
        };

        Response::ok(markup.into_string())
    }
}
