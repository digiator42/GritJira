use maud::{html, Markup};
use crate::models::{IssueModel, SprintModel};

pub fn backlog_view(backlog: &[IssueModel], sprints: &[SprintModel]) -> Markup {
    html! {
        div class="p-6 space-y-6 font-mono text-xs text-gray-200 overflow-y-auto" {
            // Header
            div class="flex justify-between items-center border-b border-gray-800 pb-4" {
                h1 class="text-xl font-bold text-white tracking-wide" { "Backlog & Sprints" }
                span class="text-gray-400" { (format!("{} Backlog Items", backlog.len())) }
            }

            // Active Sprints Section
            div class="space-y-4" {
                h2 class="text-sm font-bold uppercase tracking-wider text-blue-400" { "Active Sprints" }
                
                @if sprints.is_empty() {
                    p class="text-gray-500 italic" { "No active sprints found." }
                } @else {
                    @for sprint in sprints {
                        div class="bg-gray-900 border border-gray-800 rounded-lg p-4 space-y-2" {
                            div class="flex justify-between items-center border-b border-gray-800 pb-2" {
                                div class="font-bold text-white text-sm" { (sprint.name) }
                                span class="bg-emerald-950 text-emerald-400 border border-emerald-800/60 px-2 py-0.5 rounded text-xxs uppercase" {
                                    (sprint.status)
                                }
                            }
                            p class="text-gray-400 text-xxs" { (sprint.goal.as_deref().unwrap_or("No sprint goal set.")) }
                        }
                    }
                }
            }

            // Unassigned Backlog Section
            div class="space-y-3 pt-4 border-t border-gray-800" {
                h2 class="text-sm font-bold uppercase tracking-wider text-gray-400" { "Unassigned Backlog" }

                div class="space-y-2" {
                    @for issue in backlog {
                        div class="bg-gray-900/80 border border-gray-800 rounded-lg p-3 flex justify-between items-center hover:border-gray-700 transition" {
                            div class="flex items-center gap-3" {
                                span class="text-blue-400 font-bold" { (issue.key) }
                                span class="text-white" { (issue.summary) }
                            }

                            div class="flex items-center gap-2" {
                                span class="bg-gray-800 text-gray-400 px-2 py-0.5 rounded text-xxs uppercase" { (issue.issue_type) }
                                
                                select
                                    name="sprint_id"
                                    hx-post={(format!("/admin/jira/issues/{}/assign-sprint", issue.id))}
                                    hx-trigger="change"
                                    class="bg-gray-950 border border-gray-800 rounded px-2 py-1 text-gray-300 focus:outline-none" {
                                        option value="none" selected { "Assign to Sprint..." }
                                        @for sprint in sprints {
                                            option value={(sprint.id)} { (sprint.name) }
                                        }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}