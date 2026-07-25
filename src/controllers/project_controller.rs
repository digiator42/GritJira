use std::sync::Arc;
use gritshield::GritSanitizer;
use serde::{Deserialize, Serialize};
use gritshield::{
    http::response::HttpStatus,
    prelude::*,
};

use crate::services::project_service::ProjectService;
use crate::security::caps::{ProjectAdmin, ViewBoard};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Deserialize, GritSanitizer)]
pub struct CreateProjectPayload {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, GritSanitizer)]
pub struct UpdateProjectPayload {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub struct ProjectController;

#[controller("/api/v1/projects")]
impl ProjectController {
    /// GET /api/v1/projects - List all projects
    #[get("")]
    #[cap(ViewBoard)]
    pub async fn list_projects(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        match project_service.list_projects().await {
            Ok(projects) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: projects,
                },
            ),
            Err(e) => Response::internal_error(format!("Failed to fetch projects: {}", e)),
        }
    }

    /// GET /api/v1/projects/:id - Get project by ID
    #[get("/:id")]
    #[cap(ViewBoard)]
    pub async fn get_project(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        match project_service.get_project_by_id(project_id).await {
            Ok(Some(project)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: project,
                },
            ),
            Ok(None) => Response::not_found("Project not found"),
            Err(e) => Response::internal_error(format!("Failed to fetch project: {}", e)),
        }
    }

    /// POST /api/v1/projects - Create a new project
    #[post("")]
    #[cap(ProjectAdmin)]
    pub async fn create_project(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let payload = match ctx.json::<CreateProjectPayload>().await {
            Ok(p) => p,
            Err(e) => return Response::bad_request(format!("Invalid request body: {:?}", e)),
        };

        // Validate key is not empty and alphanumeric
        if payload.key.is_empty() || !payload.key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Response::bad_request("Project key must be alphanumeric or contain hyphens");
        }

        match project_service
            .create_project(&payload.key, &payload.name, payload.description.as_deref())
            .await
        {
            Ok(project) => Response::json(
                HttpStatus::Created,
                &ApiResponse {
                    success: true,
                    data: project,
                },
            ),
            Err(e) => {
                if e.to_string().contains("duplicate") {
                    Response::bad_request("Project key already exists")
                } else {
                    Response::internal_error(format!("Failed to create project: {}", e))
                }
            }
        }
    }

    /// PATCH /api/v1/projects/:id - Update project
    #[patch("/:id")]
    #[cap(ProjectAdmin)]
    pub async fn update_project(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let payload = match ctx.json::<UpdateProjectPayload>().await {
            Ok(p) => p,
            Err(e) => return Response::bad_request(format!("Invalid request body: {:?}", e)),
        };

        match project_service
            .update_project(project_id, payload.name.as_deref(), payload.description.as_deref())
            .await
        {
            Ok(project) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: project,
                },
            ),
            Err(e) => {
                if e.to_string().contains("RecordNotFound") {
                    Response::not_found("Project not found")
                } else {
                    Response::internal_error(format!("Failed to update project: {}", e))
                }
            }
        }
    }

    /// DELETE /api/v1/projects/:id - Delete project
    #[delete("/:id")]
    #[cap(ProjectAdmin)]
    pub async fn delete_project(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        match project_service.delete_project(project_id).await {
            Ok(true) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: "Project deleted successfully",
                },
            ),
            Ok(false) => Response::not_found("Project not found"),
            Err(e) => Response::internal_error(format!("Failed to delete project: {}", e)),
        }
    }

    /// GET /api/v1/projects/:id/issues - Get project issues
    #[get("/:id/issues")]
    #[cap(ViewBoard)]
    pub async fn get_project_issues(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        match project_service.get_project_with_issues(project_id).await {
            Ok(Some((project, issues))) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: serde_json::json!({
                        "project": project,
                        "issues": issues
                    }),
                },
            ),
            Ok(None) => Response::not_found("Project not found"),
            Err(e) => Response::internal_error(format!("Failed to fetch project issues: {}", e)),
        }
    }

    /// GET /api/v1/projects/search - Search projects
    #[get("/search")]
    #[cap(ViewBoard)]
    pub async fn search_projects(
        ctx: RequestContext,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let query = ctx
            .query
            .get("q")
            .map(|v| v.to_string())
            .unwrap_or_default();

        if query.is_empty() {
            return Response::bad_request("Search query is required");
        }

        match project_service.search_projects(&query).await {
            Ok(projects) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: projects,
                },
            ),
            Err(e) => Response::internal_error(format!("Failed to search projects: {}", e)),
        }
    }
}