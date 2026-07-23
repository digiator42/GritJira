use gritshield::event;
use gritshield::GritEvent;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ============================================================
// Event 1: Issue Created
// ============================================================
#[derive(GritEvent, Clone, Serialize, Deserialize)]
pub struct IssueCreated {
    pub issue_id: i32,
    pub key: String,
    pub summary: String,
    pub reporter_id: i32,
}

pub struct IssueCreatedNotifier;

#[event]
impl IssueCreatedNotifier {
    pub async fn handle(&self, event: Arc<IssueCreated>) {
        println!(
            "[EVENT: IssueCreated] New issue '{}' ({}) reported by user ID {}",
            event.key, event.summary, event.reporter_id
        );
    }
}

// ============================================================
// Event 2: Issue Transitioned (Kanban Board Move)
// ============================================================
#[derive(GritEvent, Clone, Serialize, Deserialize)]
pub struct IssueTransitioned {
    pub issue_id: i32,
    pub key: String,
    pub from_step_id: i32,
    pub to_step_id: i32,
    pub actor_id: i32,
}

pub struct AuditLogHandler;
pub struct SprintMetricHandler;

#[event]
impl AuditLogHandler {
    pub async fn handle(&self, event: Arc<IssueTransitioned>) {
        println!(
            "[AUDIT LOG] Issue {} moved from step {} -> step {} by user {}",
            event.key, event.from_step_id, event.to_step_id, event.actor_id
        );
    }
}

#[event]
impl SprintMetricHandler {
    pub async fn handle(&self, event: Arc<IssueTransitioned>) {
        println!(
            "[METRICS] Updating workflow metrics for issue ID {}",
            event.issue_id
        );
    }
}

// ============================================================
// Event 3: Comment Added
// ============================================================
#[derive(GritEvent, Clone, Serialize, Deserialize)]
pub struct CommentAdded {
    pub comment_id: i32,
    pub issue_id: i32,
    pub author_id: i32,
}

pub struct CommentNotificationHandler;

#[event]
impl CommentNotificationHandler {
    pub async fn handle(&self, event: Arc<CommentAdded>) {
        println!(
            "[EVENT: CommentAdded] Comment #{} added on issue ID {} by user {}",
            event.comment_id, event.issue_id, event.author_id
        );
    }
}