use gritshield::{job, GritJob};
use serde::{Deserialize, Serialize};

// ============================================================
// Job 1: Send Issue Digest / Notifications
// ============================================================
#[derive(Serialize, Deserialize, GritJob)]
pub struct SendIssueDigestJob {
    pub issue_id: i32,
    pub recipient_emails: Vec<String>,
}

#[job(retries = 5)]
impl SendIssueDigestJob {
    pub async fn perform(&self) -> Result<(), String> {
        println!(
            "[JOB: Digest] Dispatching notifications for Issue #{} to {} recipients",
            self.issue_id,
            self.recipient_emails.len()
        );
        Ok(())
    }
}

// ============================================================
// Job 2: Export Project Archive
// ============================================================
#[derive(Serialize, Deserialize, GritJob)]
pub struct ExportProjectArchiveJob {
    pub project_id: i32,
    pub export_format: String, // "json" or "csv"
}

#[job(retries = 2)]
impl ExportProjectArchiveJob {
    pub async fn perform(&self) -> Result<(), String> {
        println!(
            "[JOB: Export] Compiling archive for Project #{} in {} format",
            self.project_id, self.export_format
        );
        Ok(())
    }
}