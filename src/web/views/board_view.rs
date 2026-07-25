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
            // Board Header
            div class="flex items-center justify-between px-4 py-3 border-b border-gray-800 bg-gray-900/30 flex-shrink-0" {
                div class="flex items-center gap-3" {
                    span class="text-sm font-bold text-white" { (project_name) }
                    span class="text-xxs text-gray-400 font-mono" { "Sprint #" (sprint_id) }
                }
                div class="flex items-center gap-2" {
                    a href={"/jira/board?project_id=" (project_id)}
                        hx-get={"/jira/board?project_id=" (project_id)}
                        hx-target="#main-content"
                        hx-push-url="true"
                        class="text-xxs text-blue-400 hover:underline" {
                        "Refresh"
                    }
                    a href={"/jira/backlog?project_id=" (project_id)}
                        hx-get={"/jira/backlog?project_id=" (project_id)}
                        hx-target="#main-content"
                        hx-push-url="true"
                        class="text-xxs text-gray-400 hover:underline" {
                        "Backlog"
                    }
                }
            }

            // Board Columns
            div class="flex gap-4 h-[calc(100vh-12rem)] overflow-x-auto p-4" {
                @for (column, issues) in columns {
                    div class="w-80 bg-gray-950 border border-gray-800/80 rounded-xl flex flex-col flex-shrink-0" {
                        // Column Header
                        div class="p-3 border-b border-gray-800 flex justify-between items-center bg-gray-900/30" {
                            h3 class="text-xs font-mono font-bold text-gray-300 uppercase tracking-wider" { (column.name) }
                            span class="text-xxs font-mono bg-gray-800 text-gray-400 px-2 py-0.5 rounded-full" { (issues.len()) }
                        }

                        // Drop Target Container (HTMX + SortableJS)
                        div id={(format!("col-{}", column.id))}
                            class="flex-1 overflow-y-auto p-2 space-y-2 sortable-column"
                            data-column-id={(column.id)} {
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
