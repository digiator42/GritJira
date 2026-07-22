use gritshield::prelude::*;

pub async fn system_info(_ctx: RequestContext) -> Response {
    Response::ok("GritShield Engine Core Node Online.")
}
