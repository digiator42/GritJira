// src/web/partials/create_issue_modal.rs

use crate::models::sprint::Model as SprintModel;
use maud::{Markup, html};

pub fn create_issue_modal(
    project_id: i32,
    project_key: Option<&str>,
    sprints: &[SprintModel],
) -> Markup {
    let project_key_display = project_key.unwrap_or("GRIT");

    html! {
        // ─── MODAL BACKDROP ───
        div id="create-issue-modal"
            class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-md animate-fade-in p-4"
            onclick="if(event.target === this) this.remove()" {

            // ─── MODAL CONTAINER ───
            div class="bg-gradient-to-br from-gray-900/95 to-gray-950/95 backdrop-blur-xl border border-gray-800/80 rounded-2xl w-full max-w-2xl max-h-[90vh] flex flex-col shadow-2xl shadow-black/50 animate-slide-up overflow-hidden"
                onclick="event.stopPropagation()" {

                // ─── HEADER ───
                div class="flex items-center justify-between px-6 py-5 border-b border-gray-800/60 bg-gradient-to-r from-blue-900/20 to-transparent flex-shrink-0" {
                    div class="flex items-center gap-3" {
                        div class="w-10 h-10 bg-gradient-to-br from-blue-500 to-indigo-600 rounded-xl flex items-center justify-center shadow-lg shadow-blue-500/30" {
                            span class="text-white text-lg font-bold" { "+" }
                        }
                        div {
                            h3 class="text-base font-bold text-white font-mono tracking-wide" { "Create Issue" }
                            p class="text-xs text-gray-400 font-mono" { (format!("{} Project", project_key_display)) }
                        }
                    }
                    button
                        onclick="document.getElementById('create-issue-modal').remove()"
                        class="text-gray-400 hover:text-white transition p-2 rounded-lg hover:bg-gray-800/50" {
                        span class="text-sm" { "✕" }
                    }
                }

                // ─── ERROR DISPLAY ───
                div id="create-issue-error" class="hidden px-6 pt-4" {
                    div class="bg-red-500/10 border border-red-500/30 rounded-xl p-4 text-center" {
                        p id="create-issue-error-message" class="text-red-400 text-sm font-medium" {}
                    }
                }

                // ─── FORM ───
                form id="create-issue-form"
                     hx-post="/api/v1/issues"
                     hx-ext="json-enc"
                     hx-target="#create-issue-result"
                     hx-swap="innerHTML"
                     hx-on--after-request="handleCreateIssueResponse(event)"
                     class="flex-1 overflow-y-auto p-6 space-y-5" {

                    // ─── HIDDEN FIELDS ───
                    input type="hidden" name="project_id" value=(project_id);

                    // ─── SUMMARY ───
                    div class="space-y-2" {
                        label class="block text-xs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                            "Summary"
                            span class="text-red-400 ml-1" { "*" }
                        }
                        input type="text"
                            name="summary"
                            required
                            placeholder="What needs to be done?"
                            autofocus
                            class="w-full bg-gray-950/50 border border-gray-800/80 rounded-xl px-4 py-3 text-sm text-gray-100 placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all duration-200" {}
                    }

                    // ─── DESCRIPTION ───
                    div class="space-y-2" {
                        label class="block text-xs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                            "Description"
                        }
                        textarea name="description"
                            rows="4"
                            placeholder="Describe the issue in detail..."
                            class="w-full bg-gray-950/50 border border-gray-800/80 rounded-xl px-4 py-3 text-sm text-gray-100 placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all duration-200 resize-y" {}
                    }

                    // ─── GRID: TYPE, PRIORITY ───
                    div class="grid grid-cols-2 gap-4" {
                        // Issue Type
                        div class="space-y-1.5" {
                            label class="block text-xxs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                                "Issue Type"
                                span class="text-red-400 ml-1" { "*" }
                            }
                            select name="issue_type"
                                required
                                class="w-full bg-gray-950/50 border border-gray-800/80 rounded-xl px-4 py-3 text-sm text-gray-100 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all duration-200 appearance-none cursor-pointer" {
                                option value="task" selected { "📋 Task" }
                                option value="bug" { "🐛 Bug" }
                                option value="story" { "📖 Story" }
                                option value="epic" { "🚀 Epic" }
                            }
                        }

                        // Priority
                        div class="space-y-1.5" {
                            label class="block text-xxs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                                "Priority"
                                span class="text-red-400 ml-1" { "*" }
                            }
                            select name="priority"
                                required
                                class="w-full bg-gray-950/50 border border-gray-800/80 rounded-xl px-4 py-3 text-sm text-gray-100 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all duration-200 appearance-none cursor-pointer" {
                                option value="1" { "🔴 Highest (P1)" }
                                option value="2" { "🟠 High (P2)" }
                                option value="3" selected { "🟡 Medium (P3)" }
                                option value="4" { "🟢 Low (P4)" }
                                option value="5" { "⚪ Lowest (P5)" }
                            }
                        }
                    }

                    // ─── SPRINT SELECTION (MANDATORY) ───
                    div class="space-y-1.5" {
                        label class="block text-xxs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                            "Sprint"
                            span class="text-red-400 ml-1" { "*" }
                        }
                        select name="sprint_id"
                            required
                            class="w-full bg-gray-950/50 border border-gray-800/80 rounded-xl px-4 py-3 text-sm text-gray-100 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all duration-200 appearance-none cursor-pointer" {
                            @if sprints.is_empty() {
                                option value="" selected { "No sprints available - create one first" }
                            } @else {
                                @for sprint in sprints {
                                    option value=(sprint.id) {
                                        (sprint.name) " (" (sprint.status) ")"
                                    }
                                }
                            }
                        }
                        p class="text-xxs text-gray-500" {
                            "Issues must be assigned to a sprint to appear on the board."
                        }
                    }

                    // ─── ASSIGNEE ───
                    div class="space-y-1.5" {
                        label class="block text-xxs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                            "Assignee"
                        }
                        select name="assignee_id"
                            class="w-full bg-gray-950/50 border border-gray-800/80 rounded-xl px-4 py-3 text-sm text-gray-100 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all duration-200 appearance-none cursor-pointer" {
                            option value="" { "Unassigned" }
                            option value="1" { "Alex Developer" }
                            option value="2" { "Sarah Manager" }
                            option value="3" { "John Admin" }
                        }
                    }

                    // ─── LABELS ───
                    div class="space-y-2" {
                        label class="block text-xs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                            "Labels"
                        }
                        input type="text"
                            name="labels"
                            placeholder="frontend, bugfix, high-impact"
                            class="w-full bg-gray-950/50 border border-gray-800/80 rounded-xl px-4 py-3 text-sm text-gray-100 placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20 transition-all duration-200";
                    }

                    // ─── RESULT CONTAINER (for success/error messages) ───
                    div id="create-issue-result" {}

                    // ─── FOOTER ───
                    div class="flex items-center justify-end gap-3 pt-4 border-t border-gray-800/60 bg-gradient-to-r from-gray-900/50 to-transparent -mx-6 px-6 py-4 flex-shrink-0" {
                        button type="button"
                            onclick="document.getElementById('create-issue-modal').remove()"
                            class="px-5 py-2.5 text-sm font-mono text-gray-400 hover:text-gray-200 hover:bg-gray-800/50 rounded-xl transition-all duration-200" {
                            "Cancel"
                        }
                        button type="submit"
                            class="px-6 py-2.5 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-500 hover:to-indigo-500 text-white font-mono text-sm font-semibold rounded-xl transition-all duration-200 flex items-center gap-2 shadow-lg shadow-blue-900/50 hover:shadow-xl hover:shadow-blue-900/60 hover:scale-[1.02] active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed"
                            id="create-submit-btn" {
                            span class="inline" { "Create Issue" }
                            span class="htmx-indicator inline animate-spin" { "⟳" }
                        }
                    }
                }
            }
        }

        // ─── JAVASCRIPT HANDLER ───
        script {
            (maud::PreEscaped(r#"
                function handleCreateIssueResponse(event) {
                    const response = event.detail.xhr;
                    const resultContainer = document.getElementById('create-issue-result');
                    
                    if (response.status >= 200 && response.status < 300) {
                        // Success - show success message and close after delay
                        resultContainer.innerHTML = `
                            <div class="bg-green-950/30 border border-green-800/60 rounded-lg p-4 text-center">
                                <div class="text-green-400 text-2xl mb-2">✅</div>
                                <p class="text-green-300 text-sm">Issue created successfully!</p>
                            </div>
                        `;
                        setTimeout(function() {
                            const modal = document.getElementById('create-issue-modal');
                            if (modal) modal.remove();
                            // Redirect to backlog
                            htmx.ajax('GET', '/jira/backlog', {
                                target: '#main-content',
                                swap: 'innerHTML',
                                pushUrl: true
                            });
                        }, 1500);
                    } else {
                        // Error - show error message in the modal
                        const errorDiv = document.getElementById('create-issue-error');
                        const errorMsg = document.getElementById('create-issue-error-message');
                        if (errorDiv && errorMsg) {
                            try {
                                const data = JSON.parse(response.responseText);
                                errorMsg.textContent = data.message || 'An error occurred';
                            } catch {
                                errorMsg.textContent = response.responseText || 'An error occurred';
                            }
                            errorDiv.classList.remove('hidden');
                        }
                        // Re-enable the submit button
                        document.getElementById('create-submit-btn').disabled = false;
                    }
                }
            "#))
        }

        // ─── STYLES ───
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
                .animate-fade-in {
                    animation: fade-in 0.2s ease-out;
                }
                .animate-slide-up {
                    animation: slide-up 0.25s ease-out;
                }
                .htmx-request #create-submit-btn span.inline {
                    display: none;
                }
                .htmx-request #create-submit-btn span.htmx-indicator {
                    display: inline !important;
                }
            "#))
        }
    }
}
