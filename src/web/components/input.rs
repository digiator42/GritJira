// src/web/components/input.rs
use maud::{Markup, html};

/// Text input with enhanced styling
pub fn text_input(name: &str, placeholder: &str, value: Option<&str>, extra_classes: Option<&str>) -> Markup {
    let classes = format!(
        "w-full bg-gray-950/50 border border-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-100 placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/50 transition-all duration-200 {}",
        extra_classes.unwrap_or("")
    );
    
    html! {
        input 
            type="text"
            name=(name)
            placeholder=(placeholder)
            value=[value]
            class=(classes);
    }
}

/// Search input with icon
pub fn search_input(name: &str, placeholder: &str, hx_target: &str, hx_url: &str) -> Markup {
    html! {
        div class="relative" {
            span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" { "🔍" }
            input 
                type="text"
                name=(name)
                placeholder=(placeholder)
                class="w-full bg-gray-950/50 border border-gray-800 rounded-lg pl-10 pr-4 py-2.5 text-sm text-gray-100 placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/50 transition-all duration-200"
                hx-get=(hx_url)
                hx-trigger="keyup changed delay:300ms"
                hx-target=(hx_target)
                hx-indicator=".search-indicator";
            div class="search-indicator absolute right-3 top-1/2 -translate-y-1/2 htmx-indicator" {
                (crate::web::components::loading::spinner(Some("sm")))
            }
        }
    }
}

/// Select dropdown
pub fn select_input(name: &str, options: &[(String, String)], selected: Option<&str>) -> Markup {
    html! {
        select 
            name=(name)
            class="w-full bg-gray-950/50 border border-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-100 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/50 transition-all duration-200 cursor-pointer" {
            @for (value, label) in options {
                option value=(value) selected=(selected.map(|s| s == value).unwrap_or(false)) {
                    (label)
                }
            }
        }
    }
}

/// Textarea
pub fn textarea_input(name: &str, placeholder: &str, rows: Option<i32>) -> Markup {
    html! {
        textarea
            name=(name)
            placeholder=(placeholder)
            rows=(rows.unwrap_or(3))
            class="w-full bg-gray-950/50 border border-gray-800 rounded-lg px-4 py-2.5 text-sm text-gray-100 placeholder-gray-500 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500/50 transition-all duration-200 resize-none" {}
    }
}

/// Checkbox
pub fn checkbox_input(name: &str, label: &str, checked: bool) -> Markup {
    html! {
        label class="flex items-center gap-2 cursor-pointer group" {
            input 
                type="checkbox"
                name=(name)
                checked[checked]
                class="w-4 h-4 rounded border-gray-700 bg-gray-950/50 text-blue-600 focus:ring-blue-500/50 focus:ring-offset-0 cursor-pointer";
            span class="text-sm text-gray-300 group-hover:text-white transition-colors" { (label) }
        }
    }
}

/// Radio button group
pub fn radio_input(name: &str, value: &str, label: &str, checked: bool) -> Markup {
    html! {
        label class="flex items-center gap-2 cursor-pointer group" {
            input 
                type="radio"
                name=(name)
                value=(value)
                checked[checked]
                class="w-4 h-4 border-gray-700 bg-gray-950/50 text-blue-600 focus:ring-blue-500/50 focus:ring-offset-0 cursor-pointer";
            span class="text-sm text-gray-300 group-hover:text-white transition-colors" { (label) }
        }
    }
}
