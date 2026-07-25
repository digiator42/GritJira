use std::sync::Arc;
use serde::{Deserialize, Serialize};
use gritshield::GritSanitizer;
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;

use crate::repositories::user::UserRepository;

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
    pub async fn handle_login(ctx: RequestContext, user_repo: Arc<UserRepository>) -> Response {
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

        // Set session context
        ctx.set_session_data("user_id", &user_id.to_string());
        ctx.set_session_data("role", &user.role);

        Response::json(
            HttpStatus::Ok,
            &AuthResponse {
                success: true,
                message: "Login successful".into(),
                user_id: Some(user_id),
                role: Some(user.role.clone()),
            },
        )
    }

    #[post("/register")]
    pub async fn handle_register(ctx: RequestContext) -> Response {
        let payload = match ctx.json::<RegisterPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid registration data"),
        };

        // TODO: Persist user to DB via UserRepository
        ctx.set_session_data("user_id", "102");
        ctx.set_session_data("role", "Developer");

        Response::json(
            HttpStatus::Created,
            &AuthResponse {
                success: true,
                message: "Registration successful".into(),
                user_id: Some(102),
                role: Some("Developer".into()),
            },
        )
    }
}