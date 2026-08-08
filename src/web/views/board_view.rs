// src/web/views/board_view.rs
use crate::models::{IssueModel, WorkflowStepModel};
use maud::{Markup, html};

pub fn kanban_board_view(
    columns: &[(WorkflowStepModel, Vec<IssueModel>)],
    project_id: i32,
    sprint_id: i32,
    project_name: &str,
) -> Markup {
    html! {
        div class="flex flex-col h-full" {
            // Board Header with enhanced styling
            div class="flex items-center justify-between px-6 py-4 border-b border-gray-800/60 bg-gradient-to-r from-gray-900/50 to-transparent flex-shrink-0" {
                div class="flex items-center gap-4" {
                    div class="w-10 h-10 bg-gradient-to-br from-emerald-500 to-green-600 rounded-xl flex items-center justify-center shadow-lg shadow-emerald-500/30" {
                        span class="text-white text-lg" { "📋" }
                    }
                    div {
                        h2 class="text-lg font-bold text-white" { (project_name) }
                        div class="flex items-center gap-2 mt-1" {
                            span class="text-xs text-gray-400 font-mono" { "Sprint #" (sprint_id) }
                            span class="text-xs bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 px-2 py-0.5 rounded-full" { "Active" }
                        }
                    }
                }
                div class="flex items-center gap-2" {
                    a href={"/jira/board?project_id=" (project_id)}
                        hx-get={"/jira/board?project_id=" (project_id)}
                        hx-target="#main-content"
                        hx-push-url="true"
                        class="px-3 py-2 text-xs text-gray-400 hover:text-white hover:bg-gray-800/50 rounded-lg transition-all duration-200 flex items-center gap-1" {
                        span { "🔄" }
                        span { "Refresh" }
                    }
                    a href={"/jira/backlog?project_id=" (project_id)}
                        hx-get={"/jira/backlog?project_id=" (project_id)}
                        hx-target="#main-content"
                        hx-push-url="true"
                        class="px-3 py-2 text-xs text-gray-400 hover:text-white hover:bg-gray-800/50 rounded-lg transition-all duration-200 flex items-center gap-1" {
                        span { "📦" }
                        span { "Backlog" }
                    }
                }
            }

            // Board Columns with enhanced styling
            div class="flex gap-5 h-[calc(100vh-13rem)] overflow-x-auto p-6" {
                @for (column, issues) in columns {
                    div class="w-80 bg-gradient-to-br from-gray-900/90 to-gray-950/90 backdrop-blur-sm border border-gray-800/80 rounded-2xl flex flex-col flex-shrink-0 shadow-xl shadow-gray-900/20" {
                        // Column Header
                        div class="p-4 border-b border-gray-800/60 flex justify-between items-center bg-gradient-to-r from-gray-900/50 to-transparent" {
                            h3 class="text-sm font-mono font-bold text-gray-200 uppercase tracking-wider" { (column.name) }
                            span class={"text-xs font-mono px-2.5 py-1 rounded-full border " 
                                (if issues.len() > 0 { "bg-blue-500/20 text-blue-400 border-blue-500/30" } else { "bg-gray-800/50 text-gray-500 border-gray-700/50" })} {
                                (issues.len())
                            }
                        }

                        // Drop Target Container (HTMX + SortableJS)
                        div id={(format!("col-{}", column.id))}
                            class="flex-1 overflow-y-auto p-3 space-y-3 sortable-column min-h-[100px] transition-colors duration-200"
                            data-column-id={(column.id)} {
                            @if issues.is_empty() {
                                div class="text-center py-8 text-gray-600 text-xs border-2 border-dashed border-gray-800/50 rounded-xl" {
                                    p { "No issues yet" }
                                    p class="mt-1" { "Drag issues here" }
                                }
                            }
                            @for issue in issues {
                                (crate::web::partials::issue_card::issue_card(issue))
                            }
                        }
                    }
                }
            }
        }
    }
}
