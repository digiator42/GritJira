use crate::controllers::get_project_context;
use crate::repositories::webhook::WebhookRepository;
use crate::security::caps::ProjectAdmin;
use gritshield::http::response::HttpStatus;
use gritshield::{GritSanitizer, prelude::*};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Deserialize, GritSanitizer)]
pub struct CreateWebhookPayload {
    #[clean(trim, html_escape)]
    pub name: String,

    #[clean(trim)]
    pub url: String,

    #[clean(trim, lowercase)]
    pub event: String,
}

pub struct WebhookController;

#[controller("/api/v1/webhooks")]
impl WebhookController {
    /// GET /api/v1/webhooks?project_id=N - List webhooks for a project
    #[get("")]
    #[cap(ProjectAdmin)]
    pub async fn list(
        ctx: RequestContext,
        webhook_repo: Arc<WebhookRepository>,
    ) -> Response {
        let project_id = get_project_context(&ctx);
        match webhook_repo.list_by_project(project_id).await {
            Ok(hooks) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: hooks,
                },
            ),
            Err(e) => Response::internal_error(e.to_string()),
        }
    }

    /// POST /api/v1/webhooks - Register a webhook
    #[post("")]
    #[cap(ProjectAdmin)]
    pub async fn create(
        ctx: RequestContext,
        webhook_repo: Arc<WebhookRepository>,
    ) -> Response {
        let project_id = get_project_context(&ctx);
        let payload = match ctx.json::<CreateWebhookPayload>().await {
            Ok(p) => p,
            Err(_) => return Response::bad_request("Invalid webhook payload"),
        };

        if payload.url.trim().is_empty() {
            return Response::bad_request("Webhook URL is required");
        }

        match webhook_repo
            .create(project_id, &payload.name, &payload.url, &payload.event)
            .await
        {
            Ok(hook) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: hook,
                },
            ),
            Err(e) => Response::bad_request(format!("Failed to create webhook: {}", e)),
        }
    }

    /// DELETE /api/v1/webhooks/:id - Remove a webhook (admin)
    #[delete("/:id")]
    #[cap(ProjectAdmin)]
    pub async fn delete(
        ctx: RequestContext,
        webhook_repo: Arc<WebhookRepository>,
    ) -> Response {
        let webhook_id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid webhook ID"),
        };

        match webhook_repo.delete(webhook_id).await {
            Ok(true) => Response::json(
                HttpStatus::Ok,
                &ApiResponse {
                    success: true,
                    data: serde_json::json!({ "deleted": true }),
                },
            ),
            Ok(false) => Response::not_found("Webhook not found"),
            Err(e) => Response::internal_error(e.to_string()),
        }
    }
}
