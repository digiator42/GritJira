use maud::{Markup, html};

pub fn create_project_modal() -> Markup {
    html! {
        div id="create-project-modal"
            class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm animate-fade-in p-4"
            onclick="if(event.target === this) this.remove()" {

            div class="bg-gray-900 border border-gray-800/60 rounded-xl w-full max-w-md flex flex-col shadow-2xl animate-slide-up overflow-hidden"
                onclick="event.stopPropagation()" {

                // ─── HEADER ───
                div class="flex items-center justify-between px-6 py-4 border-b border-gray-800/60 bg-gray-900/50 flex-shrink-0" {
                    div class="flex items-center gap-3" {
                        div class="w-8 h-8 bg-green-600/20 rounded-lg flex items-center justify-center" {
                            span class="text-green-400 text-sm font-bold" { "+" }
                        }
                        div {
                            h3 class="text-sm font-bold text-gray-100 font-mono tracking-wide" { "Create Project" }
                            p class="text-xxs text-gray-500 font-mono" { "Start a new Jira project" }
                        }
                    }
                    button
                        onclick="document.getElementById('create-project-modal').remove()"
                        class="text-gray-500 hover:text-gray-300 transition p-1 rounded hover:bg-gray-800/50" {
                        span class="text-sm" { "✕" }
                    }
                }

                // ─── FORM ───
                form hx-post="/api/v1/projects"
                     hx-ext="json-enc"
                     hx-target="#main-content"
                     hx-swap="innerHTML"
                     hx-on--after-request="document.getElementById('create-project-modal').remove()"
                     class="flex-1 overflow-y-auto p-6 space-y-4" {

                    // ─── PROJECT KEY ───
                    div class="space-y-1.5" {
                        label class="block text-xxs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                            "Project Key"
                            span class="text-red-400 ml-1" { "*" }
                        }
                        input type="text"
                            name="key"
                            required
                            placeholder="e.g. JIRA, GRIT, PROJ"
                            pattern="[A-Za-z0-9-]+"
                            title="Alphanumeric characters and hyphens only"
                            class="w-full bg-gray-950 border border-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-100 placeholder-gray-600 focus:outline-none focus:border-green-500 focus:ring-1 focus:ring-green-500/30 transition uppercase"
                            autofocus {}
                        div class="text-xxs text-gray-500" {
                            "Alphanumeric characters and hyphens only. Will be automatically uppercased."
                        }
                    }

                    // ─── PROJECT NAME ───
                    div class="space-y-1.5" {
                        label class="block text-xxs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                            "Project Name"
                            span class="text-red-400 ml-1" { "*" }
                        }
                        input type="text"
                            name="name"
                            required
                            placeholder="My Awesome Project"
                            class="w-full bg-gray-950 border border-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-100 placeholder-gray-600 focus:outline-none focus:border-green-500 focus:ring-1 focus:ring-green-500/30 transition" {}
                    }

                    // ─── DESCRIPTION ───
                    div class="space-y-1.5" {
                        label class="block text-xxs font-mono font-semibold uppercase tracking-wider text-gray-400" {
                            "Description"
                        }
                        textarea name="description"
                            rows="3"
                            placeholder="Describe what this project is about..."
                            class="w-full bg-gray-950 border border-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-100 placeholder-gray-600 focus:outline-none focus:border-green-500 focus:ring-1 focus:ring-green-500/30 transition resize-y" {}
                    }

                    // ─── FOOTER ───
                    div class="flex items-center justify-end gap-3 pt-4 border-t border-gray-800/60 bg-gray-900/30 -mx-6 px-6 py-4 flex-shrink-0" {
                        button type="button"
                            onclick="document.getElementById('create-project-modal').remove()"
                            class="px-4 py-2 text-sm font-mono text-gray-400 hover:text-gray-200 hover:bg-gray-800/50 rounded-lg transition" {
                            "Cancel"
                        }
                        button type="submit"
                            class="px-5 py-2 bg-green-600 hover:bg-green-500 text-white font-mono text-sm font-semibold rounded-lg transition flex items-center gap-2 shadow-lg shadow-green-950/50 disabled:opacity-50 disabled:cursor-not-allowed"
                            id="create-project-submit-btn" {
                            span class="inline" { "Create Project" }
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
                .htmx-request #create-project-submit-btn span.inline {
                    display: none;
                }
                .htmx-request #create-project-submit-btn span.htmx-indicator {
                    display: inline !important;
                }
            "#))
        }
    }
}