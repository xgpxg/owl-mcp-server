use std::process::exit;
use tokio::signal::unix::{SignalKind, signal};

mod api_store;
mod http_server;
mod http_service;
mod log_config;
mod mcp_server;
mod mcp_service;
mod res;
mod result;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    log_config::init_log();

    // 加载API文件
    api_store::init()?;

    // 启动mcp server
    mcp_server::start().await?;

    Ok(())
}
