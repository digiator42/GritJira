use gritshield::routing::RequestContext;
use maud::{Markup, html};

use crate::models::ProjectModel;

pub fn projects_view(projects: &[crate::models::ProjectModel]) -> Markup {
    html! {
        div class="p-6 space-y-4 font-mono text-xs text-gray-200" {
            div class="flex justify-between items-center border-b border-gray-800 pb-4" {
                div {
                    h1 class="text-xl font-bold text-white tracking-wide" { "Projects" }
                    span class="text-gray-400 text-xxs" { (format!("{} projects", projects.len())) }
                }
                // Create Project Button
                button
                    hx-get="/jira/projects/new-modal"
                    hx-target="#modals-container"
                    hx-swap="innerHTML"
                    class="bg-green-600 hover:bg-green-500 text-white font-mono font-semibold text-xs py-2 px-4 rounded-lg transition duration-150 flex items-center gap-2 shadow-lg shadow-green-950/50" {
                    span { "+" }
                    span { "Create Project" }
                }
            }

            div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4" {
                @for project in projects {
                    div class="bg-gray-900 border border-gray-800 rounded-lg p-4 hover:border-gray-700 transition" {
                        div class="flex items-center justify-between" {
                            div class="flex items-center gap-2" {
                                span class="text-xl" { "📁" }
                                h2 class="text-sm font-bold text-white" { (project.name) }
                            }
                            span class="text-xxs bg-blue-950 text-blue-400 border border-blue-800/60 px-2 py-0.5 rounded" {
                                (project.key)
                            }
                        }
                        p class="text-gray-400 text-xxs mt-2" { (project.description.as_deref().unwrap_or("No description")) }
                        div class="mt-3 flex items-center gap-2" {
                            a href={"/jira/board?project_id=" (project.id)}
                               hx-get={"/jira/board?project_id=" (project.id)}
                               hx-target="#main-content"
                               hx-push-url="true"
                               class="text-xs text-blue-400 hover:underline" {
                                "View Board"
                            }
                            a href={"/jira/backlog?project_id=" (project.id)}
                               hx-get={"/jira/backlog?project_id=" (project.id)}
                               hx-target="#main-content"
                               hx-push-url="true"
                               class="text-xs text-gray-400 hover:underline" {
                                "Backlog"
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn project_selector(ctx: &RequestContext, projects: &[ProjectModel]) -> Markup {
    let current_project_id: i32 = ctx
        .get_session_data("current_project_id")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(1);

    html! {
        div class="relative" {
            select
                id="project-select"
                name="project_id"
                hx-get="/jira/switch-project"
                hx-trigger="change"
                hx-target="#main-content"
                hx-swap="innerHTML"
                hx-push-url="true"
                class="w-full bg-gray-950 border border-gray-800 rounded-lg px-3 py-1.5 text-sm text-gray-200 focus:outline-none focus:border-blue-500" {

                @if projects.is_empty() {
                    option value="1" selected { "No Projects Available" }
                } @else {
                    @for project in projects {
                        @let is_selected = project.id == current_project_id;
                        option value=(project.id) selected[is_selected] {
                            (project.name) " (" (project.key) ")"
                        }
                    }
                }
            }
        }
    }
}
