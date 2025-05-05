use crate::mcp_service::McpService;
use rmcp::ServiceExt;
use rmcp::transport::{SseServer, stdio};
use serde::de::Error;

pub async fn start() -> anyhow::Result<()> {
    tokio::spawn(async move {
        let transport = stdio();
        let service = McpService::new();
        let server = service.serve(transport).await?;

        log::info!("api-call mcp server started");


        tokio::signal::ctrl_c().await?;

        server.cancel().await?;
        Ok::<(),anyhow::Error>(())
    });
    Ok(())
}
