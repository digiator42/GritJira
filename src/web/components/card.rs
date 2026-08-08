// src/web/components/card.rs
use maud::{Markup, html};

/// Enhanced card component with hover effects
pub fn card(content: Markup, extra_classes: Option<&str>) -> Markup {
    let classes = format!(
        "bg-gray-900/80 backdrop-blur-sm border border-gray-800/80 rounded-xl p-4 hover:border-gray-700/80 hover:shadow-xl hover:shadow-gray-900/20 transition-all duration-300 {}",
        extra_classes.unwrap_or("")
    );
    
    html! {
        div class=(classes) {
            (content)
        }
    }
}

/// Metric card for dashboard
pub fn metric_card(title: &str, value: &str, change: Option<&str>, icon: &str, color: &str) -> Markup {
    let color_classes = match color {
        "blue" => "from-blue-500/20 to-indigo-500/20 border-blue-500/30",
        "green" => "from-emerald-500/20 to-green-500/20 border-emerald-500/30",
        "purple" => "from-purple-500/20 to-pink-500/20 border-purple-500/30",
        "orange" => "from-orange-500/20 to-amber-500/20 border-orange-500/30",
        _ => "from-gray-500/20 to-slate-500/20 border-gray-500/30",
    };
    
    html! {
        div class="relative overflow-hidden bg-gradient-to-br rounded-xl border p-5 hover:scale-[1.02] transition-transform duration-300" {
            div class=(format!("absolute inset-0 bg-gradient-to-br {}", color_classes)) {}
            div class="relative z-10" {
                div class="flex items-center justify-between" {
                    div class="flex items-center gap-3" {
                        span class="text-2xl" { (icon) }
                        div {
                            p class="text-gray-400 text-xs font-medium uppercase tracking-wider" { (title) }
                            p class="text-2xl font-bold text-white mt-1" { (value) }
                        }
                    }
                    @if let Some(change) = change {
                        span class={"text-xs font-medium " (if change.starts_with('+') { "text-emerald-400" } else { "text-red-400" })} {
                            (change)
                        }
                    }
                }
            }
        }
    }
}

/// Status card for sprint/project status
pub fn status_card(title: &str, status: &str, description: &str, icon: &str) -> Markup {
    let (status_bg, status_text) = match status.to_lowercase().as_str() {
        "active" | "in progress" => ("bg-emerald-500/20 text-emerald-400 border-emerald-500/30", "Active"),
        "completed" | "done" => ("bg-blue-500/20 text-blue-400 border-blue-500/30", "Completed"),
        "planning" => ("bg-amber-500/20 text-amber-400 border-amber-500/30", "Planning"),
        "blocked" => ("bg-red-500/20 text-red-400 border-red-500/30", "Blocked"),
        _ => ("bg-gray-500/20 text-gray-400 border-gray-500/30", status),
    };
    
    html! {
        div class="bg-gray-900/80 backdrop-blur-sm border border-gray-800/80 rounded-xl p-5 hover:border-gray-700/80 transition-all duration-300" {
            div class="flex items-start justify-between" {
                div class="flex items-center gap-3" {
                    div class="p-2 bg-gray-800/50 rounded-lg" {
                        span class="text-xl" { (icon) }
                    }
                    div {
                        h3 class="text-sm font-semibold text-white" { (title) }
                        p class="text-gray-400 text-xs mt-1 line-clamp-2" { (description) }
                    }
                }
                span class={"px-2 py-1 text-xs font-medium rounded-full border " (status_bg)} {
                    (status_text)
                }
            }
        }
    }
}
