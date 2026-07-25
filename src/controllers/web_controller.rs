use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use sea_orm::DbErr;
use std::sync::Arc;

use crate::services::JqlParser;
use crate::services::project_service::ProjectService;
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
    pub async fn board_page(
        ctx: RequestContext,
        board_service: Arc<BoardService>,
        project_service: Arc<ProjectService>,
    ) -> Response {
        // Get project_id from query params, default to 1
        let project_id: i32 = ctx
            .query
            .get("project_id")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);

        // Get sprint_id from query params, or find active sprint for the project
        let sprint_id: i32 = match ctx.query.get("sprint_id").and_then(|v| v.parse().ok()) {
            Some(id) => id,
            None => {
                // Try to find the active sprint for this project
                match board_service.get_active_sprint(project_id).await {
                    Ok(sprint) => sprint.id,
                    Err(DbErr::RecordNotFound(_)) => {
                        // No active sprint found, use default or create one
                        // For now, try to get any sprint for this project
                        match board_service.get_first_sprint(project_id).await {
                            Ok(sprint) => sprint.id,
                            Err(DbErr::RecordNotFound(_)) => {
                                // No sprints exist, show empty board with message
                                let error_markup = html! {
                                    div class="p-6 text-center text-gray-400" {
                                        div class="text-4xl mb-4" { "📋" }
                                        h2 class="text-lg font-bold text-white" { "No Sprints Found" }
                                        p class="text-sm mt-2" { "This project doesn't have any sprints yet." }
                                        a href=(format!("/jira/backlog?project_id={}", project_id))
                                            hx-get=(format!("/jira/backlog?project_id={}", project_id))
                                            hx-target="#main-content"
                                            hx-push-url="true"
                                            class="mt-4 inline-block bg-blue-600 hover:bg-blue-500 text-white px-4 py-2 rounded-lg text-xs transition" {
                                            { "Go to Backlog to create a sprint" }
                                        }
                                    }
                                };
                                return error_markup.render(ctx, "Board");
                            }
                            Err(e) => {
                                let error_markup = html! {
                                    div class="p-6 text-red-400" {
                                        "Failed to load sprints: " (e.to_string())
                                    }
                                };
                                return error_markup.render(ctx, "Error");
                            }
                        }
                    }
                    Err(e) => {
                        let error_markup = html! {
                            div class="p-6 text-red-400" {
                                "Failed to load active sprint: " (e.to_string())
                            }
                        };
                        return error_markup.render(ctx, "Error");
                    }
                }
            }
        };

        // Get the project name for the header
        let project_name = match project_service.get_project_by_id(project_id).await {
            Ok(Some(project)) => &project.core.name.clone(),
            Ok(None) => "Unknown Project",
            Err(_) => "Unknown Project",
        };

        // Fetch the board columns
        let columns = match board_service
            .get_kanban_columns(project_id, sprint_id)
            .await
        {
            Ok(cols) => cols,
            Err(e) => {
                let error_markup = html! {
                    div class="p-6 text-red-400" {
                        "Failed to load board: " (e.to_string())
                    }
                };
                return error_markup.render(ctx, "Error");
            }
        };

        // Render the board view
        let board_markup = kanban_board_view(&columns, project_id, sprint_id, &project_name);
        board_markup.render(ctx, &format!("{} | Board", project_name))
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
        let jql = ctx
            .query
            .get("jql")
            .map(|v| v.to_string())
            .unwrap_or("".to_string());

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

    /// GET /jira/projects
    #[get("/projects")]
    pub async fn projects_page(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let projects = match project_service.list_projects().await {
            Ok(p) => p,
            Err(e) => {
                let error_markup = html! {
                    div class="p-6 text-red-400" {
                        "Failed to load projects: " (e.to_string())
                    }
                };
                return error_markup.render(ctx, "Error");
            }
        };

        let markup = projects_view(&projects);
        markup.render(ctx, "Projects")
    }

    /// GET /jira/projects/:id (optional - project detail page)
    #[get("/projects/:id")]
    pub async fn project_detail_page(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => {
                let error_markup = html! {
                    div class="p-6 text-red-400" { "Invalid project ID" }
                };
                return error_markup.render(ctx, "Error");
            }
        };

        match project_service.get_project_with_issues(project_id).await {
            Ok(Some((project, issues))) => {
                let markup = html! {
                    div class="p-6 space-y-4 font-mono text-xs text-gray-200" {
                        div class="flex justify-between items-center border-b border-gray-800 pb-4" {
                            div {
                                h1 class="text-xl font-bold text-white" { (project.name) }
                                span class="text-gray-400 text-xxs" { (project.key) }
                            }
                            span class="text-gray-400" { (format!("{} issues", issues.len())) }
                        }
                        div class="mt-4 space-y-2" {
                            @for issue in issues {
                                div class="bg-gray-900 border border-gray-800 rounded-lg p-3 flex justify-between items-center" {
                                    span class="text-blue-400 font-bold" { (issue.key) }
                                    span class="text-gray-300" { (issue.summary) }
                                    span class="text-xxs bg-gray-800 text-gray-400 px-2 py-0.5 rounded" { (issue.issue_type) }
                                }
                            }
                        }
                    }
                };
                markup.render(ctx, &format!("{} | Project", project.key))
            }
            Ok(None) => {
                let error_markup = html! {
                    div class="p-6 text-red-400" { "Project not found" }
                };
                error_markup.render(ctx, "Error")
            }
            Err(e) => {
                let error_markup = html! {
                    div class="p-6 text-red-400" { "Failed to load project: " (e.to_string()) }
                };
                error_markup.render(ctx, "Error")
            }
        }
    }
}
