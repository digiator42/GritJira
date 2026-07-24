use crate::{models::IssueModel, web::components::badge::priority_badge};
use maud::{Markup, html};

pub fn issue_card(issue: &IssueModel) -> Markup {
    html! {
        div id={(format!("issue-{}", issue.id))}
            class="bg-gray-900 border border-gray-800 p-3 rounded-lg shadow-sm hover:border-gray-700 cursor-grab active:cursor-grabbing transition"
            data-issue-id={(issue.id)}
            hx-get={(format!("/issues/{}/detail-modal", issue.id))}
            hx-target="#modals-container"
            hx-swap="innerHTML" {

            div class="flex items-center justify-between mb-2" {
                span class="text-xxs font-mono text-gray-500 uppercase tracking-wider" { (issue.key) }
                (priority_badge(&issue.priority))
            }
            p class="text-xs font-semibold text-gray-200 line-clamp-2" { (issue.summary) }
        }
    }
}
