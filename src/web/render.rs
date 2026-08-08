// src/web/render.rs
use gritshield::{http::Response, routing::RequestContext};
use maud::Markup;
use crate::web::layouts::shell::shell;

pub trait MaudRender {
    fn render(self, ctx: RequestContext, title: &str) -> Response;
}

impl MaudRender for Markup {
    fn render(self, ctx: RequestContext, title: &str) -> Response {
        let is_htmx = ctx.headers
            .get("hx-request")
            .and_then(|v| v.first())
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
        
        // 2. Authenticated users: Partial swap vs Full Shell
        if is_htmx {
            Response::ok(self.into_string())
        } else {
            shell(ctx, title, self, false)
        }
    }
}