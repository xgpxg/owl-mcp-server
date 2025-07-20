mod api_store;
mod mcp_server;
mod mcp_service;
mod res;
mod result;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    common::init_log();

    // 加载API文件
    api_store::init()?;

    // 启动mcp server
    mcp_server::start().await?;

    Ok(())
}
