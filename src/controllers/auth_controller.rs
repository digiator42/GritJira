use gritshield::GritSanitizer;
use gritshield::prelude::*;
use gritshield::security::errors::ShieldError;
use sea_orm::DbErr;
use serde::Deserialize;

use crate::repositories::user::UserRepository;
use crate::web::render::MaudRender;
use crate::web::views::auth_view::{login_page_view, register_page_view};

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

pub struct AuthController;

#[controller("/auth")]
impl AuthController {
    // --- LOGIN ---
    #[get("/login")]
    pub async fn login_page(ctx: RequestContext) -> Response {
        login_page_view().render(ctx, true, "Login - GritJira")
    }

    #[post("/login")]
    pub async fn handle_login(ctx: RequestContext, user_repo: Arc<UserRepository>) -> Response {
        let payload = match ctx.json::<LoginPayload>().await {
            Ok(p) => p,
            Err(e) => return Response::bad_request(format!("{:?}", e)),
        };

        // 1. Authenticate user against DB (pseudocode)
        let user = user_repo.find_one_by_email(&payload.email).await.unwrap();

        let user_id = user.unwrap().id;

        // 2. Set Session Context
        ctx.set_session_data("user_id", &format!("{}{}", "user_", user_id));

        match user_id {
            1 => ctx.set_session_data("role", "Admin"),
            3 => ctx.set_session_data("role", "Developer"),
            _ => ctx.set_session_data("role", "User"),
        };

        // 3. HTMX Response: Redirect to board upon success
        Response::ok("").with_header("HX-Redirect", "/jira/board")
    }

    // --- REGISTER ---
    #[get("/register")]
    pub async fn register_page(ctx: RequestContext) -> Response {
        register_page_view().render(ctx, true, "Register - GritJira")
    }

    #[post("/register")]
    pub async fn handle_register(ctx: RequestContext) -> Response {
        let payload = match ctx.json::<RegisterPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid registration data"),
        };

        // Save user to DB & auto-login
        ctx.set_session_data("user_id", "usr_102");
        ctx.set_session_data("role", "Developer");

        Response::ok("").with_header("HX-Redirect", "/jira/board")
    }
}
