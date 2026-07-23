use gritshield::GritSanitizer;
use serde::Deserialize;

// ============================================================
// 1. Issue Creation Payload
// ============================================================
#[derive(Deserialize, GritSanitizer)]
pub struct CreateIssuePayload {
    #[clean(trim, html_escape)]
    pub title: String,

    #[clean(trim, html_escape)]
    pub description: String,

    #[clean(trim, lowercase)]
    pub issue_type: String, // e.g., "bug", "task", "story"

    pub priority: i32,
    pub sprint_id: Option<i32>,
}

// ============================================================
// 2. Comment Payload (Handles nested structs cleanly)
// ============================================================
#[derive(Deserialize, GritSanitizer)]
pub struct AddCommentPayload {
    #[clean(trim, html_escape)]
    pub body: String,
}

// ============================================================
// 3. Issue Update Payload
// ============================================================
#[derive(Deserialize, GritSanitizer)]
pub struct UpdateIssuePayload {
    #[clean(trim, html_escape)]
    pub title: Option<String>,

    #[clean(trim, html_escape)]
    pub description: Option<String>,

    #[clean(nested)] // Calls nested struct/vector sanitization in-place
    pub initial_comment: Option<AddCommentPayload>,
}