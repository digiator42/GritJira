use crate::repositories::attachment::AttachmentRepository;
use crate::security::caps::{IssueEdit, ViewBoard};
use crate::services::attachment_service::AttachmentService;
use gritshield::http::response::HttpStatus;
use gritshield::prelude::*;
use gritshield::GritSanitizer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use base64::Engine as _;

#[derive(Deserialize, Serialize, GritSanitizer)]
pub struct UploadAttachmentPayload {
    #[clean(trim)]
    pub filename: String,

    #[clean(trim, lowercase)]
    pub mime: String,

    pub data_base64: String,
}

#[derive(Serialize)]
pub struct AttachmentContent {
    pub id: i32,
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: i32,
    pub data_base64: String,
}

pub struct AttachmentController;

#[controller("/api/v1/attachments")]
impl AttachmentController {
    /// GET /api/v1/attachments/:id/content - Fetch attachment bytes as base64 JSON
    #[get("/:id/content")]
    #[cap(ViewBoard)]
    pub async fn content(
        ctx: RequestContext,
        attachment_repo: Arc<AttachmentRepository>,
        attachment_service: Arc<AttachmentService>,
    ) -> Response {
        let id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid attachment ID"),
        };
        let att = match attachment_repo.get_by_id(id).await {
            Ok(Some(a)) => a,
            Ok(None) => return Response::json_not_found_msg("Attachment not found"),
            Err(e) => return Response::json_internal_error(&e.to_string()),
        };
        match attachment_service.read_bytes(&att.storage_key) {
            Ok(bytes) => Response::json(
                HttpStatus::Ok,
                &AttachmentContent {
                    id: att.id,
                    filename: att.filename,
                    mime_type: att.mime_type,
                    size_bytes: att.size_bytes,
                    data_base64: base64::engine::general_purpose::STANDARD.encode(bytes),
                },
            ),
            Err(_) => Response::json_not_found_msg("Attachment file missing from storage"),
        }
    }

    /// DELETE /api/v1/attachments/:id - Remove an attachment
    #[delete("/:id")]
    #[cap(IssueEdit)]
    pub async fn delete(
        ctx: RequestContext,
        attachment_repo: Arc<AttachmentRepository>,
        attachment_service: Arc<AttachmentService>,
    ) -> Response {
        let id: i32 = match ctx.params.get("id").and_then(|p| p.parse().ok()) {
            Some(id) => id,
            None => return Response::bad_request("Invalid attachment ID"),
        };
        let att = match attachment_repo.get_by_id(id).await {
            Ok(Some(a)) => a,
            Ok(None) => return Response::json_not_found_msg("Attachment not found"),
            Err(e) => return Response::json_internal_error(&e.to_string()),
        };
        attachment_service.remove_bytes(&att.storage_key);
        if let Err(e) = attachment_repo.delete(id).await {
            return Response::json_internal_error(&e.to_string());
        }
        Response::json_no_content()
    }
}