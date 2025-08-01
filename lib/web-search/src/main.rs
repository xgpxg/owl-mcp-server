use crate::mcp_server::WebSearchMcpServer;
use crate::search::WebSearch;
use clap::{Parser, ValueEnum};
use rmcp::ServiceExt;
use rmcp::transport::stdio;

mod extract_text;
mod html_helper;
mod http_server;
mod mcp_server;
mod search;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// 执行网页搜索
    Search {
        /// 搜索关键词
        #[arg(short, long)]
        query: String,

        /// 搜索结果数量，最大值为100
        #[arg(short, long, default_value = "10")]
        count: u32,
    },

    /// 提取网页内容
    Extract {
        /// 要提取的网页URL
        #[arg(short, long)]
        url: String,
        /// 抓取类型
        #[arg(short, long, default_value = "static")]
        fetch_type: FetchType,
        /// 提取方式
        #[arg(short, long, default_value = "algorithm")]
        extract_type: ExtractType,
    },

    /// 以HTTP服务启动
    Http {
        #[arg(short, long, default_value = "10020")]
        port: u16,
    },
    /// 以MCP服务启动
    MCP {},
}

#[derive(Debug, Clone, ValueEnum, PartialOrd, PartialEq)]
enum FetchType {
    #[value(name = "static")]
    Static,
    #[value(name = "dynamic")]
    Dynamic,
}

#[derive(Debug, Clone, ValueEnum, PartialOrd, PartialEq)]
enum ExtractType {
    #[value(name = "algorithm")]
    Algorithm,
    #[value(name = "ai")]
    AI,
    #[value(name = "mix")]
    Mix,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    common::init_log();

    setup_panic_hook();

    let args = Args::parse();

    let ctrl_c = tokio::signal::ctrl_c();

    tokio::select! {
        result = run_command(args) => {
            result?
        }
        _ = ctrl_c => {
            log::info!("shutting down...");
            cleanup().await;
            std::process::exit(0);
        }
    }

    Ok(())
}

async fn run_command(args: Args) -> anyhow::Result<()> {
    match args.command {
        Command::Search { query, count } => {
            let results = WebSearch::search(&query, Some(count))
                .await
                .expect("Search failed");
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
        Command::Extract {
            url,
            fetch_type,
            extract_type,
        } => {
            let result = WebSearch::extract(&url, fetch_type, extract_type)
                .await
                .expect("Extract failed");
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Command::Http { port } => {
            http_server::start_http_server(port).await?;
        }
        Command::MCP { .. } => {
            let transport = stdio();
            let service = WebSearchMcpServer::new();
            let server = service.serve(transport).await?;

            log::info!("web-search mcp server started");

            server.waiting().await?;

            log::info!("web-search mcp server stopped");
        }
    }

    WebSearch::close().await;

    Ok(())
}

async fn cleanup() {
    WebSearch::close().await;
}

fn setup_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        log::error!("panic error: {:?}", panic_info);

        // 在单独的线程中执行清理（因为 panic 环境下不能使用 async）
        let _ = std::thread::spawn(|| {
            // 创建一个新的 runtime 来执行异步清理
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                cleanup().await;
            });
        })
        .join();

        // 调用默认的 panic 处理
        default_hook(panic_info);
    }));
}
