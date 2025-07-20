use rmcp::handler::server::tool::{Parameters, ToolRouter};
use rmcp::model::{CallToolResult, Content};
use rmcp::transport::stdio;
use rmcp::{ErrorData, ServerHandler, ServiceError, ServiceExt, tool, tool_handler, tool_router, schemars};

mod image_to_text;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    #[tool(description = "Increment the counter by 1")]
    async fn chat_to_image(
        &self,
        Parameters(Req {
            prompt,
            image_paths,
        }): Parameters<Req>,
    ) -> Result<CallToolResult, ErrorData> {
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
