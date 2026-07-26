use gritshield::{http::Response, routing::RequestContext};
use maud::{DOCTYPE, Markup, html};

use crate::web::views::{auth_view::login_page_view, projects_view::project_selector};

/// Master Shell Layout for GritJira / GritAdmin
pub fn shell(ctx: RequestContext, title: &str, content: Markup, is_htmx: bool) -> Response {
    if is_htmx {
        return Response::ok(content);
    }

    let side_bar = html! {
        aside class="w-64 bg-gray-900/60 border-r border-gray-800/80 flex flex-col flex-shrink-0" {
            // Brand Header
            div class="p-4 border-b border-gray-800/80 flex items-center justify-between" {
                div class="flex items-center space-x-2" {
                    span class="text-xl" { "⚡" }
                    span class="font-bold text-sm tracking-wide text-white font-mono" { "GritJira" }
                }
                span class="text-xxs bg-blue-950 text-blue-400 border border-blue-800/60 font-mono px-2 py-0.5 rounded" { "v0.1.0" }
            }

            // Project Selector (moved to top of nav)
            div class="p-3 border-b border-gray-800/80" {
                div class="text-xxs text-gray-500 uppercase tracking-wider mb-2" { "Project" }
                div id="project-selector-container"
                    hx-get="/jira/project-selector"
                    hx-trigger="load"
                    hx-swap="innerHTML" {
                    div class="text-gray-500 text-xs animate-pulse" { "Loading projects..." }
                }
            }

            // Navigation Items
            nav class="flex-1 p-3 space-y-1 overflow-y-auto font-mono text-xs" {


                a href="/jira/board"
                    hx-get="/jira/board"
                    hx-target="#main-content"
                    hx-indicator="body"
                    hx-push-url="true"
                    class="flex items-center gap-2.5 p-2 rounded-lg text-gray-300 hover:bg-gray-800/70 hover:text-white transition" {
                    "📋 Sprint Board"
                }

                a href="/jira/backlog"
                    hx-get="/jira/backlog"
                    hx-target="#main-content"
                    hx-indicator="body"
                    hx-push-url="true"
                    class="flex items-center gap-2.5 p-2 rounded-lg text-gray-300 hover:bg-gray-800/70 hover:text-white transition" {
                    "📦 Backlog"
                }

                a href="/jira/projects"
                    hx-get="/jira/projects"
                    hx-target="#main-content"
                    hx-push-url="true"
                    class="flex items-center gap-2.5 p-2 rounded-lg text-gray-300 hover:bg-gray-800/70 hover:text-white transition" {
                        "📁 Projects"
                    }

                a href="/jira/search"
                    hx-get="/jira/search"
                    hx-target="#main-content"
                    hx-push-url="true"
                    class="flex items-center gap-2.5 p-2 rounded-lg text-gray-300 hover:bg-gray-800/70 hover:text-white transition" {
                        "🔍 Search"
                    }
            }

            // Footer Action
            div class="p-3 border-t border-gray-800/80" {
                button
                    hx-get="/jira/issues/new-modal"
                    hx-target="#modals-container"
                    // hx-swap="innerHTML"
                    hx-swap="outerHTML"
                    class="w-full bg-blue-600 hover:bg-blue-500 text-white font-mono font-semibold text-xs py-2 px-3 rounded-lg transition duration-150 flex items-center justify-center gap-2 shadow-lg shadow-blue-950/50" {
                    span { "+" }
                    span { "Create Issue" }
                }
            }
        }

        // --- MAIN VIEWPORT ---
        main class="flex-1 overflow-y-auto flex flex-col min-w-0 bg-gray-950" {
            div id="main-content" class="flex-1 flex flex-col min-h-0" {
                (content)
            }
        }
    };

    let shell = html! {
        (DOCTYPE)
        html lang="en" class="h-full bg-gray-950 text-gray-100" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) " | GritJira" }

                // --- Core Libraries ---
                script src="https://unpkg.com/htmx.org@1.9.10" {}
                script src="https://cdn.tailwindcss.com" {}

                // --- Kanban Drag & Drop Engine ---
                script src="https://cdn.jsdelivr.net/npm/sortablejs@1.15.2/Sortable.min.js" {}

                // --- Graphviz Topology Renderers ---
                script src="https://cdnjs.cloudflare.com/ajax/libs/viz.js/2.1.2/viz.js" {}
                script src="https://cdnjs.cloudflare.com/ajax/libs/viz.js/2.1.2/full.render.js" {}
                script src="https://cdn.jsdelivr.net/npm/svg-pan-zoom@3.6.1/dist/svg-pan-zoom.min.js" {}
                script src="https://unpkg.com/htmx.org@1.9.10/dist/ext/json-enc.js" {}
                script src="https://unpkg.com/htmx.org@1.9.10/dist/ext/client-side-templates.js" {}

                // --- SortableJS + HTMX Kanban Binding Script ---
                script {
                (maud::PreEscaped(r#"
                    document.addEventListener("DOMContentLoaded", function () {
                        initKanbanSortables();
                    });

                    document.body.addEventListener("htmx:afterSettle", function () {
                        initKanbanSortables();
                    });

                    function initKanbanSortables() {
                        const columns = document.querySelectorAll('.sortable-column');

                        columns.forEach((column) => {
                            if (column.dataset.sortableInitialized) return;
                            column.dataset.sortableInitialized = "true";

                            Sortable.create(column, {
                                group: 'kanban-board',
                                animation: 150,
                                ghostClass: 'opacity-40',
                                dragClass: 'shadow-2xl',
                                
                                onEnd: function (evt) {
                                    const itemEl = evt.item;
                                    const targetColumn = evt.to;
                                    
                                    const issueId = itemEl.dataset.issueId;
                                    const targetStepId = targetColumn.dataset.columnId;

                                    if (!issueId || !targetStepId) return;
                                    
                                    itemEl.style.opacity = '0.5';

                                    htmx.ajax('POST', `/api/v1/board/issues/${issueId}/move`, {
                                        values: { step_id: targetStepId },
                                        target: itemEl,
                                        swap: 'outerHTML',
                                        onError: function() {
                                            // Revert opacity on error
                                            itemEl.style.opacity = '1';
                                        }
                                    });
                                }
                            });
                        });
                    }
                "#))
            }

            // Mustache template for issues search results (outside the script block)
            script id="search-result-template" type="text/x-mustache-template" {
                (maud::PreEscaped(r#"
                    {{#data}}
                    <div class="bg-gray-900 border border-gray-800 rounded-lg p-3 flex justify-between items-center hover:border-gray-700 transition">
                        <div>
                            <span class="text-blue-400 font-bold">{{key}}</span>
                            <span class="text-gray-300 ml-2">{{summary}}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-xxs bg-gray-800 text-gray-400 px-2 py-0.5 rounded">{{status}}</span>
                            <span class="text-xxs text-gray-500">{{issue_type}}</span>
                        </div>
                    </div>
                    {{/data}}
                    {{^data}}
                    <p class="text-gray-500 italic mt-4">No issues found.</p>
                    {{/data}}
                "#))
            }

                style {
                    (maud::PreEscaped(r#"
                        /* Global HTMX Progress Indicator Bar */
                        .htmx-indicator-bar {
                            display: none;
                            position: fixed;
                            top: 0;
                            left: 0;
                            width: 100%;
                            height: 3px;
                            background: linear-gradient(90deg, #3b82f6, #8b5cf6, #ec4899);
                            background-size: 200% 100%;
                            animation: loading-bar-move 1.2s infinite linear;
                            z-index: 9999;
                        }
                        body.htmx-request .htmx-indicator-bar,
                        .htmx-request.htmx-indicator-bar {
                            display: block !important;
                        }
                        .htmx-request.htmx-indicator {
                            display: block !important;
                        }
                        .htmx-indicator {
                            display: none !important;
                        }
                        .htmx-request .htmx-indicator {
                            display: block !important;
                        }
                        /* Already in your shell.rs */
                        .htmx-request #create-project-submit-btn span.inline {
                            display: none;
                        }
                        .htmx-request #create-project-submit-btn span.htmx-indicator {
                            display: inline !important;
                        }
                        @keyframes loading-bar-move {
                            0% { background-position: 100% 0%; }
                            100% { background-position: -100% 0%; }
                        }

                        /* Scrollbar customization for boards */
                        ::-webkit-scrollbar { width: 8px; height: 8px; }
                        ::-webkit-scrollbar-track { background: #090d16; }
                        ::-webkit-scrollbar-thumb { background: #1f2937; border-radius: 4px; }
                        ::-webkit-scrollbar-thumb:hover { background: #374151; }
                    "#))
                }
            }

            body class="h-full font-sans antialiased bg-gray-950 text-gray-100 flex flex-col overflow-hidden" {
                // Top Indicator Line
                div class="htmx-indicator-bar" {}

                @if ctx.is_user_authenticated() {
                    div class="flex h-screen overflow-hidden" {
                        // --- SIDEBAR ---
                        (side_bar)
                    }
                } @else {
                    (login_page_view())
                }

                // Global Containers for Modals & Dynamic Toasts
                div id="modals-container" class="z-50" {}
                div id="toast-container" class="fixed bottom-4 right-4 z-50 space-y-2 pointer-events-none" {}
            }
        }
    };

    Response::ok(shell)
}
