use gritshield::routing::RequestContext;
use maud::{Markup, html};

use crate::models::ProjectModel;

pub fn projects_view(projects: &[crate::models::ProjectModel]) -> Markup {
    html! {
        div class="p-6 space-y-6 font-mono text-xs text-gray-200" {
            div class="flex justify-between items-center border-b border-gray-800/60 pb-4" {
                div {
                    h1 class="text-2xl font-bold text-white tracking-wide" { "Projects" }
                    p class="text-gray-400 text-sm mt-1" { (format!("Manage your {} projects", projects.len())) }
                }
                // Create Project Button
                button
                    hx-get="/jira/projects/new-modal"
                    hx-target="#modals-container"
                    hx-swap="innerHTML"
                    class="bg-gradient-to-r from-emerald-600 to-green-600 hover:from-emerald-500 hover:to-green-500 text-white font-mono font-semibold text-xs py-2.5 px-4 rounded-lg transition-all duration-200 flex items-center gap-2 shadow-lg shadow-emerald-900/50 hover:shadow-xl hover:shadow-emerald-900/60 hover:scale-[1.02] active:scale-[0.98]" {
                    span { "+" }
                    span { "Create Project" }
                }
            }

            @if projects.is_empty() {
                div class="bg-gray-900/50 border border-dashed border-gray-700 rounded-xl p-12 text-center" {
                    div class="w-16 h-16 bg-gray-800/50 rounded-full flex items-center justify-center mx-auto mb-4" {
                        span class="text-3xl" { "📁" }
                    }
                    h3 class="text-lg font-semibold text-white mb-2" { "No projects yet" }
                    p class="text-gray-400 text-sm mb-4" { "Create your first project to get started with GritJira" }
                    button
                        hx-get="/jira/projects/new-modal"
                        hx-target="#modals-container"
                        hx-swap="innerHTML"
                        class="bg-blue-600 hover:bg-blue-500 text-white font-semibold text-xs py-2 px-4 rounded-lg transition-colors" {
                        "Create Project"
                    }
                }
            } @else {
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4" {
                    @for project in projects {
                        div class="group relative bg-gradient-to-br from-gray-900/90 to-gray-950/90 backdrop-blur-sm border border-gray-800/80 rounded-xl p-5 hover:border-gray-700/80 hover:shadow-xl hover:shadow-gray-900/30 transition-all duration-300 hover:scale-[1.02] hover:-translate-y-1" {
                            // Project icon and header
                            div class="flex items-start justify-between mb-4" {
                                div class="flex items-center gap-3" {
                                    div class="w-12 h-12 bg-gradient-to-br from-blue-500 to-indigo-600 rounded-xl flex items-center justify-center shadow-lg shadow-blue-500/30 group-hover:scale-110 transition-transform" {
                                        span class="text-2xl" { "📁" }
                                    }
                                    div {
                                        h2 class="text-sm font-bold text-white" { (project.name) }
                                        span class="text-xxs bg-blue-500/20 text-blue-400 border border-blue-500/30 px-2 py-0.5 rounded-full font-mono" {
                                            (project.key)
                                        }
                                    }
                                }
                                // Quick actions on hover
                                div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity" {
                                    button class="p-1.5 text-gray-400 hover:text-white hover:bg-gray-800/50 rounded-lg transition-colors" {
                                        span class="text-sm" { "⚙️" }
                                    }
                                    button class="p-1.5 text-gray-400 hover:text-red-400 hover:bg-red-900/20 rounded-lg transition-colors" {
                                        span class="text-sm" { "🗑️" }
                                    }
                                }
                            }
                            
                            p class="text-gray-400 text-sm line-clamp-2 mb-4 min-h-[2.5rem]" { 
                                (project.description.as_deref().unwrap_or("No description provided for this project.")) 
                            }
                            
                            // Project stats
                            div class="grid grid-cols-3 gap-2 mb-4 pt-4 border-t border-gray-800/50" {
                                div class="text-center" {
                                    p class="text-lg font-bold text-white" { "12" }
                                    p class="text-xxs text-gray-500 mt-1" { "Issues" }
                                }
                                div class="text-center" {
                                    p class="text-lg font-bold text-blue-400" { "3" }
                                    p class="text-xxs text-gray-500 mt-1" { "Sprints" }
                                }
                                div class="text-center" {
                                    p class="text-lg font-bold text-emerald-400" { "5" }
                                    p class="text-xxs text-gray-500 mt-1" { "Members" }
                                }
                            }
                            
                            // Action buttons
                            div class="flex items-center gap-2" {
                                a href={"/jira/board?project_id=" (project.id)}
                                   hx-get={"/jira/board?project_id=" (project.id)}
                                   hx-target="#main-content"
                                   hx-push-url="true"
                                   class="flex-1 text-center py-2 bg-blue-600/20 hover:bg-blue-600/30 text-blue-400 hover:text-blue-300 rounded-lg transition-colors font-medium" {
                                    "📋 Board"
                                }
                                a href={"/jira/backlog?project_id=" (project.id)}
                                   hx-get={"/jira/backlog?project_id=" (project.id)}
                                   hx-target="#main-content"
                                   hx-push-url="true"
                                   class="flex-1 text-center py-2 bg-gray-800/50 hover:bg-gray-800/80 text-gray-400 hover:text-white rounded-lg transition-colors font-medium" {
                                    "📦 Backlog"
                                }
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
