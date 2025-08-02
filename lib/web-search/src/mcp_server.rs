use crate::search::WebSearch;
use crate::{ExtractType, FetchType};
use rmcp::handler::server::tool::{Parameters, ToolRouter};
use rmcp::model::{CallToolResult, Content};
use rmcp::{ErrorData, ServerHandler, schemars, tool, tool_handler, tool_router};

pub(crate) struct WebSearchMcpServer {
    tool_router: ToolRouter<WebSearchMcpServer>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub(crate) struct SearchReq {
    pub keyword: String,
    #[serde(default = "default_count")]
    pub count: Option<u32>,
}
fn default_count() -> Option<u32> {
    Some(10)
}
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct ExtractReq {
    pub url: String,
}

#[tool_router]
impl WebSearchMcpServer {
    pub(crate) fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
    #[tool(
        description = "网页搜索功能：输入关键词和结果数量，搜索互联网获取相关网页信息。返回结果包括每个网页的标题、内容摘要和原始URL链接，帮助用户快速了解搜索主题的相关信息"
    )]
    async fn search(
        &self,
        Parameters(SearchReq { keyword, count }): Parameters<SearchReq>,
    ) -> Result<CallToolResult, ErrorData> {
        log::info!(
            "[web-search]search keyword: {}, count: {:?}",
            keyword,
            count
        );
        match WebSearch::search(&keyword, count).await {
            Ok(res) => Ok(CallToolResult::success(vec![Content::json(res)?])),
            Err(e) => {
                return Err(ErrorData::parse_error(e.to_string(), None));
            }
        }
    }

    #[tool(
        description = "网页内容提取工具：从指定URL链接中提取网页正文内容，并将其转换为结构化的Markdown格式，便于阅读和后续处理"
    )]
    async fn extract(
        &self,
        Parameters(ExtractReq { url }): Parameters<ExtractReq>,
    ) -> Result<CallToolResult, ErrorData> {
        log::info!("[web-search]extract url: {}", url);
        match WebSearch::extract(&url, FetchType::Dynamic, ExtractType::Algorithm).await {
            Ok(res) => Ok(CallToolResult::success(vec![Content::json(res)?])),
            Err(e) => {
                return Err(ErrorData::parse_error(e.to_string(), None));
            }
        }
    }
}

#[tool_handler]
impl ServerHandler for WebSearchMcpServer {}
