// src/web/components/loading.rs
use maud::{Markup, html};

/// Spinner for loading states
pub fn spinner(size: Option<&str>) -> Markup {
    let size_class = match size {
        Some("sm") => "w-4 h-4",
        Some("lg") => "w-8 h-8",
        _ => "w-6 h-6",
    };
    
    html! {
        svg class={(format!("animate-spin {}", size_class))} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" {
            circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" {}
            path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" {}
        }
    }
}

/// Skeleton loader for cards
pub fn skeleton_card() -> Markup {
    html! {
        div class="bg-gray-900/80 border border-gray-800/80 rounded-xl p-4 animate-pulse" {
            div class="flex items-center gap-3 mb-3" {
                div class="w-8 h-8 bg-gray-800 rounded-lg" {}
                div class="flex-1 space-y-2" {
                    div class="h-4 bg-gray-800 rounded w-3/4" {}
                    div class="h-3 bg-gray-800 rounded w-1/2" {}
                }
            }
            div class="space-y-2" {
                div class="h-3 bg-gray-800 rounded" {}
                div class="h-3 bg-gray-800 rounded w-5/6" {}
            }
        }
    }
}

/// Skeleton loader for list items
pub fn skeleton_list_item() -> Markup {
    html! {
        div class="flex items-center gap-3 p-3 bg-gray-900/50 border border-gray-800/50 rounded-lg animate-pulse" {
            div class="w-10 h-10 bg-gray-800 rounded-full" {}
            div class="flex-1 space-y-2" {
                div class="h-4 bg-gray-800 rounded w-3/4" {}
                div class="h-3 bg-gray-800 rounded w-1/2" {}
            }
        }
    }
}

/// Full page loading overlay
pub fn page_loader(message: Option<&str>) -> Markup {
    html! {
        div class="fixed inset-0 bg-gray-950/90 backdrop-blur-sm flex items-center justify-center z-50" {
            div class="text-center space-y-4" {
                (spinner(Some("lg")))
                p class="text-gray-400 text-sm font-medium" { (message.unwrap_or("Loading...")) }
            }
        }
    }
}

/// Inline loading indicator
pub fn inline_loader(message: Option<&str>) -> Markup {
    html! {
        div class="flex items-center gap-2 text-gray-400 text-sm" {
            (spinner(Some("sm")))
            span { (message.unwrap_or("Loading...")) }
        }
    }
}
