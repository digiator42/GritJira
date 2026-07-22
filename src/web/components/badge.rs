use maud::{Markup, html};

pub enum IssuePriority {
    High,
    Medium,
    Low,
}

pub fn priority_badge(priority: &str) -> Markup {
    let (bg, label) = match priority.to_lowercase().as_str() {
        "high" | "p1" => ("bg-red-950/60 border-red-800 text-red-400", "P1 - High"),
        "medium" | "p2" => ("bg-amber-950/60 border-amber-800 text-amber-400", "P2 - Med"),
        _ => ("bg-blue-950/60 border-blue-800 text-blue-400", "P3 - Low"),
    };

    html! {
        span class={"px-2 py-0.5 text-xxs font-mono rounded border " (bg)} {
            (label)
        }
    }
}