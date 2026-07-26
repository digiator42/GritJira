use crate::models::{IssueModel, SprintModel};
use maud::{Markup, html};

pub fn backlog_view(backlog: &[IssueModel], sprints: &[SprintModel], project_id: i32) -> Markup {
    html! {
        div class="p-6 space-y-6 font-mono text-xs text-gray-200 overflow-y-auto" {
            // Header with Create Sprint button
            div class="flex justify-between items-center border-b border-gray-800 pb-4" {
                div {
                    h1 class="text-xl font-bold text-white tracking-wide" { "Backlog & Sprints" }
                    span class="text-gray-400 text-xxs" { (format!("{} Backlog Items", backlog.len())) }
                }
                div class="flex items-center gap-3" {
                    // Create Sprint Button
                    button
                        hx-get={"/jira/sprints/new-modal?project_id=" (project_id)}
                        hx-target="#modals-container"
                        hx-swap="innerHTML"
                        class="bg-green-600 hover:bg-green-500 text-white font-mono font-semibold text-xs py-2 px-4 rounded-lg transition duration-150 flex items-center gap-2 shadow-lg shadow-green-950/50" {
                        span { "+" }
                        span { "Create Sprint" }
                    }

                    // Refresh button
                    button
                        hx-get={"/jira/backlog?project_id=" (project_id)}
                        hx-target="#main-content"
                        hx-swap="innerHTML"
                        class="bg-gray-800 hover:bg-gray-700 text-gray-300 font-mono text-xs py-2 px-4 rounded-lg transition" {
                        "🔄 Refresh"
                    }
                }
            }

            // Active Sprints Section
            div id="sprint-list" class="space-y-4" {
                div class="flex items-center justify-between" {
                    h2 class="text-sm font-bold uppercase tracking-wider text-blue-400" { "Active Sprints" }
                    span class="text-xxs text-gray-500" { (format!("{} sprints", sprints.len())) }
                }

                @if sprints.is_empty() {
                    div class="bg-gray-900/50 border border-dashed border-gray-700 rounded-lg p-6 text-center" {
                        p class="text-gray-500" { "No active sprints yet." }
                        p class="text-gray-600 text-xxs mt-1" { "Click 'Create Sprint' to start planning!" }
                    }
                } @else {
                    @for sprint in sprints {
                        div class="bg-gray-900 border border-gray-800 rounded-lg p-4 space-y-2 hover:border-gray-700 transition" {
                            div class="flex justify-between items-center border-b border-gray-800 pb-2" {
                                div class="flex items-center gap-3" {
                                    div class="font-bold text-white text-sm" { (sprint.name) }
                                    span class="bg-emerald-950 text-emerald-400 border border-emerald-800/60 px-2 py-0.5 rounded text-xxs uppercase" {
                                        (sprint.status)
                                    }
                                }
                                div class="flex items-center gap-2" {
                                    // Start Sprint button (if not started)
                                    @if sprint.status != "Active" {
                                        button
                                            hx-post={"/api/v1/sprints/" (sprint.id) "/start"}
                                            hx-ext="json-enc"
                                            hx-target="#main-content"
                                            hx-swap="innerHTML"
                                            hx-on--after-request="this.closest('.bg-gray-900').remove()"
                                            class="text-xxs bg-blue-600 hover:bg-blue-500 text-white px-2 py-1 rounded transition" {
                                            "Start Sprint"
                                        }
                                    }
                                    // View Board link
                                    a href={"/jira/board?project_id=" (project_id) "&sprint_id=" (sprint.id)}
                                        hx-get={"/jira/board?project_id=" (project_id) "&sprint_id=" (sprint.id)}
                                        hx-target="#main-content"
                                        hx-push-url="true"
                                        class="text-xxs text-blue-400 hover:underline" {
                                        "View Board"
                                    }
                                }
                            }
                            p class="text-gray-400 text-xxs" { (sprint.goal.as_deref().unwrap_or("No sprint goal set.")) }
                        }
                    }
                }
            }

            // Unassigned Backlog Section
            div class="space-y-3 pt-4 border-t border-gray-800" {
                div class="flex items-center justify-between" {
                    h2 class="text-sm font-bold uppercase tracking-wider text-gray-400" { "Unassigned Backlog" }
                    span class="text-xxs text-gray-500" { (format!("{} items", backlog.len())) }
                }

                div class="space-y-2" {
                    @if backlog.is_empty() {
                        div class="bg-gray-900/50 border border-dashed border-gray-700 rounded-lg p-6 text-center" {
                            p class="text-gray-500" { "Backlog is empty." }
                            p class="text-gray-600 text-xxs mt-1" { "Create issues from the board to populate the backlog." }
                        }
                    } @else {
                        @for issue in backlog {
                            div class="bg-gray-900/80 border border-gray-800 rounded-lg p-3 flex justify-between items-center hover:border-gray-700 transition" {
                                div class="flex items-center gap-3 flex-1" {
                                    span class="text-blue-400 font-bold text-xxs" { (issue.key) }
                                    span class="text-white text-xs" { (issue.summary) }
                                    span class="text-xxs bg-gray-800 text-gray-400 px-2 py-0.5 rounded" { (issue.issue_type) }
                                }

                                div class="flex items-center gap-2" {
                                    select
                                        name="sprint_id"
                                        hx-post={"/api/v1/backlog/issues/" (issue.id) "/assign-sprint"}
                                        hx-trigger="change"
                                        hx-target="closest div"
                                        hx-swap="outerHTML"
                                        hx-indicator="this"
                                        class="bg-gray-950 border border-gray-800 rounded px-2 py-1 text-gray-300 focus:outline-none focus:border-blue-500 text-xxs" {
                                            option value="" selected { "Assign to Sprint..." }
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
}
