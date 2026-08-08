// src/web/partials/issue_card.rs
use crate::{models::IssueModel, web::components::badge::priority_badge};
use maud::{Markup, html};

pub fn issue_card(issue: &IssueModel) -> Markup {
    let priority_color = match issue.priority.to_lowercase().as_str() {
        "high" | "p1" => "border-l-red-500",
        "medium" | "p2" => "border-l-amber-500",
        _ => "border-l-blue-500",
    };

    html! {
        div id={(format!("issue-{}", issue.id))}
            class={"group relative bg-gradient-to-br from-gray-900/90 to-gray-950/90 backdrop-blur-sm border border-gray-800/80 p-4 rounded-xl shadow-sm hover:shadow-xl hover:shadow-gray-900/30 hover:border-gray-700/80 cursor-grab active:cursor-grabbing transition-all duration-300 hover:scale-[1.02] hover:-translate-y-0.5 border-l-4 " (priority_color)}
            data-issue-id={(issue.id)}
            onclick={"if (!this.classList.contains('is-dragging')) { htmx.ajax('GET', '/jira/issues/" (issue.id) "/detail-modal', {target: '#modals-container', swap: 'innerHTML'}) }"} {

            // Priority indicator bar on the left
            div class="absolute left-0 top-0 bottom-0 w-1 rounded-l-xl opacity-0 group-hover:opacity-100 transition-opacity" {
                @if issue.priority.to_lowercase() == "high" || issue.priority.to_lowercase() == "p1" {
                    div class="h-full bg-gradient-to-b from-red-500 to-red-600" {}
                } @else if issue.priority.to_lowercase() == "medium" || issue.priority.to_lowercase() == "p2" {
                    div class="h-full bg-gradient-to-b from-amber-500 to-amber-600" {}
                } @else {
                    div class="h-full bg-gradient-to-b from-blue-500 to-blue-600" {}
                }
            }

            div class="flex items-start justify-between mb-3" {
                div class="flex items-center gap-2" {
                    span class="text-xs font-mono text-blue-400 font-semibold tracking-wide" { (issue.key) }
                    @if let Some(story_points) = issue.story_points {
                        span class="text-xxs bg-gray-800/80 text-gray-400 px-1.5 py-0.5 rounded font-mono" { (story_points) " pts" }
                    }
                }
                (priority_badge(&issue.priority))
            }
            
            p class="text-sm font-medium text-gray-200 line-clamp-2 mb-3 group-hover:text-white transition-colors" { (issue.summary) }
            
            // Bottom metadata row
            div class="flex items-center justify-between pt-3 border-t border-gray-800/50" {
                div class="flex items-center gap-2" {
                    span class="text-xxs bg-gray-800/50 text-gray-400 px-2 py-1 rounded-md font-medium" { (issue.issue_type) }
                    @if let Some(assignee_id) = &issue.assignee_id {
                        div class="flex items-center gap-1.5" {
                            div class="w-5 h-5 bg-gradient-to-br from-purple-500 to-pink-500 rounded-full flex items-center justify-center text-xs font-bold text-white" {
                                "👤"
                            }
                            span class="text-xxs text-gray-400" { (assignee_id) }
                        }
                    }
                }
                // Actions on hover
                div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity" {
                    button class="p-1 text-gray-500 hover:text-white hover:bg-gray-800/50 rounded transition-colors" {
                        span class="text-xs" { "✏️" }
                    }
                    button class="p-1 text-gray-500 hover:text-red-400 hover:bg-red-900/20 rounded transition-colors" {
                        span class="text-xs" { "🗑️" }
                    }
                }
            }
        }
    }
}