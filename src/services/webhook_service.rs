use crate::repositories::webhook::WebhookRepository;
use gritshield::GritComponent;
use sea_orm::DatabaseConnection;
use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

#[derive(Clone, GritComponent)]
pub struct WebhookService {
    pub webhook_repo: Arc<WebhookRepository>,
    pub db: DatabaseConnection,
}

impl WebhookService {
    /// Fire an event to all matching active webhooks for the project.
    ///
    /// Delivery is fire-and-forget on a background thread so the request loop
    /// is never blocked by a slow target. Best-effort: failures are logged.
    pub async fn fire(&self, project_id: i32, event: &str, payload: &Value) {
        let hooks = self
            .webhook_repo
            .list_for_event(project_id, event)
            .await
            .unwrap_or_default();
        if hooks.is_empty() {
            return;
        }
        let body = payload.to_string();
        for h in hooks {
            let url = h.url.clone();
            let name = h.name.clone();
            let body = body.clone();
            std::thread::spawn(move || {
                if let Err(e) = post_json(&url, &body) {
                    eprintln!("[webhook] {} -> {} delivery failed: {}", name, url, e);
                }
            });
        }
    }
}

/// Minimal outbound `POST application/json` over HTTP (dependency-free).
/// HTTPS targets are not supported.
fn post_json(url: &str, body: &str) -> std::io::Result<()> {
    let rest = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .unwrap_or(url);
    let (host_port, path) = match rest.split_once('/') {
        Some((hp, p)) => (hp, format!("/{}", p)),
        None => (rest, "/".to_string()),
    };
    let mut host_port_iter = host_port.splitn(2, ':');
    let host = host_port_iter.next().unwrap_or("localhost").to_string();
    let port = host_port_iter
        .next()
        .and_then(|p| p.parse().ok())
        .unwrap_or(80);

    let request = format!(
        "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nUser-Agent: GritJira-Webhook/1.0\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        path,
        host,
        body.len(),
        body
    );

    let mut conn = TcpStream::connect((host.as_str(), port))?;
    conn.write_all(request.as_bytes())?;
    let mut response = String::new();
    conn.read_to_string(&mut response)?;

    let status_ok = response.starts_with("HTTP/1.1 2") || response.starts_with("HTTP/1.0 2");
    if status_ok {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "webhook target {}/{} returned: {}",
            host,
            port,
            response.lines().next().unwrap_or("unknown status")
        )))
    }
}