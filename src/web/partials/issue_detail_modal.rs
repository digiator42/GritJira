use crate::models::IssueModel;
use maud::{Markup, html};

pub fn issue_detail_modal(issue: &IssueModel) -> Markup {
    html! {
        div id="issue-detail-modal"
            class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm p-4"
            onclick="if(event.target === this) this.remove()" {
            div class="bg-gray-900 border border-gray-800/60 rounded-xl w-full max-w-2xl max-h-[90vh] flex flex-col shadow-2xl overflow-hidden"
                onclick="event.stopPropagation()" {
                // Header
                div class="flex items-center justify-between px-6 py-4 border-b border-gray-800/60 bg-gray-900/50" {
                    div class="flex items-center gap-3" {
                        span class="text-blue-400 font-mono text-sm font-bold" { (issue.key) }
                        span class="text-gray-400 text-xs bg-gray-800 px-2 py-0.5 rounded" { (issue.issue_type) }
                    }
                    button
                        onclick="document.getElementById('issue-detail-modal').remove()"
                        class="text-gray-500 hover:text-gray-300 transition p-1 rounded hover:bg-gray-800/50" {
                        span class="text-sm" { "✕" }
                    }
                }
                // Body
                div class="flex-1 overflow-y-auto p-6 space-y-4" {
                    h2 class="text-lg font-bold text-white" { (issue.summary) }

                    @if let Some(desc) = &issue.description {
                        p class="text-sm text-gray-400 whitespace-pre-wrap" { (desc) }
                    } @else {
                        p class="text-sm text-gray-500 italic" { "No description provided." }
                    }

                    div class="grid grid-cols-2 gap-4 mt-4 pt-4 border-t border-gray-800" {
                        div {
                            span class="text-xxs text-gray-500 uppercase tracking-wider" { "Status" }
                            p class="text-sm font-mono text-gray-300" { (issue.issue_type) }
                        }
                        div {
                            span class="text-xxs text-gray-500 uppercase tracking-wider" { "Priority" }
                            p class="text-sm font-mono text-gray-300" { (issue.priority) }
                        }
                        div {
                            span class="text-xxs text-gray-500 uppercase tracking-wider" { "Assignee" }
                            p class="text-sm font-mono text-gray-300" {
                                // (&issue.assignee_id)
                            }
                        }
                        div {
                            span class="text-xxs text-gray-500 uppercase tracking-wider" { "Reporter" }
                            p class="text-sm font-mono text-gray-300" {
                                // (&issue.reporter_id)
                            }
                        }
                    }
                }
                // Footer
                div class="flex items-center justify-end px-6 py-4 border-t border-gray-800/60 bg-gray-900/50" {
                    button
                        onclick="document.getElementById('issue-detail-modal').remove()"
                        class="px-4 py-2 text-sm font-mono text-gray-400 hover:text-gray-200 hover:bg-gray-800/50 rounded-lg transition" {
                        "Close"
                    }
                }
            }
        }

        // Styles for modal animation
        style {
            (maud::PreEscaped(r#"
                @keyframes fade-in {
                    from { opacity: 0; }
                    to { opacity: 1; }
                }
                @keyframes slide-up {
                    from { opacity: 0; transform: translateY(20px) scale(0.98); }
                    to { opacity: 1; transform: translateY(0) scale(1); }
                }
                #issue-detail-modal {
                    animation: fade-in 0.2s ease-out;
                }
                #issue-detail-modal > div {
                    animation: slide-up 0.25s ease-out;
                }
            "#))
        }
    }
}
