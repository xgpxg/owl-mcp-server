use crate::http_server;
use crate::mcp_service::McpService;
use rmcp::transport::{SseServer, stdio};
use rmcp::{ServerHandler, ServiceExt};
use serde::de::Error;
use tokio::signal::unix::{SignalKind, signal};

pub async fn start() -> anyhow::Result<()> {
    let transport = stdio();
    let service = McpService::new();
    let server = service.serve(transport).await?;

    log::info!("http-api-call mcp server started");

    // 启动http服务
    let handler = http_server::start()?;

    server.waiting().await?;

    handler.abort();

    log::info!("http-api-call mcp server stopped");

    Ok(())
}
