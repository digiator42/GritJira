// src/web/components/notification.rs
use maud::{Markup, html};

/// Notification bell icon with badge
pub fn notification_bell(count: i32) -> Markup {
    html! {
        div class="relative cursor-pointer group" {
            button class="p-2 text-gray-400 hover:text-white hover:bg-gray-800/50 rounded-lg transition-colors" {
                span class="text-xl" { "🔔" }
            }
            @if count > 0 {
                div class="absolute -top-1 -right-1 w-5 h-5 bg-gradient-to-r from-red-500 to-rose-500 rounded-full flex items-center justify-center text-xs font-bold text-white shadow-lg shadow-red-500/30" {
                    (count)
                }
            }
            // Tooltip
            div class="absolute top-full right-0 mt-2 w-64 bg-gray-900/95 backdrop-blur-xl border border-gray-800/80 rounded-xl shadow-2xl opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200 z-50" {
                div class="p-4" {
                    div class="flex items-center justify-between mb-3" {
                        h3 class="text-sm font-bold text-white" { "Notifications" }
                        a href="#" class="text-xs text-blue-400 hover:underline" { "Mark all read" }
                    }
                    div class="space-y-2" {
                        div class="flex items-start gap-3 p-2 bg-gray-950/50 rounded-lg border border-gray-800/50 hover:bg-gray-800/50 transition-colors cursor-pointer" {
                            div class="w-8 h-8 bg-blue-500/20 rounded-full flex items-center justify-center flex-shrink-0" {
                                span class="text-blue-400 text-sm" { "📝" }
                            }
                            div class="flex-1 min-w-0" {
                                p class="text-sm text-gray-200" { "New issue assigned to you" }
                                p class="text-xxs text-gray-500 mt-1" { "2 minutes ago" }
                            }
                        }
                        div class="flex items-start gap-3 p-2 bg-gray-950/50 rounded-lg border border-gray-800/50 hover:bg-gray-800/50 transition-colors cursor-pointer" {
                            div class="w-8 h-8 bg-emerald-500/20 rounded-full flex items-center justify-center flex-shrink-0" {
                                span class="text-emerald-400 text-sm" { "✅" }
                            }
                            div class="flex-1 min-w-0" {
                                p class="text-sm text-gray-200" { "Sprint completed successfully" }
                                p class="text-xxs text-gray-500 mt-1" { "1 hour ago" }
                            }
                        }
                        div class="flex items-start gap-3 p-2 bg-gray-950/50 rounded-lg border border-gray-800/50 hover:bg-gray-800/50 transition-colors cursor-pointer" {
                            div class="w-8 h-8 bg-amber-500/20 rounded-full flex items-center justify-center flex-shrink-0" {
                                span class="text-amber-400 text-sm" { "💬" }
                            }
                            div class="flex-1 min-w-0" {
                                p class="text-sm text-gray-200" { "New comment on PROJ-42" }
                                p class="text-xxs text-gray-500 mt-1" { "3 hours ago" }
                            }
                        }
                    }
                    div class="mt-3 pt-3 border-t border-gray-800/50 text-center" {
                        a href="#" class="text-xs text-blue-400 hover:underline" { "View all notifications" }
                    }
                }
            }
        }
    }
}

/// Toast notification container
pub fn toast_container() -> Markup {
    html! {
        div id="toast-container" class="fixed top-4 right-4 z-50 space-y-2" {}
    }
}

/// Success toast notification
pub fn success_toast(message: &str) -> Markup {
    html! {
        div class="flex items-center gap-3 bg-gray-900/95 backdrop-blur-xl border border-emerald-500/30 rounded-xl p-4 shadow-2xl shadow-emerald-900/20 animate-slide-in" {
            div class="w-8 h-8 bg-emerald-500/20 rounded-full flex items-center justify-center flex-shrink-0" {
                span class="text-emerald-400" { "✅" }
            }
            div class="flex-1" {
                p class="text-sm text-white font-medium" { (message) }
            }
            button class="text-gray-400 hover:text-white transition-colors" {
                span { "✕" }
            }
        }
    }
}

/// Error toast notification
pub fn error_toast(message: &str) -> Markup {
    html! {
        div class="flex items-center gap-3 bg-gray-900/95 backdrop-blur-xl border border-red-500/30 rounded-xl p-4 shadow-2xl shadow-red-900/20 animate-slide-in" {
            div class="w-8 h-8 bg-red-500/20 rounded-full flex items-center justify-center flex-shrink-0" {
                span class="text-red-400" { "❌" }
            }
            div class="flex-1" {
                p class="text-sm text-white font-medium" { (message) }
            }
            button class="text-gray-400 hover:text-white transition-colors" {
                span { "✕" }
            }
        }
    }
}

/// Info toast notification
pub fn info_toast(message: &str) -> Markup {
    html! {
        div class="flex items-center gap-3 bg-gray-900/95 backdrop-blur-xl border border-blue-500/30 rounded-xl p-4 shadow-2xl shadow-blue-900/20 animate-slide-in" {
            div class="w-8 h-8 bg-blue-500/20 rounded-full flex items-center justify-center flex-shrink-0" {
                span class="text-blue-400" { "ℹ️" }
            }
            div class="flex-1" {
                p class="text-sm text-white font-medium" { (message) }
            }
            button class="text-gray-400 hover:text-white transition-colors" {
                span { "✕" }
            }
        }
    }
}

/// Warning toast notification
pub fn warning_toast(message: &str) -> Markup {
    html! {
        div class="flex items-center gap-3 bg-gray-900/95 backdrop-blur-xl border border-amber-500/30 rounded-xl p-4 shadow-2xl shadow-amber-900/20 animate-slide-in" {
            div class="w-8 h-8 bg-amber-500/20 rounded-full flex items-center justify-center flex-shrink-0" {
                span class="text-amber-400" { "⚠️" }
            }
            div class="flex-1" {
                p class="text-sm text-white font-medium" { (message) }
            }
            button class="text-gray-400 hover:text-white transition-colors" {
                span { "✕" }
            }
        }
    }
}
