use crate::repositories::sprint::SprintRepository;
use crate::services::JqlParser;
use crate::services::project_service::ProjectService;
use crate::services::{board_service::BoardService, issue_service::IssueService};
use crate::web::partials::create_issue_modal::create_issue_modal;
use crate::web::partials::create_project_modal::create_project_modal;
use crate::web::partials::create_sprint_modal::create_sprint_modal;
use crate::web::partials::issue_detail_modal::{self, issue_detail_modal};
use crate::web::render::MaudRender;
use crate::web::views::auth_view::login_page_view;
use crate::web::views::helpers::get_project_context;
use crate::web::views::projects_view::{project_selector, projects_view};
use crate::web::views::search_view::search_page;
use crate::web::views::{backlog_view::backlog_view, board_view::kanban_board_view};
use gritshield::database::GritRepository;
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use sea_orm::ColumnTrait;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use std::sync::Arc;

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
        let project_id = get_project_context(&ctx);

        // Get project details
        let project = match project_service.get_project_by_id(project_id).await {
            Ok(Some(p)) => p,
            Ok(None) => {
                let error_markup = html! {
                    div class="flex flex-col items-center justify-center h-full text-center p-8" {
                        div class="text-6xl mb-6" { "❌" }
                        h2 class="text-xl font-bold text-white mb-2" { "Project Not Found" }
                        p class="text-gray-400" { "The project you're looking for doesn't exist." }
                        a href="/jira/projects"
                            hx-get="/jira/projects"
                            hx-target="#main-content"
                            hx-push-url="true"
                            class="mt-4 text-blue-400 hover:underline" {
                            "View All Projects"
                        }
                    }
                };
                return error_markup.render(ctx, "Error");
            }
            Err(e) => {
                let error_markup = html! {
                    div class="p-6 text-red-400" {
                        "Failed to load project: " (e.to_string())
                    }
                };
                return error_markup.render(ctx, "Error");
            }
        };

        // Try to get a sprint for this project
        let sprint_id = match ctx.query.get("sprint_id").and_then(|v| v.parse().ok()) {
            Some(id) => id,
            None => {
                // Try to find active sprint
                match board_service.get_active_sprint(project_id).await {
                    Ok(sprint) => sprint.id,
                    Err(DbErr::RecordNotFound(_)) => {
                        // Try first sprint
                        match board_service.get_first_sprint(project_id).await {
                            Ok(sprint) => sprint.id,
                            Err(DbErr::RecordNotFound(_)) => {
                                // No sprints - show empty state with setup actions
                                let empty_markup = html! {
                                    div class="flex flex-col items-center justify-center h-full text-center p-8" {
                                        div class="text-6xl mb-6" { "📋" }
                                        h2 class="text-xl font-bold text-white mb-2" { "No Sprints Yet" }
                                        p class="text-gray-400 mb-2" {
                                            "Project \"" (project.name) "\" doesn't have any sprints."
                                        }
                                        p class="text-gray-400 mb-6" {
                                            "Create your first sprint from the backlog."
                                        }
                                        div class="flex items-center gap-4" {
                                            a href={"/jira/backlog?project_id=" (project_id)}
                                                hx-get={"/jira/backlog?project_id=" (project_id)}
                                                hx-target="#main-content"
                                                hx-push-url="true"
                                                class="px-6 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded-lg transition" {
                                                "Go to Backlog"
                                            }
                                            a href="/jira/projects"
                                                hx-get="/jira/projects"
                                                hx-target="#main-content"
                                                hx-push-url="true"
                                                class="px-6 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition" {
                                                "View All Projects"
                                            }
                                        }
                                        div class="mt-6 text-xxs text-gray-500" {
                                            "Or create a new project with default workflow steps"
                                        }
                                    }
                                };
                                return empty_markup.render(ctx, "Board");
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

        // Get the workflow steps for this project
        let columns = match board_service
            .get_kanban_columns(project_id, sprint_id)
            .await
        {
            Ok(cols) => {
                if cols.is_empty() {
                    // No workflow steps configured - show setup message with fix action
                    let empty_markup = html! {
                        div class="flex flex-col items-center justify-center h-full text-center p-8" {
                            div class="text-6xl mb-6" { "⚙️" }
                            h2 class="text-xl font-bold text-white mb-2" { "No Workflow Configured" }
                            p class="text-gray-400 mb-2" {
                                "Project \"" (project.name) "\" doesn't have any workflow steps."
                            }
                            p class="text-gray-400 mb-6" {
                                "Workflow steps define the columns on your board."
                            }
                            div class="flex items-center gap-4 flex-wrap justify-center" {
                                a href={"/jira/backlog?project_id=" (project_id)}
                                    hx-get={"/jira/backlog?project_id=" (project_id)}
                                    hx-target="#main-content"
                                    hx-push-url="true"
                                    class="px-6 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded-lg transition" {
                                    "Go to Backlog"
                                }
                                button
                                    hx-post={"/jira/projects/" (project_id) "/workflow/create"}
                                    hx-target="#main-content"
                                    hx-swap="innerHTML"
                                    class="px-6 py-2 bg-green-600 hover:bg-green-500 text-white rounded-lg transition" {
                                    "Create Default Workflow"
                                }
                                a href="/jira/projects"
                                    hx-get="/jira/projects"
                                    hx-target="#main-content"
                                    hx-push-url="true"
                                    class="px-6 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition" {
                                    "View All Projects"
                                }
                            }
                            div class="mt-6 text-xxs text-gray-500" {
                                "Default workflow: Backlog → To Do → In Progress → In Review → Done"
                            }
                        }
                    };
                    return empty_markup.render(ctx, "Board");
                }
                cols
            }
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
        let board_markup = kanban_board_view(&columns, project_id, sprint_id, &project.name);
        board_markup.render(ctx, &format!("{} | Board", project.name))
    }

    #[get("/login")]
    pub async fn login_page(ctx: RequestContext) -> Response {
        login_page_view().render(ctx, "Login Page")
    }

    /// GET /jira/backlog - Update to accept project_id
    #[get("/backlog")]
    pub async fn backlog_page(
        ctx: RequestContext,
        issue_service: Arc<IssueService>,
        sprint_repo: Arc<SprintRepository>,
    ) -> Response {
        let project_id = get_project_context(&ctx);

        let backlog = issue_service
            .get_backlog_issues(project_id)
            .await
            .unwrap_or_default();
        let sprints = sprint_repo
            .query()
            .where_eq(crate::models::sprint::Column::ProjectId, project_id)
            .order_desc(crate::models::sprint::Column::EndDate)
            .fetch()
            .await
            .unwrap_or_default();

        let markup = backlog_view(&backlog, &sprints, project_id);
        markup.render(ctx, "Backlog")
    }

    /// GET /jira/projects/new-modal
    #[get("/projects/new-modal")]
    pub async fn new_project_modal(ctx: RequestContext) -> Response {
        let markup = create_project_modal();
        Response::ok(markup.into_string())
    }

    /// GET /jira/issues/new-modal (Target: #modals-container)
    #[get("/issues/new-modal")]
    pub async fn new_issue_modal(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
        sprint_repo: Arc<SprintRepository>,
    ) -> Response {
        let project_id = get_project_context(&ctx);

        // Get project key for display
        let project_key = match project_service.get_project_by_id(project_id).await {
            Ok(Some(p)) => p.core.key.clone(),
            Ok(None) => "UNKNOWN".to_string(),
            Err(_) => "UNKNOWN".to_string(),
        };

        // Get sprints for this project
        let sprints = sprint_repo
            .query()
            .where_eq(crate::models::sprint::Column::ProjectId, project_id)
            .order_desc(crate::models::sprint::Column::EndDate)
            .fetch()
            .await
            .unwrap_or_default();

        let modal_markup = create_issue_modal(project_id, Some(&project_key), &sprints);
        Response::ok(modal_markup.into_string())
    }

    /// GET /jira/sprints/new-modal
    #[get("/sprints/new-modal")]
    pub async fn new_sprint_modal(ctx: RequestContext) -> Response {
        let project_id = get_project_context(&ctx);

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

    /// GET /jira/project-selector - Returns the project selector dropdown
    #[get("/project-selector")]
    pub async fn project_selector_partial(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let projects = match project_service.list_projects().await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to load projects: {}", e);
                vec![]
            }
        };

        let markup = project_selector(&ctx, &projects);
        Response::ok(markup.into_string())
    }

    #[get("/switch-project")]
    pub async fn switch_project(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = ctx
            .query
            .get("project_id")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);

        // Verify project exists
        match project_service.get_project_by_id(project_id).await {
            Ok(Some(project)) => {
                // Store in session
                ctx.set_session_data("current_project_id", &project_id.to_string());

                // Also store the project key for use in other components
                ctx.set_session_data("current_project_key", &project.key);

                // Redirect to board with the new project
                let redirect_url = format!("/jira/board?project_id={}", project_id);
                Response::ok("").with_header("HX-Redirect", &redirect_url)
            }
            Ok(None) => {
                // Project not found, keep current or default to 1
                let current = ctx
                    .get_session_data("current_project_id")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);
                let redirect_url = format!("/jira/board?project_id={}", current);
                Response::ok("").with_header("HX-Redirect", &redirect_url)
            }
            Err(e) => {
                eprintln!("Error switching project: {}", e);
                let redirect_url = "/jira/board?project_id=1".to_string();
                Response::ok("").with_header("HX-Redirect", &redirect_url)
            }
        }
    }

    /// POST /jira/projects/:id/workflow/create - Create default workflow steps for a project
    #[post("/projects/:id/workflow/create")]
    pub async fn create_project_workflow(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        match project_service
            .create_default_workflow_steps(project_id)
            .await
        {
            Ok(_) => {
                // Redirect to board
                let redirect_url = format!("/jira/board?project_id={}", project_id);
                Response::ok("").with_header("HX-Redirect", &redirect_url)
            }
            Err(e) => Response::bad_request(format!("Failed to create workflow steps: {}", e)),
        }
    }

    /// GET /jira/debug/project/:id - Debug endpoint to check project data
    #[get("/debug/project/:id")]
    pub async fn debug_project(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
        board_service: Arc<BoardService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let project = project_service.get_project_by_id(project_id).await.unwrap();
        let all_issues = board_service
            .get_all_project_issues(project_id)
            .await
            .unwrap_or_default();

        // Get sprints for this project
        let sprints = crate::models::sprint::Entity::find()
            .filter(crate::models::sprint::Column::ProjectId.eq(project_id))
            .all(&project_service.repo.db)
            .await
            .unwrap_or_default();

        let markup = html! {
            div class="p-6 space-y-4 font-mono text-xs text-gray-200" {
                h1 class="text-xl font-bold text-white" { "Debug: Project " (project_id) }

                div class="bg-gray-900 border border-gray-800 rounded-lg p-4" {
                    h2 class="text-sm font-bold text-blue-400" { "Project Info" }
                    p { "Name: " (project.as_ref().map(|p| &p.name).unwrap_or(&"Unknown".to_string())) }
                    p { "Key: " (project.as_ref().map(|p| &p.key).unwrap_or(&"Unknown".to_string())) }
                }

                div class="bg-gray-900 border border-gray-800 rounded-lg p-4" {
                    h2 class="text-sm font-bold text-green-400" { "Sprints" }
                    @if sprints.is_empty() {
                        p class="text-gray-500" { "No sprints found" }
                    } @else {
                        @for sprint in sprints {
                            div class="flex items-center gap-4 p-2 border-b border-gray-800" {
                                span { "ID: " (sprint.id) }
                                span { "Name: " (sprint.name) }
                                span { "Status: " (sprint.status) }
                            }
                        }
                    }
                }

                div class="bg-gray-900 border border-gray-800 rounded-lg p-4" {
                    h2 class="text-sm font-bold text-yellow-400" { "All Issues (" (&all_issues.len()) ")" }
                    @if all_issues.is_empty() {
                        p class="text-gray-500" { "No issues found" }
                    } @else {
                        table class="w-full text-left" {
                            thead {
                                tr class="border-b border-gray-800" {
                                    th { "ID" }
                                    th { "Key" }
                                    th { "Sprint ID" }
                                    th { "Step ID" }
                                    th { "Summary" }
                                }
                            }
                            tbody {
                                @for issue in all_issues {
                                    tr class="border-b border-gray-800" {
                                        td { (issue.id) }
                                        td class="text-blue-400" { (issue.key) }
                                        td { (&issue.sprint_id.unwrap()) }
                                        td { (issue.step_id) }
                                        td { (issue.summary) }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };

        Response::ok(markup.into_string())
    }
}
