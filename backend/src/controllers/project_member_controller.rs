// src/controllers/project_member_controller.rs
use std::sync::Arc;
use gritshield::GritSanitizer;
use serde::{Deserialize, Serialize};
use gritshield::{
    http::response::HttpStatus,
    prelude::*,
};
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::repositories::project_member::ProjectMemberRepository;
use crate::repositories::user::UserRepository;
use crate::security::caps::{ProjectAdmin, ViewBoard};

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
    ///
    /// Read-only: any project member may view the roster.
    #[get("/:project_id/members")]
    #[cap(ViewBoard)]
    pub async fn list_members(
        ctx: RequestContext,
        project_member_repo: Arc<ProjectMemberRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        // Project rosters are visible to the project's members and to global
        // Admins; other users get 403.
        let user_id = ctx.get_session_data("user_id").and_then(|id| id.parse().ok());
        let is_admin = ctx
            .get_session_data("role")
            .as_deref()
            .is_some_and(|r| r == "Admin");
        let is_member = match user_id {
            Some(uid) => project_member_repo.is_member(project_id, uid).await.unwrap_or(false),
            None => false,
        };
        if !is_admin && !is_member {
            return Response::json_forbidden_msg("You do not have access to this project's members");
        }

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
        user_repo: Arc<UserRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let payload = match ctx.json::<AddMemberPayload>().await {
            Ok(p) => p,
            Err(e) => return Response::bad_request(format!("Invalid request body: {:?}", e)),
        };

        // Resolve the user's real username for the denormalized member record
        let username = match user_repo.find_one_by_id(payload.user_id).await {
            Ok(Some(user)) => user.core.username,
            _ => {
                return Response::bad_request(format!(
                    "User {} does not exist",
                    payload.user_id
                ))
            }
        };

        match project_member_repo
            .add_user_to_project(project_id, payload.user_id, &username, &payload.role)
            .await
        {
            Ok(member) => Response::json(
                HttpStatus::Created,
                &ApiResponse {
                    success: true,
                    data: member,
                },
            ),
            Err(e) => {
                if e.to_string().contains("duplicate") {
                    Response::bad_request("User is already a member of this project")
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
        let _project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        let member_id: i32 = match ctx.params.get("member_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid member ID"),
        };

        match project_member_repo.remove_member(member_id).await {
            Ok(true) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: "Member removed successfully",
                },
            ),
            Ok(false) => Response::not_found("Member not found"),
            Err(e) => Response::bad_request(format!("Failed to remove member: {}", e)),
        }
    }

    /// PATCH /api/v1/projects/:project_id/members/:member_id - Update member role
    #[patch("/:project_id/members/:member_id")]
    #[cap(ProjectAdmin)]
    pub async fn update_member_role(
        ctx: RequestContext,
        project_member_repo: Arc<ProjectMemberRepository>,
    ) -> Response {
        let _project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
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

        match project_member_repo.update_member_role(member_id, &payload.role).await {
            Ok(Some(member)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: member,
                },
            ),
            Ok(None) => Response::not_found("Member not found"),
            Err(e) => Response::bad_request(format!("Failed to update member: {}", e)),
        }
    }
}