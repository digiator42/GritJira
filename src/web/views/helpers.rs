use gritshield::routing::RequestContext;

pub fn get_project_context(ctx: &RequestContext) -> i32 {
    // Check query params first (for direct navigation)
    if let Some(id) = ctx.query.get("project_id").and_then(|v| v.first().and_then(|s| s.parse().ok())) {
        return id;
    }

    // Check session
    if let Some(id) = ctx.get_session_data("current_project_id").and_then(|s| s.parse().ok()) {
        return id;
    }

    // Default to 1
    1
}

pub fn get_project_key(ctx: &RequestContext) -> String {
    if let Some(key) = ctx.get_session_data("current_project_key") {
        return key;
    }
    "DEFAULT".to_string()
}