use maud::{html, Markup};

pub fn create_issue_modal(project_id: i32, project_key: Option<&str>) -> Markup {
    let project_key_display = project_key.unwrap_or("GRIT");

    html! {
        // ─── MODAL BACKDROP (THIS IS THE ROOT ELEMENT) ───
        div id="create-issue-modal" 
            class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm animate-fade-in p-4"
            onclick="if(event.target === this) this.remove()" {

            // ─── MODAL CONTAINER ───
            div class="bg-gray-900 border border-gray-800/60 rounded-xl w-full max-w-2xl max-h-[90vh] flex flex-col shadow-2xl animate-slide-up overflow-hidden"
                onclick="event.stopPropagation()" {

                // ─── HEADER ───
                div class="flex items-center justify-between px-6 py-4 border-b border-gray-800/60 bg-gray-900/50 flex-shrink-0" {
                    div class="flex items-center gap-3" {
                        div class="w-8 h-8 bg-blue-600/20 rounded-lg flex items-center justify-center" {
                            span class="text-blue-400 text-sm font-bold" { "+" }
                        }
                        div {
                            h3 class="text-sm font-bold text-gray-100 font-mono tracking-wide" { "Create Issue" }
                            p class="text-xxs text-gray-500 font-mono" { (format!("{} Project", project_key_display)) }
                        }
                    }
                    button
                        onclick="document.getElementById('create-issue-modal').remove()"
                        class="text-gray-500 hover:text-gray-300 transition p-1 rounded hover:bg-gray-800/50" {
                        span class="text-sm" { "✕" }
                    }
                }

                // ─── FORM ───
                form hx-post={(format!("/jira/issues/projects/{}/issues/create", project_id))}
                     hx-ext="json-enc"
                     hx-target="#main-content"
                     hx-swap="innerHTML"
                     hx-on--after-request="document.getElementById('create-issue-modal').remove()"
                     class="flex-1 overflow-y-auto p-6 space-y-5" {

                    // ─── HIDDEN FIELDS ───
                    input type="hidden" name="project_id" value=(project_id);

                    // ─── SUMMARY ───
                    div class="space-y-1.5" {
                        label class="block text-xxs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                            "Summary"
                            span class="text-red-400 ml-1" { "*" }
                        }
                        input type="text"
                            name="summary"
                            required
                            placeholder="What needs to be done?"
                            autofocus
                            class="w-full bg-gray-950 border border-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-100 placeholder-gray-600 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/30 transition" {}
                    }

                    // ─── DESCRIPTION ───
                    div class="space-y-1.5" {
                        label class="block text-xxs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                            "Description"
                        }
                        textarea name="description"
                            rows="4"
                            placeholder="Describe the issue in detail..."
                            class="w-full bg-gray-950 border border-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-100 placeholder-gray-600 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/30 transition resize-y" {}
                    }

                    // ─── GRID: TYPE, PRIORITY, ASSIGNEE ───
                    div class="grid grid-cols-3 gap-4" {
                        // Issue Type
                        div class="space-y-1.5" {
                            label class="block text-xxs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                                "Issue Type"
                            }
                            select name="issue_type"
                                class="w-full bg-gray-950 border border-gray-800 rounded-lg px-3 py-2.5 text-sm text-gray-100 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/30 transition appearance-none cursor-pointer" {
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
                            }
                            select name="priority"
                                class="w-full bg-gray-950 border border-gray-800 rounded-lg px-3 py-2.5 text-sm text-gray-100 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/30 transition appearance-none cursor-pointer" {
                                option value="1" { "🔴 Highest (P1)" }
                                option value="2" { "🟠 High (P2)" }
                                option value="3" selected { "🟡 Medium (P3)" }
                                option value="4" { "🟢 Low (P4)" }
                                option value="5" { "⚪ Lowest (P5)" }
                            }
                        }

                        // Assignee
                        div class="space-y-1.5" {
                            label class="block text-xxs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                                "Assignee"
                            }
                            select name="assignee_id"
                                class="w-full bg-gray-950 border border-gray-800 rounded-lg px-3 py-2.5 text-sm text-gray-100 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/30 transition appearance-none cursor-pointer" {
                                option value="" { "Unassigned" }
                                option value="1" { "Alex Developer" }
                                option value="2" { "Sarah Manager" }
                                option value="3" { "John Admin" }
                            }
                        }
                    }

                    // ─── LABELS ───
                    div class="space-y-1.5" {
                        label class="block text-xxs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                            "Labels"
                        }
                        input type="text"
                            name="labels"
                            placeholder="frontend, bugfix, high-impact"
                            class="w-full bg-gray-950 border border-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-100 placeholder-gray-600 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/30 transition";
                    }

                    // ─── FOOTER ───
                    div class="flex items-center justify-end gap-3 pt-4 border-t border-gray-800/60 bg-gray-900/30 -mx-6 px-6 py-4 flex-shrink-0" {
                        button type="button"
                            onclick="document.getElementById('create-issue-modal').remove()"
                            class="px-4 py-2 text-sm font-mono text-gray-400 hover:text-gray-200 hover:bg-gray-800/50 rounded-lg transition" {
                            "Cancel"
                        }
                        button type="submit"
                            class="px-5 py-2 bg-blue-600 hover:bg-blue-500 text-white font-mono text-sm font-semibold rounded-lg transition flex items-center gap-2 shadow-lg shadow-blue-950/50 disabled:opacity-50 disabled:cursor-not-allowed"
                            id="create-submit-btn" {
                            span class="inline" { "Create Issue" }
                            span class="htmx-indicator inline animate-spin" { "⟳" }
                        }
                    }
                }
            }
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