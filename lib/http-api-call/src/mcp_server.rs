use crate::mcp_service::McpService;
use rmcp::ServiceExt;
use rmcp::transport::stdio;

pub async fn start() -> anyhow::Result<()> {
    let transport = stdio();
    let service = McpService::new();
    let server = service.serve(transport).await?;

    log::info!("http-api-call mcp server started");

    server.waiting().await?;

    log::info!("http-api-call mcp server stopped");

    Ok(())
}
