use gritshield::GritSanitizer;
use serde::{Deserialize, Deserializer};

/// Helper function to deserialize stringified numbers (e.g., "3" -> 3)
fn de_int_from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum IntOrString {
        Int(i32),
        Str(String),
    }

    match IntOrString::deserialize(deserializer)? {
        IntOrString::Int(i) => Ok(i),
        IntOrString::Str(s) => s.parse::<i32>().map_err(serde::de::Error::custom),
    }
}

// ============================================================
// 1. Issue Creation Payload
// ============================================================
#[derive(Deserialize, GritSanitizer)]
pub struct CreateIssuePayload {
    #[clean(trim, html_escape)]
    pub summary: String,

    #[clean(trim, html_escape)]
    pub description: String,

    #[clean(trim, lowercase)]
    pub issue_type: String, // e.g., "bug", "task", "story"
    
    #[serde(deserialize_with = "de_int_from_str")]
    pub priority: i32,
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