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

        let is_htmx = ctx.req.has_header("hx-request");

        match sprint_repo
            .create(project_id, &payload.name, payload.goal)
            .await
        {
            Ok(sprint) => {
                if is_htmx {
                    // Return HTML for the new sprint card to be appended
                    let sprint_html = html! {
                        div class="bg-gray-900 border border-gray-800 rounded-lg p-4 space-y-2" {
                            div class="flex justify-between items-center border-b border-gray-800 pb-2" {
                                div class="flex items-center gap-3" {
                                    div class="font-bold text-white text-sm" { (sprint.name) }
                                    span class="bg-gray-700 text-gray-300 px-2 py-0.5 rounded text-xxs uppercase" {
                                        "Planning"
                                    }
                                }
                                div class="flex items-center gap-2" {
                                    button
                                        hx-post={"/api/v1/sprints/" (sprint.id) "/start"}
                                        hx-ext="json-enc"
                                        hx-target="closest div"
                                        hx-swap="outerHTML"
                                        class="text-xxs bg-blue-600 hover:bg-blue-500 text-white px-2 py-1 rounded transition" {
                                        "Start Sprint"
                                    }
                                    a href={"/jira/board?project_id=" (project_id) "&sprint_id=" (sprint.id)}
                                        hx-get={"/jira/board?project_id=" (project_id) "&sprint_id=" (sprint.id)}
                                        hx-target="#main-content"
                                        hx-push-url="true"
                                        class="text-xxs text-blue-400 hover:underline" {
                                        "View Board"
                                    }
                                }
                            }
                            p class="text-gray-400 text-xxs" { (sprint.goal.as_deref().unwrap_or("No sprint goal set.")) }
                        }
                    };
                    // Use OOB swap to replace the "No sprints" message if needed
                    Response::ok(sprint_html.into_string())
                } else {
                    Response::json(
                        HttpStatus::Created,
                        &ApiResponse {
                            success: true,
                            data: sprint,
                        },
                    )
                }
            }
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

        let is_htmx = ctx.req.has_header("hx-request");

        match sprint_repo.start_sprint(sprint_id).await {
            Ok(sprint) => {
                if is_htmx {
                    // Return updated sprint HTML
                    let sprint_html = html! {
                        div class="bg-gray-900 border border-gray-800 rounded-lg p-4 space-y-2" {
                            div class="flex justify-between items-center border-b border-gray-800 pb-2" {
                                div class="flex items-center gap-3" {
                                    div class="font-bold text-white text-sm" { (sprint.name) }
                                    span class="bg-emerald-950 text-emerald-400 border border-emerald-800/60 px-2 py-0.5 rounded text-xxs uppercase" {
                                        "Active"
                                    }
                                }
                                div class="flex items-center gap-2" {
                                    a href={"/jira/board?project_id=" (sprint.project_id) "&sprint_id=" (sprint.id)}
                                        hx-get={"/jira/board?project_id=" (sprint.project_id) "&sprint_id=" (sprint.id)}
                                        hx-target="#main-content"
                                        hx-push-url="true"
                                        class="text-xxs text-blue-400 hover:underline" {
                                        "View Board"
                                    }
                                }
                            }
                            p class="text-gray-400 text-xxs" { (sprint.goal.as_deref().unwrap_or("No sprint goal set.")) }
                        }
                    };
                    Response::ok(sprint_html.into_string())
                } else {
                    Response::json(
                        HttpStatus::Ok,
                        &ApiResponse {
                            success: true,
                            data: sprint,
                        },
                    )
                }
            }
            Err(e) => Response::bad_request(format!("Failed to start sprint: {}", e)),
        }
    }

    /// POST /api/v1/sprints/:id/complete - Complete sprint
    #[post("/:id/complete")]
    #[cap(ProjectAdmin)]
    pub async fn complete_sprint(ctx: RequestContext, sprint_repo: Arc<SprintRepository>) -> Response {
        let sprint_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid sprint ID"),
        };

        let is_htmx = ctx.req.has_header("hx-request");

        match sprint_repo.complete_sprint(sprint_id).await {
            Ok(sprint) => {
                if is_htmx {
                    // Return updated sprint HTML
                    let sprint_html = html! {
                        div class="bg-gray-900 border border-gray-800 rounded-lg p-4 space-y-2" {
                            div class="flex justify-between items-center border-b border-gray-800 pb-2" {
                                div class="flex items-center gap-3" {
                                    div class="font-bold text-white text-sm" { (sprint.name) }
                                    span class="bg-gray-700 text-gray-300 px-2 py-0.5 rounded text-xxs uppercase" {
                                        "Completed"
                                    }
                                }
                                div class="flex items-center gap-2" {
                                    a href={"/jira/board?project_id=" (sprint.project_id)}
                                        hx-get={"/jira/board?project_id=" (sprint.project_id)}
                                        hx-target="#main-content"
                                        hx-push-url="true"
                                        class="text-xxs text-blue-400 hover:underline" {
                                        "View Board"
                                    }
                                }
                            }
                            p class="text-gray-400 text-xxs" { (sprint.goal.as_deref().unwrap_or("No sprint goal set.")) }
                        }
                    };
                    Response::ok(sprint_html.into_string())
                } else {
                    Response::json(
                        HttpStatus::Ok,
                        &ApiResponse {
                            success: true,
                            data: sprint,
                        },
                    )
                }
            }
            Err(e) => Response::bad_request(format!("Failed to complete sprint: {}", e)),
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

        let is_htmx = ctx.req.has_header("hx-request");

        match sprint_repo.delete_sprint(sprint_id).await {
            Ok(true) => {
                if is_htmx {
                    Response::ok("")
                } else {
                    Response::json(
                        HttpStatus::Ok,
                        &ApiResponse {
                            success: true,
                            data: "Sprint deleted successfully",
                        },
                    )
                }
            }
            Ok(false) => Response::not_found("Sprint not found"),
            Err(e) => Response::bad_request(format!("Failed to delete sprint: {}", e)),
        }
    }

    /// PATCH /api/v1/sprints/:id - Update sprint
    #[patch("/:id")]
    #[cap(ProjectAdmin)]
    pub async fn update_sprint(ctx: RequestContext, sprint_repo: Arc<SprintRepository>) -> Response {
        let sprint_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid sprint ID"),
        };

        let payload = match ctx.json::<UpdateSprintPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid sprint payload"),
        };

        let is_htmx = ctx.req.has_header("hx-request");

        match sprint_repo
            .update_sprint(sprint_id, payload.name.as_deref(), payload.goal.as_deref())
            .await
        {
            Ok(Some(sprint)) => {
                if is_htmx {
                    // Return updated sprint HTML
                    let sprint_html = html! {
                        div class="bg-gray-900 border border-gray-800 rounded-lg p-4 space-y-2" {
                            div class="flex justify-between items-center border-b border-gray-800 pb-2" {
                                div class="flex items-center gap-3" {
                                    div class="font-bold text-white text-sm" { (sprint.name) }
                                    span class={@if sprint.status == "Active" { "bg-emerald-950 text-emerald-400 border border-emerald-800/60 px-2 py-0.5 rounded text-xxs uppercase" } @else if sprint.status == "Completed" { "bg-gray-700 text-gray-300 px-2 py-0.5 rounded text-xxs uppercase" } @else { "bg-gray-700 text-gray-300 px-2 py-0.5 rounded text-xxs uppercase" } } {
                                        (sprint.status)
                                    }
                                }
                                div class="flex items-center gap-2" {
                                    @if sprint.status == "Planning" {
                                        button
                                            hx-post={"/api/v1/sprints/" (sprint.id) "/start"}
                                            hx-ext="json-enc"
                                            hx-target="closest div"
                                            hx-swap="outerHTML"
                                            class="text-xxs bg-blue-600 hover:bg-blue-500 text-white px-2 py-1 rounded transition" {
                                            "Start Sprint"
                                        }
                                    }
                                    a href={"/jira/board?project_id=" (sprint.project_id) "&sprint_id=" (sprint.id)}
                                        hx-get={"/jira/board?project_id=" (sprint.project_id) "&sprint_id=" (sprint.id)}
                                        hx-target="#main-content"
                                        hx-push-url="true"
                                        class="text-xxs text-blue-400 hover:underline" {
                                        "View Board"
                                    }
                                }
                            }
                            p class="text-gray-400 text-xxs" { (sprint.goal.as_deref().unwrap_or("No sprint goal set.")) }
                        }
                    };
                    Response::ok(sprint_html.into_string())
                } else {
                    Response::json(
                        HttpStatus::Ok,
                        &ApiResponse {
                            success: true,
                            data: sprint,
                        },
                    )
                }
            }
            Ok(None) => Response::not_found("Sprint not found"),
            Err(e) => Response::bad_request(format!("Failed to update sprint: {}", e)),
        }
    }
}
