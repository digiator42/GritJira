// src/web/views/board_view.rs
use maud::{html, Markup};
use crate::models::{WorkflowStepModel, IssueModel};
use crate::web::partials::issue_card::issue_card;

pub fn kanban_board_view(columns: &[(WorkflowStepModel, Vec<IssueModel>)]) -> Markup {
    html! {
        div class="flex gap-4 h-[calc(100vh-8rem)] overflow-x-auto p-4" {
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
                            (issue_card(issue))
                        }
                    }
                }
            }
        }
    }
}