use gritshield::{http::Response, routing::RequestContext};
use maud::Markup;

use crate::web::layouts::shell::shell;

pub trait MaudRender {
    fn render(self, ctx:RequestContext, is_htmx: bool, title: &str) -> Response;
}

impl MaudRender for Markup {
    fn render(self, ctx: RequestContext, is_htmx: bool, title: &str) -> Response {
        let is_inner_htmx = ctx.headers
            .get("Hx-Request")
            .map(|v| v == "true")
            .unwrap_or(false);

        if is_inner_htmx {
            // Return fragment directly for HTMX swaps
            Response::ok(self.into_string())
        } else {
            // Wrap in admin_shell for full page reloads
            shell(ctx, title, self, false)
        }
    }
}