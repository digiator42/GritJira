use std::sync::Arc;
use serde::Serialize;
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;

use crate::dtos::CreateSprintPayload;
use crate::repositories::sprint::SprintRepository;
use crate::security::caps::{ProjectAdmin, ViewBoard};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

pub struct SprintController;

#[controller("/api/v1/sprints")]
impl SprintController {
    /// POST /api/v1/sprints/projects/:project_id - Create new sprint
    #[post("/projects/:project_id")]
    #[cap(ProjectAdmin)]
    pub async fn create_sprint(
        ctx: RequestContext,
        sprint_repo: Arc<SprintRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let payload = match ctx.json::<CreateSprintPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid sprint payload"),
        };

        match sprint_repo.create(project_id, &payload.name, payload.goal).await {
            Ok(sprint) => Response::json(
                HttpStatus::Created,
                &ApiResponse { success: true, data: sprint },
            ),
            Err(e) => Response::bad_request(format!("Failed to create sprint: {}", e)),
        }
    }

    /// POST /api/v1/sprints/:id/start - Activate sprint
    #[post("/:id/start")]
    #[cap(ProjectAdmin)]
    pub async fn start_sprint(
        ctx: RequestContext,
        sprint_repo: Arc<SprintRepository>,
    ) -> Response {
        let sprint_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid sprint ID"),
        };

        match sprint_repo.start_sprint(sprint_id).await {
            Ok(sprint) => Response::json(
                HttpStatus::Ok,
                &ApiResponse { success: true, data: sprint },
            ),
            Err(e) => Response::bad_request(format!("Failed to start sprint: {}", e)),
        }
    }
}