use std::sync::Arc;
use serde::Deserialize;
use gritshield::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm::EntityTrait;
use crate::repositories::workflow::WorkflowRepository;
use crate::security::caps::ProjectAdmin;

#[derive(Deserialize)]
pub struct UpdateWorkflowPayload {
    pub name: Option<String>,
    pub position: Option<i32>,
}

pub struct WorkflowController;

#[controller("/jira/projects")]
impl WorkflowController {
    /// POST /jira/projects/:project_id/workflow/add - Add workflow step
    #[post("/:project_id/workflow/add")]
    #[cap(ProjectAdmin)]
    pub async fn add_workflow_step(
        ctx: RequestContext,
        workflow_repo: Arc<WorkflowRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let is_htmx = ctx.req.has_header("hx-request");

        match workflow_repo.create_step(project_id).await {
            Ok(step) => {
                if is_htmx {
                    let step_html = html! {
                        div class="flex items-center justify-between bg-gray-950 border border-gray-800 rounded-lg p-2 hover:border-gray-700 transition" {
                            div class="flex items-center gap-3 flex-1" {
                                span class="text-xxs text-gray-500 w-6" { (step.position) }
                                input type="text"
                                    value=(step.name)
                                    class="flex-1 bg-transparent text-sm text-white focus:outline-none focus:border-blue-500 border border-transparent rounded px-2 py-1"
                                    hx-patch={"/jira/projects/" (project_id) "/workflow/" (step.id)}
                                    hx-trigger="change"
                                    hx-target="this"
                                    hx-swap="outerHTML"
                                    placeholder="Step name...";
                                span class="text-gray-500" { "⬜" }
                            }
                            div class="flex items-center gap-2" {
                                button
                                    hx-post={"/jira/projects/" (project_id) "/workflow/" (step.id) "/toggle"}
                                    hx-target="closest div"
                                    hx-swap="outerHTML"
                                    class="text-gray-500 hover:text-gray-400" {
                                    "✅"
                                }
                                button
                                    hx-delete={"/jira/projects/" (project_id) "/workflow/" (step.id)}
                                    hx-target="closest div"
                                    hx-swap="outerHTML"
                                    hx-confirm={"Delete workflow step '" (step.name) "'?"}
                                    class="text-red-400 hover:text-red-300 text-xxs transition" {
                                    "✕"
                                }
                            }
                        }
                    };
                    Response::ok(step_html.into_string())
                } else {
                    Response::json_ok(&step)
                }
            }
            Err(e) => {
                if is_htmx {
                    let error_html = html! {
                        div class="text-red-400 text-sm p-2" {
                            "Failed to add step: " (e.to_string())
                        }
                    };
                    Response::bad_request(error_html.into_string())
                } else {
                    Response::bad_request(format!("Failed to add step: {}", e))
                }
            }
        }
    }

    /// PATCH /jira/projects/:project_id/workflow/:step_id - Update workflow step
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

        let is_htmx = ctx.req.has_header("hx-request");

        // For HTMX form submissions, use form fields
        let name = ctx.form.fields.get("name").map(|v| v.as_str());

        match workflow_repo.update_step(step_id, name).await {
            Ok(Some(step)) => {
                if is_htmx {
                    let input_html = html! {
                        input type="text"
                            value=(step.name)
                            class="flex-1 bg-transparent text-sm text-white focus:outline-none focus:border-blue-500 border border-transparent rounded px-2 py-1"
                            hx-patch={"/jira/projects/" (ctx.params.get("project_id").unwrap()) "/workflow/" (step.id)}
                            hx-trigger="change"
                            hx-target="this"
                            hx-swap="outerHTML"
                            placeholder="Step name...";
                    };
                    Response::ok(input_html.into_string())
                } else {
                    Response::json_ok(&step)
                }
            }
            Ok(None) => Response::not_found("Step not found"),
            Err(e) => Response::bad_request(format!("Failed to update step: {}", e)),
        }
    }

    /// POST /jira/projects/:project_id/workflow/:step_id/toggle - Toggle workflow step completion
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

        let is_htmx = ctx.req.has_header("hx-request");

        match workflow_repo.toggle_completed(step_id).await {
            Ok(Some(step)) => {
                if is_htmx {
                    let step_html = html! {
                        div class="flex items-center justify-between bg-gray-950 border border-gray-800 rounded-lg p-2 hover:border-gray-700 transition" {
                            div class="flex items-center gap-3 flex-1" {
                                span class="text-xxs text-gray-500 w-6" { (step.position) }
                                input type="text"
                                    value=(step.name)
                                    class="flex-1 bg-transparent text-sm text-white focus:outline-none focus:border-blue-500 border border-transparent rounded px-2 py-1"
                                    hx-patch={"/jira/projects/" (ctx.params.get("project_id").unwrap()) "/workflow/" (step.id)}
                                    hx-trigger="change"
                                    hx-target="this"
                                    hx-swap="outerHTML"
                                    placeholder="Step name...";
                                span class={@if step.is_completed { "text-green-400" } @else { "text-gray-500" } } {
                                    @if step.is_completed { "✅" } @else { "⬜" }
                                }
                            }
                            div class="flex items-center gap-2" {
                                button
                                    hx-post={"/jira/projects/" (ctx.params.get("project_id").unwrap()) "/workflow/" (step.id) "/toggle"}
                                    hx-target="closest div"
                                    hx-swap="outerHTML"
                                    class={@if step.is_completed { "text-green-400 hover:text-green-300" } @else { "text-gray-500 hover:text-gray-400" } } {
                                    @if step.is_completed { "⬜" } @else { "✅" }
                                }
                                button
                                    hx-delete={"/jira/projects/" (ctx.params.get("project_id").unwrap()) "/workflow/" (step.id)}
                                    hx-target="closest div"
                                    hx-swap="outerHTML"
                                    hx-confirm={"Delete workflow step '" (step.name) "'?"}
                                    class="text-red-400 hover:text-red-300 text-xxs transition" {
                                    "✕"
                                }
                            }
                        }
                    };
                    Response::ok(step_html.into_string())
                } else {
                    Response::json_ok(&step)
                }
            }
            Ok(None) => Response::not_found("Step not found"),
            Err(e) => Response::bad_request(format!("Failed to toggle step: {}", e)),
        }
    }

    /// DELETE /jira/projects/:project_id/workflow/:step_id - Delete workflow step
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

        let is_htmx = ctx.req.has_header("hx-request");

        match workflow_repo.delete_step(step_id).await {
            Ok(true) => {
                if is_htmx {
                    Response::ok("")
                } else {
                    Response::json_ok(&serde_json::json!({ "success": true }))
                }
            }
            Ok(false) => Response::not_found("Step not found"),
            Err(e) => Response::bad_request(format!("Failed to delete step: {}", e)),
        }
    }
}