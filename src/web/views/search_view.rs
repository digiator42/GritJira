// src/web/views/search_view.rs
use maud::{Markup, html};

pub fn search_page() -> Markup {
    html! {
        div class="p-6 space-y-4 font-mono text-xs text-gray-200" {
            div class="flex justify-between items-center border-b border-gray-800 pb-4" {
                h1 class="text-xl font-bold text-white tracking-wide" { "Issue Search (JQL)" }
            }

            div class="flex gap-4" {
                input type="text"
                    id="jql-input"
                    name="jql"
                    placeholder="e.g. project_id = 1 AND priority = 1"
                    class="flex-1 bg-gray-950 border border-gray-800 rounded-lg px-4 py-2 text-sm text-gray-100 placeholder-gray-600 focus:outline-none focus:border-blue-500"
                    hx-get="/api/v1/issues/search"
                    hx-trigger="keyup changed delay:500ms"
                    hx-target="#search-results"
                    hx-indicator="#search-spinner"
                    hx-ext="json-enc,client-side-templates"
                    hx-vals="js:{jql: document.getElementById('jql-input').value}"
                    hx-template="search-result-template"
                    hx-swap="innerHTML";
            }

            div id="search-spinner" class="htmx-indicator" {
                span class="animate-pulse" { "Searching..." }
            }

            div id="search-results" class="mt-4 space-y-2" {
                p class="text-gray-500" { "Start typing to search issues..." }
            }
        }
    }
}
