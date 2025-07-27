use rmcp::handler::server::tool::{Parameters, ToolRouter};
use rmcp::model::{CallToolResult, Content};
use rmcp::transport::stdio;
use rmcp::{
    ErrorData, ServerHandler, ServiceExt, schemars, tool, tool_handler, tool_router,
};

mod image_to_text;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    common::init_log();

    let transport = stdio();
    let service = ImageQA::new();
    let server = service.serve(transport).await?;

    log::info!("image-qa-online mcp server started");

    server.waiting().await?;

    log::info!("image-qa-online mcp server stopped");

    Ok(())
}

struct ImageQA {
    tool_router: ToolRouter<ImageQA>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct Req {
    pub prompt: String,
    pub image_paths: Vec<String>,
}

#[tool_router]
impl ImageQA {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
    #[tool(description = "可以对图片进行提问，分析图片，提取图片中的文本，生成图片摘要")]
    async fn chat_to_image(
        &self,
        Parameters(Req {
            prompt,
            image_paths,
        }): Parameters<Req>,
    ) -> Result<CallToolResult, ErrorData> {
        log::info!(
            "[image-qa-online]prompt: {}, image_paths: {:?}",
            prompt,
            image_paths
        );
        match image_to_text::extra(prompt, image_paths).await {
            Ok(res) => Ok(CallToolResult::success(vec![Content::text(
                res.unwrap_or_default(),
            )])),
            Err(e) => {
                return Err(ErrorData::parse_error(e.to_string(), None));
            }
        }
    }
}

#[tool_handler]
impl ServerHandler for ImageQA {}

#[tokio::test]
async fn test_image_to_text() {
    let res = image_to_text::extra(
        "请将图片中的内容翻译成中文".to_string(),
        vec!["https://pic1.imgdb.cn/item/687cf7ad58cb8da5c8c89527.png".to_string()],
    )
    .await
    .unwrap();

    println!("res: {:?}", res);
}
