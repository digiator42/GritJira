use gritshield::{job, GritJob};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, GritJob)]
pub struct GenerateSprintBurndownJob {
    pub sprint_id: i32,
    pub project_id: i32,
}

#[job(retries = 3)]
impl GenerateSprintBurndownJob {
    pub async fn perform(&self) -> Result<(), String> {
        println!(
            "[JOB: Burndown] Recalculating burn-down chart for Sprint #{} (Project #{})",
            self.sprint_id, self.project_id
        );
        
        // 1. Fetch active issues and remaining story points from database
        // 2. Persist calculated points into sprint_metrics table

        Ok(())
    }
}