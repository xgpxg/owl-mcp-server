use rmcp::model::{ListToolsResult, PaginatedRequestParam, Tool};
use rmcp::{RoleServer, ServerHandler, tool};
use std::sync::Arc;

use crate::api_store;
use rmcp::service::RequestContext;

#[derive(Debug, Clone)]
pub struct McpService;

#[tool(tool_box)]
impl McpService {
    pub fn new() -> Self {
        Self {}
    }
}

impl ServerHandler for McpService {
    /*    fn call_tool(
        &self,
        request: CallToolRequestParam,
        context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<CallToolResult, Error>> + Send + '_ {
        todo!()
    }*/

    fn list_tools(
        &self,
        request: PaginatedRequestParam,
        context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, rmcp::Error>> + Send + '_ {
        let tools = api_store::get_all_api()
            .iter()
            .map(|api| Tool {
                name: api.name.clone().into(),
                description: api.description.clone().into(),
                input_schema: Arc::new(api.schema.clone().unwrap_or_default().into()),
            })
            .collect::<Vec<_>>();
        std::future::ready(Ok(ListToolsResult {
            tools,
            next_cursor: None,
        }))
    }
}
