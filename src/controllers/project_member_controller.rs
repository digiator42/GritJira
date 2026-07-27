// src/controllers/project_member_controller.rs
use std::sync::Arc;
use gritshield::GritSanitizer;
use serde::{Deserialize, Serialize};
use gritshield::{
    http::response::HttpStatus,
    prelude::*,
};
use sea_orm::{ActiveModelTrait, Set};

use crate::repositories::project_member::ProjectMemberRepository;
use crate::security::caps::ProjectAdmin;
use serde_aux::field_attributes::{
    deserialize_number_from_string,
    deserialize_option_number_from_string,
};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Deserialize, GritSanitizer)]
pub struct AddMemberPayload {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub user_id: i32,
    pub role: String,
}

pub struct ProjectMemberController;

#[controller("/api/v1/projects")]
impl ProjectMemberController {
    /// GET /api/v1/projects/:project_id/members - List all members
    #[get("/:project_id/members")]
    #[cap(ProjectAdmin)]
    pub async fn list_members(
        ctx: RequestContext,
        project_member_repo: Arc<ProjectMemberRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        match project_member_repo.get_project_members_with_users(project_id).await {
            Ok(members) => {
                // Convert to a format that includes username
                let formatted_members: Vec<serde_json::Value> = members
                    .into_iter()
                    .map(|(member, user)| {
                        serde_json::json!({
                            "id": member.id,
                            "project_id": member.project_id,
                            "user_id": member.user_id,
                            "username": user.username,
                            "role": member.role,
                            "joined_at": member.joined_at,
                        })
                    })
                    .collect();

                Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: formatted_members,
                    },
                )
            }
            Err(e) => Response::internal_error(format!("Failed to fetch members: {}", e)),
        }
    }

    /// POST /api/v1/projects/:project_id/members - Add member to project
    #[post("/:project_id/members")]
    #[cap(ProjectAdmin)]
    pub async fn add_member(
        ctx: RequestContext,
        project_member_repo: Arc<ProjectMemberRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let payload = match ctx.json::<AddMemberPayload>().await {
            Ok(p) => p,
            Err(e) => return Response::bad_request(format!("Invalid request body: {:?}", e)),
        };

        let is_htmx = ctx.req.has_header("hx-request");

        match project_member_repo
            .add_user_to_project(project_id, payload.user_id, "dummy", &payload.role)
            .await
        {
            Ok(_) => {
                if is_htmx {
                    // Redirect to refresh the members list
                    let redirect_url = format!("/jira/settings/users?project_id={}", project_id);
                    Response::ok("").with_header("HX-Redirect", &redirect_url)
                } else {
                    Response::json(
                        HttpStatus::Created,
                        &ApiResponse {
                            success: true,
                            data: "Member added successfully",
                        },
                    )
                }
            }
            Err(e) => {
                if is_htmx {
                    let error_html = html! {
                        div class="bg-red-950/30 border border-red-800/60 rounded-lg p-3 text-center" {
                            p class="text-red-300 text-sm" { "Failed to add member: " (e.to_string()) }
                        }
                    };
                    Response::bad_request(error_html.into_string())
                } else {
                    Response::bad_request(format!("Failed to add member: {}", e))
                }
            }
        }
    }

    /// DELETE /api/v1/projects/:project_id/members/:member_id - Remove member
    #[delete("/:project_id/members/:member_id")]
    #[cap(ProjectAdmin)]
    pub async fn remove_member(
        ctx: RequestContext,
        project_member_repo: Arc<ProjectMemberRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let member_id: i32 = match ctx.params.get("member_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid member ID"),
        };

        let is_htmx = ctx.req.has_header("hx-request");

        match project_member_repo.remove_member(member_id).await {
            Ok(true) => {
                if is_htmx {
                    Response::ok("")
                } else {
                    Response::json(
                        HttpStatus::Ok,
                        &ApiResponse {
                            success: true,
                            data: "Member removed successfully",
                        },
                    )
                }
            }
            Ok(false) => Response::not_found("Member not found"),
            Err(e) => {
                if is_htmx {
                    Response::bad_request(format!("Failed to remove member: {}", e))
                } else {
                    Response::bad_request(format!("Failed to remove member: {}", e))
                }
            }
        }
    }

    /// PATCH /api/v1/projects/:project_id/members/:member_id - Update member role
    #[patch("/:project_id/members/:member_id")]
    #[cap(ProjectAdmin)]
    pub async fn update_member_role(
        ctx: RequestContext,
        project_member_repo: Arc<ProjectMemberRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let member_id: i32 = match ctx.params.get("member_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid member ID"),
        };

        let payload = match ctx.json::<AddMemberPayload>().await {
            Ok(p) => p,
            Err(e) => return Response::bad_request(format!("Invalid request body: {:?}", e)),
        };

        let is_htmx = ctx.req.has_header("hx-request");

        match project_member_repo.update_member_role(member_id, &payload.role).await {
            Ok(Some(member)) => {
                if is_htmx {
                    // Return HTML for the updated row
                    let member_html = html! {
                        tr class="hover:bg-gray-800/50 transition" {
                            td class="py-2" {
                                div class="flex items-center gap-2" {
                                    span class="text-gray-400" { "👤" }
                                    span { (member.username) }
                                }
                            }
                            td {
                                select
                                    name="role"
                                    hx-patch={"/api/v1/projects/" (project_id) "/members/" (member.id)}
                                    hx-trigger="change"
                                    hx-ext="json-enc"
                                    hx-target="closest tr"
                                    hx-swap="outerHTML"
                                    class="bg-gray-950 border border-gray-800 rounded px-2 py-1 text-xs text-gray-300 focus:outline-none focus:border-blue-500" {
                                    option value="Admin" selected[member.role == "Admin"] { "Admin" }
                                    option value="Member" selected[member.role == "Member"] { "Member" }
                                    option value="Viewer" selected[member.role == "Viewer"] { "Viewer" }
                                }
                            }
                            td class="text-xxs text-gray-500" { (member.joined_at.format("%Y-%m-%d").to_string()) }
                            td class="text-right" {
                                button
                                    hx-delete={"/api/v1/projects/" (project_id) "/members/" (member.id)}
                                    hx-target="closest tr"
                                    hx-swap="outerHTML"
                                    hx-confirm={"Remove " (member.username) " from the project?"}
                                    class="text-red-400 hover:text-red-300 text-xxs transition" {
                                    "✕ Remove"
                                }
                            }
                        }
                    };
                    Response::ok(member_html.into_string())
                } else {
                    Response::json(
                        HttpStatus::Ok,
                        &ApiResponse {
                            success: true,
                            data: member,
                        },
                    )
                }
            }
            Ok(None) => Response::not_found("Member not found"),
            Err(e) => Response::bad_request(format!("Failed to update member: {}", e)),
        }
    }
}