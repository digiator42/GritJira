use std::sync::Arc;
use serde::Serialize;
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use gritshield::database::GritRepository;
use crate::repositories::user::UserRepository;
use crate::security::caps::ViewBoard;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
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
            Ok(users) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: users,
                },
            ),
            Err(e) => Response::internal_error(format!("Failed to fetch users: {}", e)),
        }
    }
}