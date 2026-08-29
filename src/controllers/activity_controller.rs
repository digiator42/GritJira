use crate::controllers::get_project_context;
use crate::repositories::activity_log::ActivityLogRepository;
use crate::security::caps::ViewBoard;
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

pub struct ActivityController;

#[controller("/api/v1/activity")]
impl ActivityController {
    /// GET /api/v1/activity/projects/:project_id - Latest activity (audit trail)
    #[get("/projects/:project_id")]
    #[cap(ViewBoard)]
    pub async fn get_activity(
        ctx: RequestContext,
        activity_log_repo: Arc<ActivityLogRepository>,
    ) -> Response {
        let project_id: i32 = match ctx.params.get("project_id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid project ID"),
        };

        match activity_log_repo.list_by_project(project_id, 100).await {
            Ok(entries) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: entries,
                },
            ),
            Err(e) => Response::internal_error(e.to_string()),
        }
    }
}

pub struct NotificationController;

#[controller("/api/v1/notifications")]
impl NotificationController {
    /// GET /api/v1/notifications?project_id=N - Notifications feed for the current user
    #[get("")]
    #[cap(ViewBoard)]
    pub async fn get_notifications(
        ctx: RequestContext,
        activity_log_repo: Arc<ActivityLogRepository>,
    ) -> Response {
        let project_id = get_project_context(&ctx);
        let user_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        match activity_log_repo.list_for_user(user_id, project_id, 100).await {
            Ok(entries) => {
                let unread = activity_log_repo
                    .unread_count_for_user(user_id, project_id)
                    .await
                    .unwrap_or(0);
                Response::json(
                    HttpStatus::Ok,
                    &ApiResponse {
                        success: true,
                        data: serde_json::json!({ "items": entries, "unread": unread }),
                    },
                )
            }
            Err(e) => Response::internal_error(e.to_string()),
        }
    }

    /// GET /api/v1/notifications/unread?project_id=N - Unread count only
    #[get("/unread")]
    #[cap(ViewBoard)]
    pub async fn get_unread(
        ctx: RequestContext,
        activity_log_repo: Arc<ActivityLogRepository>,
    ) -> Response {
        let project_id = get_project_context(&ctx);
        let user_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        match activity_log_repo.unread_count_for_user(user_id, project_id).await {
            Ok(unread) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: unread,
                },
            ),
            Err(e) => Response::internal_error(e.to_string()),
        }
    }

    /// POST /api/v1/notifications/read?project_id=N - Mark all of my notifications read
    #[post("/read")]
    #[cap(ViewBoard)]
    pub async fn mark_read(
        ctx: RequestContext,
        activity_log_repo: Arc<ActivityLogRepository>,
    ) -> Response {
        let project_id = get_project_context(&ctx);
        let user_id = ctx
            .get_session_data("user_id")
            .and_then(|id| id.parse().ok())
            .unwrap_or(1);

        match activity_log_repo.mark_read_for_user(user_id, project_id).await {
            Ok(marked) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: serde_json::json!({ "marked_read": marked }),
                },
            ),
            Err(e) => Response::internal_error(e.to_string()),
        }
    }
}