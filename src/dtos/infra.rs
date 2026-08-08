use serde::{Deserialize, Deserializer};
use serde_aux::field_attributes::{
    deserialize_number_from_string,
    deserialize_option_number_from_string,
};
use gritshield::GritSanitizer;

#[derive(Deserialize, GritSanitizer, Debug)]
pub struct CreateIssuePayload {
    #[clean(trim, html_escape)]
    pub summary: String,

    #[clean(trim, html_escape)]
    pub description: Option<String>,

    #[clean(trim, lowercase)]
    pub issue_type: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub priority: i32,

    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub sprint_id: Option<i32>,

    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub story_points: Option<i32>,

    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub assignee_id: Option<i32>,

    #[clean(trim)]
    #[serde(default)]
    pub labels: Option<String>,

    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub project_id: Option<i32>,
}

#[derive(Deserialize, GritSanitizer)]
pub struct AddCommentPayload {
    #[clean(trim, html_escape)]
    pub body: String,
}

#[derive(Deserialize, GritSanitizer)]
pub struct CreateSprintPayload {
    #[clean(trim, html_escape)]
    pub name: String,

    #[clean(trim, html_escape)]
    pub goal: Option<String>,
}

#[derive(Deserialize, GritSanitizer)]
pub struct UpdateSprintPayload {
    #[clean(trim, html_escape)]
    pub name: Option<String>,

    #[clean(trim, html_escape)]
    pub goal: Option<String>,
}

#[derive(Deserialize, GritSanitizer)]
pub struct MoveIssuePayload {
    pub target_step_id: i32,
}

// ============================================================
// 3. Issue Update Payload
// ============================================================
#[derive(Deserialize, GritSanitizer)]
pub struct UpdateIssuePayload {
    #[clean(trim, html_escape)]
    pub summary: Option<String>,

    #[clean(trim, html_escape)]
    pub description: Option<String>,

    #[serde(default)]
    pub priority: Option<i32>,

    #[clean(trim, lowercase)]
    #[serde(default)]
    pub issue_type: Option<String>,

    #[serde(default)]
    pub story_points: Option<i32>,
}
