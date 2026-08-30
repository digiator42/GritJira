use std::sync::Arc;
use serde::{Deserialize, Serialize};
use gritshield::GritSanitizer;
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use gritshield::database::GritRepository;
use crate::models::user::Model as UserModel;
use crate::repositories::user::UserRepository;
use crate::security::caps::ViewBoard;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Serialize)]
pub struct PublicUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub avatar_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

impl From<UserModel> for PublicUser {
    fn from(u: UserModel) -> Self {
        Self {
            id: u.id,
            username: u.username,
            email: u.email,
            role: u.role,
            avatar_url: u.avatar_url,
            created_at: u.created_at,
        }
    }
}

#[derive(Deserialize, GritSanitizer)]
pub struct UpdateProfilePayload {
    #[clean(trim, html_escape)]
    #[serde(default)]
    pub username: Option<String>,

    #[clean(trim, lowercase)]
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Deserialize, GritSanitizer)]
pub struct ChangePasswordPayload {
    pub current_password: String,
    pub new_password: String,
}

pub struct UserController;

#[controller("/api/v1/users")]
impl UserController {
    /// GET /api/v1/users - List users for assignee selection
    #[get("")]
    #[cap(ViewBoard)]
    pub async fn list_users(
        _ctx: RequestContext,
        user_repo: Arc<UserRepository>,
    ) -> Response {
        // Query users using GritRepository's default fetch
        match user_repo.query().fetch().await {
            Ok(users) => {
                let public_users: Vec<PublicUser> = users.into_iter().map(PublicUser::from).collect();
                Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: public_users,
                    },
                )
            }
            Err(e) => Response::internal_error(format!("Failed to fetch users: {}", e)),
        }
    }

    /// PATCH /api/v1/users/me - Update the authenticated user's profile
    #[patch("/me")]
    #[cap(ViewBoard)]
    pub async fn update_me(
        ctx: RequestContext,
        user_repo: Arc<UserRepository>,
    ) -> Response {
        let user_id: i32 = match ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
        {
            Some(id) => id,
            None => return Response::json_unauthorized_msg("Not authenticated"),
        };

        let payload = match ctx.json::<UpdateProfilePayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid profile payload"),
        };

        if payload.email.as_deref().is_some_and(|e| !e.contains('@')) {
            return Response::bad_request("Invalid email address");
        }

        match user_repo
            .update_profile(user_id, payload.username.as_deref(), payload.email.as_deref())
            .await
        {
            Ok(Some(user)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: PublicUser::from(user),
                },
            ),
            Ok(None) => Response::not_found("User not found"),
            Err(e) => {
                if e.to_string().contains("duplicate") {
                    Response::bad_request("Username or email already exists")
                } else {
                    Response::internal_error(format!("Failed to update profile: {}", e))
                }
            }
        }
    }

    /// POST /api/v1/users/me/password - Change the authenticated user's password
    #[post("/me/password")]
    #[cap(ViewBoard)]
    pub async fn change_password(
        ctx: RequestContext,
        user_repo: Arc<UserRepository>,
    ) -> Response {
        let user_id: i32 = match ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
        {
            Some(id) => id,
            None => return Response::json_unauthorized_msg("Not authenticated"),
        };

        let payload = match ctx.json::<ChangePasswordPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid password payload"),
        };

        if payload.current_password.is_empty() {
            return Response::bad_request("Current password is required");
        }
        if payload.new_password.len() < 6 {
            return Response::bad_request("New password must be at least 6 characters");
        }

        match user_repo
            .change_password(
                user_id,
                &payload.current_password,
                &payload.new_password,
            )
            .await
        {
            Ok(Some(true)) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: "Password updated",
                },
            ),
            Ok(Some(false)) => Response::bad_request("Current password is incorrect"),
            Ok(None) => Response::not_found("User not found"),
            Err(e) => Response::internal_error(format!("Failed to change password: {}", e)),
        }
    }
}