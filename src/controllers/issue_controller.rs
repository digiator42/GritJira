use std::sync::Arc;
use gritshield::prelude::*;
use gritshield::routing::engine::ShieldResult;

use crate::dtos::{CreateIssuePayload, AddCommentPayload};
use crate::security::caps::{IssueCreate, IssueEdit};
use crate::services::board_service::BoardService;

pub struct IssueController;

#[controller("/api/issues")]
impl IssueController {
    #[post("/create")]
    #[cap(IssueCreate)]
    pub async fn create_issue(ctx: RequestContext) -> ShieldResult<Response> {
        // Automatically deserializes & sanitizes title, description, and issue_type in-place!
        let payload = ctx.json::<CreateIssuePayload>().await?;

        println!("Safe Title: {}", payload.title);         // Trimmed & HTML escaped
        println!("Clean Type: {}", payload.issue_type);    // Trimmed & lowercased

        // Proceed with sanitized payload...
        Ok(Response::ok("Issue created safely"))
    }

    #[post("/:id/comments")]
    #[cap(IssueEdit)]
    pub async fn add_comment(ctx: RequestContext) -> ShieldResult<Response> {
        let payload = ctx.json::<AddCommentPayload>().await?;

        println!("Safe Comment Body: {}", payload.body); // XSS vectors stripped/escaped

        Ok(Response::ok("Comment added safely"))
    }
}