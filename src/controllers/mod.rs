use gritshield::routing::RequestContext;

pub mod auth_controller;
pub mod backlog_controller;
pub mod board_controller;
pub mod issue_controller;
pub mod project_controller;
pub mod project_member_controller;
pub mod sprint_controller;
pub mod user_controller;
pub mod activity_controller;
pub mod webhook_controller;
pub mod attachment_controller;

/// Resolve the active project context for a request.
///
/// Priority: `?project_id=` query param (direct navigation) > session
/// `current_project_id` > project 1.
pub fn get_project_context(ctx: &RequestContext) -> i32 {
    if let Some(id) = ctx
        .query
        .get("project_id")
        .and_then(|v| v.first().and_then(|s| s.parse().ok()))
    {
        return id;
    }

    if let Some(id) = ctx
        .get_session_data("current_project_id")
        .and_then(|s| s.parse().ok())
    {
        return id;
    }

    1
}

/// Resolve the active project key from session state.
pub fn get_project_key(ctx: &RequestContext) -> String {
    ctx.get_session_data("current_project_key")
        .unwrap_or_else(|| "DEFAULT".to_string())
}