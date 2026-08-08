use crate::repositories::project::ProjectRepository;
use crate::repositories::project_member::ProjectMemberRepository;
use crate::repositories::user::UserRepository;
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use gritshield::{GritSanitizer, info};
use sea_orm::ModelTrait;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, GritSanitizer)]
pub struct LoginPayload {
    #[clean(trim, lowercase)]
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, GritSanitizer)]
pub struct RegisterPayload {
    #[clean(trim, html_escape)]
    pub name: String,
    #[clean(trim, lowercase)]
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<i32>,
    pub role: Option<String>,
}

pub struct AuthController;

#[controller("/api/v1/auth")]
impl AuthController {
    #[post("/login")]
    pub async fn handle_login(
        ctx: RequestContext,
        user_repo: Arc<UserRepository>,
        project_repo: Arc<ProjectRepository>,
        project_member_repo: Arc<ProjectMemberRepository>,
    ) -> Response {
        let payload = match ctx.json::<LoginPayload>().await {
            Ok(p) => p,
            Err(e) => return Response::bad_request(format!("Invalid request body: {:?}", e)),
        };

        // Authenticate user against DB
        let user = match user_repo.find_one_by_email(&payload.email).await {
            Ok(Some(u)) => u,
            Ok(None) => return Response::unauthorized("Invalid credentials"),
            Err(_) => return Response::internal_error("Database error"),
        };

        let user_id = user.id;

        // Get default project from project_members table
        let default_project_id = match project_member_repo.get_user_default_project(user_id).await {
            Ok(Some(project_id)) => project_id,
            Ok(None) => {
                // User has no projects - try to get first project
                match project_repo.get_first_project().await {
                    Ok(Some(project_id)) => {
                        // Add user to first project
                        let _ = project_member_repo
                            .add_user_to_project(project_id, user_id, &user.username, "Member")
                            .await;
                        project_id
                    }
                    Ok(None) | Err(_) => {
                        1
                    }
                }
            }
            Err(e) => {
                1
            }
        };
        // Get the project key for the default project
        let project_key = match project_repo.get_project_key(default_project_id).await {
            Ok(Some(key)) => key,
            Ok(None) => "DEFAULT".to_string(),
            Err(e) => "DEFAULT".to_string(),
        };

        // Set session context
        ctx.set_session_data("user_id", &user_id.to_string());
        ctx.set_session_data("role", &user.role);
        ctx.set_session_data("current_project_id", &default_project_id.to_string());
        ctx.set_session_data("current_project_key", &project_key);

        // Set user's default project in session for quick access
        ctx.set_session_data("default_project_id", &default_project_id.to_string());

        info!(
            "User {} logged in with default project {} ({})",
            user.email, default_project_id, project_key
        );

        Response::json(
            HttpStatus::Ok,
            &AuthResponse {
                success: true,
                message: "Login successful".into(),
                user_id: Some(user_id),
                role: Some(user.role.clone()),
            },
        )
        .with_header(
            "HX-Redirect",
            &format!("/jira/board?project_id={}", default_project_id),
        )
    }

    #[post("/register")]
    pub async fn handle_register(
        ctx: RequestContext,
        user_repo: Arc<UserRepository>,
    ) -> Response {
        let payload = match ctx.json::<RegisterPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid registration data"),
        };

        // Persist user to DB via UserRepository
        match user_repo
            .create(&payload.name, &payload.email, &payload.password, "Member")
            .await
        {
            Ok(user) => {
                ctx.set_session_data("user_id", &user.id.to_string());
                ctx.set_session_data("role", &user.role);

                Response::json(
                    HttpStatus::Created,
                    &AuthResponse {
                        success: true,
                        message: "Registration successful".into(),
                        user_id: Some(user.id),
                        role: Some(user.role),
                    },
                )
            }
            Err(e) => {
                if e.to_string().contains("duplicate") {
                    Response::bad_request("User with this email or username already exists")
                } else {
                    Response::internal_error(format!("Failed to create user: {}", e))
                }
            }
        }
    }
}
