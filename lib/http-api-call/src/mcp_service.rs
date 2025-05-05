use rmcp::model::{
    CancelledNotificationParam, InitializeRequestParam, InitializeResult, ListToolsResult,
    PaginatedRequestParam, Tool,
};
use rmcp::{Error, RoleServer, ServerHandler, tool};
use std::sync::Arc;

use crate::{api_store, http_server};
use rmcp::service::RequestContext;
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct McpService;

#[tool(tool_box)]
impl McpService {
    pub fn new() -> Self {
        Self {}
    }
}

impl ServerHandler for McpService {
    fn list_tools(
        &self,
        _request: PaginatedRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, Error>> + Send + '_ {
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
