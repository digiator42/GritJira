use gritshield::database::GritRepository;
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use serde::Serialize;
use std::sync::Arc;

use crate::dtos::{CreateSprintPayload, UpdateSprintPayload};
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
    /// GET /api/v1/sprints/projects/:project_id - List all sprints for a project
    #[get("/projects/:project_id")]
    #[cap(ViewBoard)]
    pub async fn get_sprints(
        ctx: RequestContext,
        sprint_repo: Arc<SprintRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let sprints = sprint_repo
            .query()
            .where_eq(crate::models::sprint::Column::ProjectId, project_id)
            .order_asc(crate::models::sprint::Column::Id)
            .fetch()
            .await
            .unwrap_or_default();

        Response::json(
            HttpStatus::Ok,
            &ApiResponse {
                success: true,
                data: sprints,
            },
        )
    }

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

        match sprint_repo
            .create(project_id, &payload.name, payload.goal)
            .await
        {
            Ok(sprint) => Response::json(
                HttpStatus::Created,
                &ApiResponse {
                    success: true,
                    data: sprint,
                },
            ),
            Err(e) => Response::bad_request(format!("Failed to create sprint: {}", e)),
        }
    }

    /// POST /api/v1/sprints/:id/start - Activate sprint
    #[post("/:id/start")]
    #[cap(ProjectAdmin)]
    pub async fn start_sprint(ctx: RequestContext, sprint_repo: Arc<SprintRepository>) -> Response {
        let sprint_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid sprint ID"),
        };

        match sprint_repo.start_sprint(sprint_id).await {
            Ok(sprint) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: sprint,
                },
            ),
            Err(e) => Response::bad_request(format!("Failed to start sprint: {}", e)),
        }
    }

    /// POST /api/v1/sprints/:id/complete - Complete sprint
    #[post("/:id/complete")]
    #[cap(ProjectAdmin)]
    pub async fn complete_sprint(
        ctx: RequestContext,
        sprint_repo: Arc<SprintRepository>,
    ) -> Response {
        let sprint_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid sprint ID"),
        };

        match sprint_repo.complete_sprint(sprint_id).await {
            Ok(sprint) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: sprint,
                },
            ),
            Err(e) => Response::bad_request(format!("Failed to complete sprint: {}", e)),
        }
    }

    /// POST /api/v1/sprints/:id/reopen - Reactivate a completed sprint
    #[post("/:id/reopen")]
    #[cap(ProjectAdmin)]
    pub async fn reopen_sprint(ctx: RequestContext, sprint_repo: Arc<SprintRepository>) -> Response {
        let sprint_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid sprint ID"),
        };

        match sprint_repo.reopen_sprint(sprint_id).await {
            Ok(sprint) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: sprint,
                },
            ),
            Err(e) => Response::bad_request(format!("Failed to reopen sprint: {}", e)),
        }
    }

    /// DELETE /api/v1/sprints/:id - Delete sprint
    #[delete("/:id")]
    #[cap(ProjectAdmin)]
    pub async fn delete_sprint(ctx: RequestContext, sprint_repo: Arc<SprintRepository>) -> Response {
        let sprint_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid sprint ID"),
        };

        match sprint_repo.delete_sprint(sprint_id).await {
            Ok(true) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: "Sprint deleted successfully",
                },
            ),
            Ok(false) => Response::not_found("Sprint not found"),
            Err(e) => Response::bad_request(format!("Failed to delete sprint: {}", e)),
        }
    }

    /// PATCH /api/v1/sprints/:id - Update sprint
    #[patch("/:id")]
    #[cap(ProjectAdmin)]
    pub async fn update_sprint(
        ctx: RequestContext,
        sprint_repo: Arc<SprintRepository>,
    ) -> Response {
        let sprint_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid sprint ID"),
        };

        let payload = match ctx.json::<UpdateSprintPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid sprint payload"),
        };

        match sprint_repo
            .update_sprint(sprint_id, payload.name.as_deref(), payload.goal.as_deref())
            .await
        {
            Ok(Some(sprint)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: sprint,
                },
            ),
            Ok(None) => Response::not_found("Sprint not found"),
            Err(e) => Response::bad_request(format!("Failed to update sprint: {}", e)),
        }
    }
}