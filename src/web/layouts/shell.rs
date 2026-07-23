use gritshield::http::Response;
use maud::{html, Markup, DOCTYPE};

/// Master Shell Layout for GritJira / GritAdmin
pub fn shell(title: &str, content: Markup, is_htmx: bool) -> Response {
    if is_htmx {
        return Response::ok(content);
    }

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

                div class="flex h-screen overflow-hidden" {
                    // --- SIDEBAR ---
                    aside class="w-64 bg-gray-900/60 border-r border-gray-800/80 flex flex-col flex-shrink-0" {
                        // Brand Header
                        div class="p-4 border-b border-gray-800/80 flex items-center justify-between" {
                            div class="flex items-center space-x-2" {
                                span class="text-xl" { "⚡" }
                                span class="font-bold text-sm tracking-wide text-white font-mono" { "GritJira" }
                            }
                            span class="text-xxs bg-blue-950 text-blue-400 border border-blue-800/60 font-mono px-2 py-0.5 rounded" { "v0.1.0" }
                        }

                        // Navigation Items
                        nav class="flex-1 p-3 space-y-1 overflow-y-auto font-mono text-xs" {
                            a href="/admin/jira/board"
                               hx-get="/admin/jira/board"
                               hx-target="#main-content"
                               hx-indicator="body"
                               hx-push-url="true"
                               class="flex items-center gap-2.5 p-2 rounded-lg text-gray-300 hover:bg-gray-800/70 hover:text-white transition" {
                                "📋 Sprint Board"
                            }

                            a href="/admin/jira/backlog"
                               hx-get="/admin/jira/backlog"
                               hx-target="#main-content"
                               hx-indicator="body"
                               hx-push-url="true"
                               class="flex items-center gap-2.5 p-2 rounded-lg text-gray-300 hover:bg-gray-800/70 hover:text-white transition" {
                                "📦 Backlog"
                            }

                            a href="/admin/jira/projects"
                               hx-get="/admin/jira/projects"
                               hx-target="#main-content"
                               hx-indicator="body"
                               hx-push-url="true"
                               class="flex items-center gap-2.5 p-2 rounded-lg text-gray-300 hover:bg-gray-800/70 hover:text-white transition" {
                                "📁 Projects"
                            }
                        }

                        // Footer Action
                        div class="p-3 border-t border-gray-800/80" {
                            button
                                hx-get="/admin/jira/issues/new-modal"
                                hx-target="#modals-container"
                                hx-swap="innerHTML"
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
                }

                // Global Containers for Modals & Dynamic Toasts
                div id="modals-container" class="z-50" {}
                div id="toast-container" class="fixed bottom-4 right-4 z-50 space-y-2 pointer-events-none" {}
            }
        }
    };

    Response::ok(shell)
}