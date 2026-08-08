use gritshield::{http::Response, routing::RequestContext};
use maud::{DOCTYPE, Markup, html};

use crate::web::views::{auth_view::login_page_view, projects_view::project_selector};

/// Master Shell Layout for GritJira / GritAdmin
pub fn shell(ctx: RequestContext, title: &str, content: Markup, is_htmx: bool) -> Response {
    if is_htmx {
        return Response::ok(content);
    }

    let side_bar = html! {
        aside class="w-64 bg-gradient-to-b from-gray-900/95 to-gray-950/95 backdrop-blur-xl border-r border-gray-800/60 flex flex-col flex-shrink-0" {
            // Brand Header with enhanced styling
            div class="p-5 border-b border-gray-800/60 flex items-center justify-between bg-gradient-to-r from-blue-900/20 to-transparent" {
                div class="flex items-center space-x-3" {
                    div class="w-8 h-8 bg-gradient-to-br from-blue-500 to-indigo-600 rounded-lg flex items-center justify-center shadow-lg shadow-blue-500/30" {
                        span class="text-white text-sm font-bold" { "⚡" }
                    }
                    div {
                        span class="font-bold text-sm tracking-wide text-white font-mono" { "GritJira" }
                        div class="flex items-center gap-1 mt-0.5" {
                            span class="text-xxs bg-blue-500/20 text-blue-400 border border-blue-500/30 font-mono px-1.5 py-0.5 rounded" { "v0.1.0" }
                            span class="text-xxs text-emerald-400" { "●" }
                        }
                    }
                }
                // Notification Bell
                (crate::web::components::notification::notification_bell(3))
            }

            // Project Selector with enhanced styling
            div class="p-4 border-b border-gray-800/60 bg-gray-950/30" {
                div class="flex items-center justify-between mb-2" {
                    span class="text-xxs text-gray-500 uppercase tracking-wider font-semibold" { "Project" }
                    button 
                        hx-get="/jira/projects/new-modal"
                        hx-target="#modals-container"
                        hx-swap="innerHTML"
                        class="text-gray-400 hover:text-white hover:bg-gray-800/50 p-1 rounded transition-colors" {
                        span class="text-sm" { "+" }
                    }
                }
                div id="project-selector-container"
                    hx-get="/jira/project-selector"
                    hx-trigger="load"
                    hx-swap="innerHTML" {
                    div class="flex items-center gap-2 text-gray-500 text-xs animate-pulse" {
                        (crate::web::components::loading::spinner(Some("sm")))
                        span { "Loading projects..." }
                    }
                }
            }

            // Navigation Items with enhanced styling
            nav class="flex-1 p-3 space-y-1 overflow-y-auto font-mono text-xs" {
                a href="/jira/dashboard"
                    hx-get="/jira/dashboard"
                    hx-target="#main-content"
                    hx-indicator="body"
                    hx-push-url="true"
                    class="group flex items-center gap-3 p-2.5 rounded-lg text-gray-300 hover:bg-gradient-to-r hover:from-blue-900/30 hover:to-transparent hover:text-white transition-all duration-200 border border-transparent hover:border-blue-500/20" {
                    span class="text-lg group-hover:scale-110 transition-transform" { "📊" }
                    span class="font-medium" { "Dashboard" }
                    div class="ml-auto opacity-0 group-hover:opacity-100 transition-opacity" {
                        span class="text-gray-500 text-xxs" { "→" }
                    }
                }

                a href="/jira/board"
                    hx-get="/jira/board"
                    hx-target="#main-content"
                    hx-indicator="body"
                    hx-push-url="true"
                    class="group flex items-center gap-3 p-2.5 rounded-lg text-gray-300 hover:bg-gradient-to-r hover:from-emerald-900/30 hover:to-transparent hover:text-white transition-all duration-200 border border-transparent hover:border-emerald-500/20" {
                    span class="text-lg group-hover:scale-110 transition-transform" { "📋" }
                    span class="font-medium" { "Sprint Board" }
                    div class="ml-auto opacity-0 group-hover:opacity-100 transition-opacity" {
                        span class="text-gray-500 text-xxs" { "→" }
                    }
                }

                a href="/jira/backlog"
                    hx-get="/jira/backlog"
                    hx-target="#main-content"
                    hx-indicator="body"
                    hx-push-url="true"
                    class="group flex items-center gap-3 p-2.5 rounded-lg text-gray-300 hover:bg-gradient-to-r hover:from-purple-900/30 hover:to-transparent hover:text-white transition-all duration-200 border border-transparent hover:border-purple-500/20" {
                    span class="text-lg group-hover:scale-110 transition-transform" { "📦" }
                    span class="font-medium" { "Backlog" }
                    div class="ml-auto opacity-0 group-hover:opacity-100 transition-opacity" {
                        span class="text-gray-500 text-xxs" { "→" }
                    }
                }

                a href="/jira/projects"
                    hx-get="/jira/projects"
                    hx-target="#main-content"
                    hx-push-url="true"
                    class="group flex items-center gap-3 p-2.5 rounded-lg text-gray-300 hover:bg-gradient-to-r hover:from-amber-900/30 hover:to-transparent hover:text-white transition-all duration-200 border border-transparent hover:border-amber-500/20" {
                        span class="text-lg group-hover:scale-110 transition-transform" { "📁" }
                        span class="font-medium" { "Projects" }
                        div class="ml-auto opacity-0 group-hover:opacity-100 transition-opacity" {
                            span class="text-gray-500 text-xxs" { "→" }
                        }
                    }

                a href="/jira/search"
                    hx-get="/jira/search"
                    hx-target="#main-content"
                    hx-push-url="true"
                    class="group flex items-center gap-3 p-2.5 rounded-lg text-gray-300 hover:bg-gradient-to-r hover:from-pink-900/30 hover:to-transparent hover:text-white transition-all duration-200 border border-transparent hover:border-pink-500/20" {
                        span class="text-lg group-hover:scale-110 transition-transform" { "🔍" }
                        span class="font-medium" { "Search" }
                        div class="ml-auto opacity-0 group-hover:opacity-100 transition-opacity" {
                            span class="text-gray-500 text-xxs" { "→" }
                        }
                    }
                
                // Settings divider with styling
                div class="border-t border-gray-800/60 my-3 pt-3" {
                    span class="text-xxs text-gray-600 uppercase tracking-wider font-semibold" { "System" }
                }

                a href="/jira/settings"
                    hx-get="/jira/settings"
                    hx-target="#main-content"
                    hx-push-url="true"
                    class="group flex items-center gap-3 p-2.5 rounded-lg text-gray-400 hover:bg-gradient-to-r hover:from-gray-800/50 hover:to-transparent hover:text-white transition-all duration-200" {
                    span class="text-lg group-hover:scale-110 transition-transform" { "⚙️" }
                    span class="font-medium" { "Settings" }
                    div class="ml-auto opacity-0 group-hover:opacity-100 transition-opacity" {
                        span class="text-gray-500 text-xxs" { "→" }
                    }
                }
            }

            // Footer Action with enhanced styling
            div class="p-4 border-t border-gray-800/60 bg-gradient-to-t from-blue-900/10 to-transparent" {
                button
                    hx-get="/jira/issues/new-modal"
                    hx-target="#modals-container"
                    hx-swap="outerHTML"
                    class="w-full bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-500 hover:to-indigo-500 text-white font-mono font-semibold text-xs py-2.5 px-4 rounded-lg transition-all duration-200 flex items-center justify-center gap-2 shadow-lg shadow-blue-900/50 hover:shadow-xl hover:shadow-blue-900/60 hover:scale-[1.02] active:scale-[0.98]" {
                    span class="text-lg" { "+" }
                    span { "Create Issue" }
                }
            }
        }

        // --- MAIN VIEWPORT ---
        main class="flex-1 overflow-y-auto flex flex-col min-w-0 bg-gradient-to-br from-gray-950 via-gray-900 to-gray-950" {
            div id="main-content" class="flex-1 flex flex-col min-h-0" {
                (content)
            }
        }

        // Toast Container
        (crate::web::components::notification::toast_container())
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
                        initLoadingStates();
                    });

                    document.body.addEventListener("htmx:afterSettle", function () {
                        initKanbanSortables();
                    });

                    document.body.addEventListener("htmx:afterSwap", function () {
                        initKanbanSortables();
                    });

                    // Show loading indicator
                    document.body.addEventListener("htmx:beforeRequest", function(evt) {
                        const target = evt.detail.target;
                        if (target && !target.classList.contains('htmx-indicator')) {
                            const loader = document.createElement('div');
                            loader.className = 'htmx-loading-overlay';
                            loader.innerHTML = '<div class="flex items-center justify-center gap-2 text-gray-400 text-sm"><svg class="animate-spin w-4 h-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg><span>Loading...</span></div>';
                            loader.style.cssText = 'position: absolute; inset: 0; background: rgba(3, 7, 18, 0.8); backdrop-filter: blur(4px); display: flex; align-items: center; justify-content: center; z-index: 10; border-radius: inherit;';
                            target.style.position = 'relative';
                            target.appendChild(loader);
                        }
                    });

                    // Hide loading indicator
                    document.body.addEventListener("htmx:afterRequest", function(evt) {
                        const target = evt.detail.target;
                        if (target) {
                            const loader = target.querySelector('.htmx-loading-overlay');
                            if (loader) loader.remove();
                        }
                    });

                    function initLoadingStates() {
                        document.querySelectorAll('button, a, input, select').forEach(el => {
                            el.style.transition = 'all 0.2s ease';
                        });
                    }

                    function initKanbanSortables() {
                        const columns = document.querySelectorAll('.sortable-column');

                        columns.forEach((column) => {
                            // Destroy existing sortable instance if it exists
                            if (column._sortable) {
                                column._sortable.destroy();
                                column._sortable = null;
                            }

                            column._sortable = Sortable.create(column, {
                                group: {
                                    name: 'kanban-board',
                                    pull: true,
                                    put: true
                                },
                                animation: 150,
                                ghostClass: 'sortable-ghost',
                                dragClass: 'sortable-drag',
                                chosenClass: 'sortable-chosen',
                                fallbackClass: 'sortable-fallback',
                                forceFallback: false,
                                swapThreshold: 0.65,

                                onStart: function (evt) {
                                    // Prevent click handlers during drag
                                    evt.item.classList.add('is-dragging');
                                    document.body.classList.add('is-dragging');
                                },

                                onEnd: function (evt) {
                                    const itemEl = evt.item;
                                    const targetColumn = evt.to;
                                    const sourceColumn = evt.from;

                                    // Remove drag classes
                                    itemEl.classList.remove('is-dragging');
                                    document.body.classList.remove('is-dragging');

                                    const issueId = itemEl.dataset.issueId;
                                    const targetStepId = targetColumn.dataset.columnId;

                                    // Only make API call if column changed
                                    if (!issueId || !targetStepId || sourceColumn === targetColumn) {
                                        return;
                                    }

                                    console.log('Moving issue', issueId, 'to step', targetStepId);

                                    itemEl.style.opacity = '0.5';
                                    itemEl.style.transform = 'scale(0.95)';

                                    htmx.ajax('POST', `/api/v1/board/issues/${issueId}/move`, {
                                        values: { step_id: targetStepId },
                                        target: itemEl,
                                        swap: 'outerHTML',
                                        onError: function() {
                                            itemEl.style.opacity = '1';
                                            itemEl.style.transform = 'scale(1)';
                                            // Revert the move visually
                                            sourceColumn.appendChild(itemEl);
                                        },
                                        onSuccess: function() {
                                            itemEl.style.opacity = '1';
                                            itemEl.style.transform = 'scale(1)';
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

                        /* Toast animations */
                        @keyframes slide-in {
                            from {
                                transform: translateX(100%);
                                opacity: 0;
                            }
                            to {
                                transform: translateX(0);
                                opacity: 1;
                            }
                        }
                        .animate-slide-in {
                            animation: slide-in 0.3s ease-out;
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

                        /* SortableJS drag feedback styles */
                        .sortable-ghost {
                            opacity: 0.4;
                            background: rgba(59, 130, 246, 0.1);
                            border: 2px dashed #3b82f6;
                            transform: scale(0.95);
                        }

                        .sortable-drag {
                            opacity: 1;
                            transform: scale(1.05) rotate(2deg);
                            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5), 0 10px 10px -5px rgba(0, 0, 0, 0.3);
                            cursor: grabbing !important;
                        }

                        .sortable-chosen {
                            cursor: grabbing !important;
                        }

                        .sortable-fallback {
                            opacity: 0.8;
                            background: #1f2937;
                        }

                        /* Prevent click during drag */
                        .is-dragging {
                            pointer-events: none !important;
                        }

                        /* Drop placeholder visual feedback */
                        .sortable-column.sortable-droppable {
                            background: rgba(59, 130, 246, 0.05);
                            border-color: rgba(59, 130, 246, 0.3);
                        }
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
