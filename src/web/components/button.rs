// src/web/components/button.rs
use maud::{Markup, html};

/// Primary action button with gradient effect
pub fn primary_button(text: &str, icon: Option<&str>) -> Markup {
    html! {
        button class="group relative inline-flex items-center gap-2 px-4 py-2.5 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-500 hover:to-indigo-500 text-white font-semibold text-sm rounded-lg shadow-lg shadow-blue-900/50 transition-all duration-200 hover:shadow-xl hover:shadow-blue-900/60 hover:scale-[1.02] active:scale-[0.98]" {
            @if let Some(icon) = icon {
                span class="text-lg group-hover:rotate-12 transition-transform" { (icon) }
            }
            span { (text) }
            // Subtle shine effect
            div class="absolute inset-0 rounded-lg bg-gradient-to-r from-transparent via-white/10 to-transparent -translate-x-full group-hover:translate-x-full transition-transform duration-700" {}
        }
    }
}

/// Secondary action button
pub fn secondary_button(text: &str, icon: Option<&str>) -> Markup {
    html! {
        button class="inline-flex items-center gap-2 px-4 py-2.5 bg-gray-800/80 hover:bg-gray-700/80 border border-gray-700 text-gray-200 font-medium text-sm rounded-lg transition-all duration-200 hover:border-gray-600 hover:shadow-lg active:scale-[0.98]" {
            @if let Some(icon) = icon {
                span class="text-lg" { (icon) }
            }
            span { (text) }
        }
    }
}

/// Success button
pub fn success_button(text: &str, icon: Option<&str>) -> Markup {
    html! {
        button class="inline-flex items-center gap-2 px-4 py-2.5 bg-gradient-to-r from-emerald-600 to-green-600 hover:from-emerald-500 hover:to-green-500 text-white font-semibold text-sm rounded-lg shadow-lg shadow-emerald-900/50 transition-all duration-200 hover:shadow-xl hover:shadow-emerald-900/60 hover:scale-[1.02] active:scale-[0.98]" {
            @if let Some(icon) = icon {
                span class="text-lg" { (icon) }
            }
            span { (text) }
        }
    }
}

/// Danger button
pub fn danger_button(text: &str, icon: Option<&str>) -> Markup {
    html! {
        button class="inline-flex items-center gap-2 px-4 py-2.5 bg-gradient-to-r from-red-600 to-rose-600 hover:from-red-500 hover:to-rose-500 text-white font-semibold text-sm rounded-lg shadow-lg shadow-red-900/50 transition-all duration-200 hover:shadow-xl hover:shadow-red-900/60 hover:scale-[1.02] active:scale-[0.98]" {
            @if let Some(icon) = icon {
                span class="text-lg" { (icon) }
            }
            span { (text) }
        }
    }
}

/// Ghost button (minimal)
pub fn ghost_button(text: &str, icon: Option<&str>) -> Markup {
    html! {
        button class="inline-flex items-center gap-2 px-4 py-2.5 text-gray-400 hover:text-white hover:bg-gray-800/50 font-medium text-sm rounded-lg transition-all duration-200 active:scale-[0.98]" {
            @if let Some(icon) = icon {
                span class="text-lg" { (icon) }
            }
            span { (text) }
        }
    }
}

/// Icon-only button
pub fn icon_button(icon: &str, tooltip: Option<&str>) -> Markup {
    html! {
        button title=(tooltip.unwrap_or("")) class="p-2 text-gray-400 hover:text-white hover:bg-gray-800/50 rounded-lg transition-all duration-200 active:scale-[0.95]" {
            span class="text-lg" { (icon) }
        }
    }
}
