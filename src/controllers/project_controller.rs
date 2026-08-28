use gritshield::GritSanitizer;
use gritshield::{http::response::HttpStatus, prelude::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::repositories::workflow::WorkflowRepository;
use crate::security::caps::{ProjectAdmin, ViewBoard};
use crate::services::project_service::ProjectService;

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

#[derive(Deserialize, GritSanitizer)]
pub struct CreateWorkflowStepPayload {
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Deserialize, GritSanitizer)]
pub struct UpdateWorkflowStepPayload {
    #[serde(default)]
    pub name: Option<String>,
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
        if payload.key.is_empty()
            || !payload
                .key
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
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
            .update_project(
                project_id,
                payload.name.as_deref(),
                payload.description.as_deref(),
            )
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
            .and_then(|v| v.first())
            .map(|s| s.to_string())
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

    // ======================================================
    // Workflow API (per project)
    // ======================================================

    /// GET /api/v1/projects/:id/workflow - List workflow steps (ordered by position)
    #[get("/:id/workflow")]
    #[cap(ViewBoard)]
    pub async fn get_workflow(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        match workflow_repo.find_steps_by_project(project_id).await {
            Ok(steps) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: steps,
                },
            ),
            Err(e) => Response::internal_error(format!("Failed to fetch workflow: {}", e)),
        }
    }

    /// POST /api/v1/projects/:id/workflow/steps - Add a workflow step (appended)
    #[post("/:id/workflow/steps")]
    #[cap(ProjectAdmin)]
    pub async fn add_workflow_step(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let payload = match ctx.json::<CreateWorkflowStepPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid workflow step payload"),
        };

        let name = payload.name.as_deref().unwrap_or("New Step");

        match workflow_repo.create_step_with_name(project_id, name).await {
            Ok(step) => Response::json(
                HttpStatus::Created,
                &ApiResponse {
                    success: true,
                    data: step,
                },
            ),
            Err(e) => Response::bad_request(format!("Failed to add step: {}", e)),
        }
    }

    /// POST /api/v1/projects/:id/workflow/default - Seed the default workflow
    /// columns (To Do / In Progress / In Review / Done) if the project has none.
    #[post("/:id/workflow/default")]
    #[cap(ProjectAdmin)]
    pub async fn ensure_default_workflow(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
        project_service: Arc<ProjectService>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let existing = match workflow_repo.find_steps_by_project(project_id).await {
            Ok(steps) => steps,
            Err(e) => return Response::internal_error(format!("Failed to fetch workflow: {}", e)),
        };

        if !existing.is_empty() {
            return Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: existing,
                },
            );
        }

        match project_service
            .create_default_workflow_steps(project_id)
            .await
        {
            Ok(()) => match workflow_repo.find_steps_by_project(project_id).await {
                Ok(steps) => Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: steps,
                    },
                ),
                Err(e) => Response::internal_error(format!("Failed to fetch workflow: {}", e)),
            },
            Err(e) => Response::bad_request(format!("Failed to create default workflow: {}", e)),
        }
    }

    /// PATCH /api/v1/projects/:project_id/workflow/:step_id - Rename a workflow step
    #[patch("/:project_id/workflow/:step_id")]
    #[cap(ProjectAdmin)]
    pub async fn update_workflow_step(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
    ) -> Response {
        let step_id: i32 = match ctx.params.get("step_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid step ID"),
        };

        let payload = match ctx.json::<UpdateWorkflowStepPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid workflow step payload"),
        };

        match workflow_repo.update_step(step_id, payload.name.as_deref()).await {
            Ok(Some(step)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: step,
                },
            ),
            Ok(None) => Response::not_found("Step not found"),
            Err(e) => Response::bad_request(format!("Failed to update step: {}", e)),
        }
    }

    /// POST /api/v1/projects/:project_id/workflow/:step_id/toggle - Toggle
    /// the "completed" flag of a workflow step
    #[post("/:project_id/workflow/:step_id/toggle")]
    #[cap(ProjectAdmin)]
    pub async fn toggle_workflow_step(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
    ) -> Response {
        let step_id: i32 = match ctx.params.get("step_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid step ID"),
        };

        match workflow_repo.toggle_completed(step_id).await {
            Ok(Some(step)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: step,
                },
            ),
            Ok(None) => Response::not_found("Step not found"),
            Err(e) => Response::bad_request(format!("Failed to toggle step: {}", e)),
        }
    }

    /// DELETE /api/v1/projects/:project_id/workflow/:step_id - Delete a workflow step
    #[delete("/:project_id/workflow/:step_id")]
    #[cap(ProjectAdmin)]
    pub async fn delete_workflow_step(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
    ) -> Response {
        let step_id: i32 = match ctx.params.get("step_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid step ID"),
        };

        match workflow_repo.delete_step(step_id).await {
            Ok(true) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: "Workflow step deleted",
                },
            ),
            Ok(false) => Response::not_found("Step not found"),
            Err(e) => Response::bad_request(format!("Failed to delete step: {}", e)),
        }
    }
}