use gritshield::GritSanitizer;
use gritshield::{http::response::HttpStatus, prelude::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

        let is_htmx = ctx.req.has_header("hx-request");

        match project_service
            .create_project(&payload.key, &payload.name, payload.description.as_deref())
            .await
        {
            Ok(project) => {
                if is_htmx {
                    // For HTMX, return a success message and redirect to projects page
                    let success_html = html! {
                        div class="bg-green-950/30 border border-green-800/60 rounded-lg p-6 text-center" {
                            div class="text-green-400 text-4xl mb-3" { "✅" }
                            h3 class="text-lg font-bold text-white mb-1" { "Project Created!" }
                            p class="text-gray-300 text-sm" {
                                (format!("{} ({}) has been created successfully.", project.name, project.key))
                            }
                            p class="text-gray-500 text-xxs mt-2" {
                                "Default workflow steps have been added automatically."
                            }
                            div class="mt-4 flex items-center justify-center gap-3" {
                                button
                                    hx-get="/jira/projects"
                                    hx-target="#main-content"
                                    hx-swap="innerHTML"
                                    hx-push-url="true"
                                    class="bg-blue-600 hover:bg-blue-500 text-white px-4 py-2 rounded-lg text-xs transition" {
                                    "View All Projects"
                                }
                                button
                                    hx-get={"/jira/board?project_id=" (project.id)}
                                    hx-target="#main-content"
                                    hx-swap="innerHTML"
                                    hx-push-url="true"
                                    class="bg-green-600 hover:bg-green-500 text-white px-4 py-2 rounded-lg text-xs transition" {
                                    "View Board"
                                }
                            }
                        }
                    };
                    Response::ok(success_html.into_string())
                } else {
                    Response::json(
                        HttpStatus::Created,
                        &ApiResponse {
                            success: true,
                            data: project,
                        },
                    )
                }
            }
            Err(e) => {
                if is_htmx {
                    let error_html = html! {
                        div class="bg-red-950/30 border border-red-800/60 rounded-lg p-4 text-center" {
                            div class="text-red-400 text-2xl mb-2" { "❌" }
                            p class="text-red-300 text-sm" { "Failed to create project" }
                            p class="text-gray-400 text-xxs mt-1" { (e.to_string()) }
                            button
                                hx-get="/jira/projects"
                                hx-target="#main-content"
                                hx-swap="innerHTML"
                                class="mt-3 bg-gray-700 hover:bg-gray-600 text-white px-4 py-2 rounded-lg text-xs transition" {
                                "Back to Projects"
                            }
                        }
                    };
                    Response::bad_request(error_html.into_string())
                } else {
                    if e.to_string().contains("duplicate") {
                        Response::bad_request("Project key already exists")
                    } else {
                        Response::internal_error(format!("Failed to create project: {}", e))
                    }
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
}
