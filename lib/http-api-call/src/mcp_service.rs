use crate::api_store;
use crate::api_store::Api;
use crate::res::{PageRes, Res};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method};
use rmcp::model::{
    CallToolRequestParam, CallToolResult, Content, ErrorData, ListToolsResult,
    PaginatedRequestParam, Tool,
};
use rmcp::schemars::schema::Schema;
use rmcp::schemars::{JsonSchema, SchemaGenerator};
use rmcp::service::RequestContext;
use rmcp::{Error, RoleServer, ServerHandler};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use std::time::Duration;
use common::log;

#[derive(Debug, Clone)]
pub struct McpService;

impl McpService {
    pub fn new() -> Self {
        Self {}
    }

    /// 添加或更新API
    fn add_or_update_api(&self, req: Api) -> Res<()> {
        match api_store::add_api(req) {
            Ok(_) => Res::success(()),
            Err(e) => Res::error(&e.to_string()),
        }
    }

    /// 删除API
    fn remove_api(&self, req: RemoveApiReq) -> Res<()> {
        match api_store::remove_api(&req.name) {
            Ok(_) => Res::success(()),
            Err(e) => Res::error(&e.to_string()),
        }
    }
    /// 查询API列表
    fn list_api(&self, req: Option<ListApiReq>) -> Res<PageRes<Api>> {
        let req = req.unwrap();
        match api_store::filter_api_page(
            &req.filter.unwrap_or_default(),
            req.page.page_num as usize,
            req.page.page_size as usize,
        ) {
            Ok(list) => Res::success(PageRes {
                page_num: req.page.page_num,
                page_size: req.page.page_size,
                total: api_store::count() as u64,
                list,
            }),
            Err(e) => Res::error(&e.to_string()),
        }
    }
}

impl ServerHandler for McpService {
    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, Error> {
        let name = request.name.as_ref();
        let res = match name {
            "add_or_update_api" => {
                let req = request.arguments.unwrap();
                let req = serde_json::from_value::<Api>(req.into()).unwrap();
                serde_json::to_value(self.add_or_update_api(req))
            }
            "remove_api" => {
                let req = request.arguments.unwrap();
                let req = serde_json::from_value::<RemoveApiReq>(req.into()).unwrap();
                serde_json::to_value(self.remove_api(req))
            }
            "list_api" => {
                let req = request.arguments.unwrap();
                let req = serde_json::from_value::<Option<ListApiReq>>(req.into()).unwrap();
                serde_json::to_value(self.list_api(req))
            }
            _ => {
                let api = api_store::get_api(name);
                if api.is_none() {
                    serde_json::to_value(Res::success(()))
                } else {
                    let api = api.unwrap();
                    let url = &api.url;
                    // 提取参数
                    let (query, body, header) = extract_parameters(&api, &request);
                    // 注意大小写，只支持大写
                    let method = Method::from_str(&api.method.to_uppercase()).unwrap();
                    log::info!(
                        "call api, method: {} ,url:  {}, header:{:?}, query: {:?}, body: {:?}",
                        method,
                        url,
                        header,
                        query,
                        body
                    );
                    // 请求接口
                    let res = HTTP_CLIENT
                        .request(method, url)
                        .query(&query)
                        .json(&body)
                        .headers(HeaderMap::from_iter(header.iter().map(|(k, v)| {
                            let name = HeaderName::from_str(k).expect("Invalid header name");
                            let value = HeaderValue::from_str(v.as_str().unwrap())
                                .expect("Invalid header value");
                            (name, value)
                        })))
                        .send()
                        .await;
                    log::info!("call api res: {:?}", res);

                    let res = match res {
                        Ok(res) => res.json::<serde_json::Value>().await,
                        Err(e) => return Err(ErrorData::internal_error(e.to_string(), None)),
                    };
                    let res = match res {
                        Ok(res) => res,
                        Err(e) => return Err(ErrorData::internal_error(e.to_string(), None)),
                    };
                    serde_json::to_value(res)
                }
            }
        };
        let content = match Content::json(res.unwrap()) {
            Ok(content) => content,
            Err(e) => return Err(e),
        };
        let result = CallToolResult::success(vec![content]);
        Ok(result)
    }

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageReq {
    pub page_num: u64,
    pub page_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemoveApiReq {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(self) struct ListApiReq {
    pub page: PageReq,
    pub filter: Option<String>,
}
impl JsonSchema for ListApiReq {
    fn schema_name() -> String {
        "ListApiReq".to_string()
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        Schema::Bool(true)
    }
}

impl JsonSchema for RemoveApiReq {
    fn schema_name() -> String {
        "RemoveApiReq".to_string()
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        Schema::Bool(true)
    }
}

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .connect_timeout(Duration::from_secs(3))
        .read_timeout(Duration::from_secs(30))
        .build()
        .unwrap()
});

/// 从MCP请求参数中提取参数
fn extract_parameters(
    api: &Api,
    call_request: &CallToolRequestParam,
) -> (
    HashMap<String, Value>,
    HashMap<String, Value>,
    HashMap<String, Value>,
) {
    let mut query_params = HashMap::new();
    let mut body_params = HashMap::new();
    let mut header_params = HashMap::new();

    if let Some(params) = api.request_param.clone() {
        let params = params.as_array().unwrap_or(&vec![]).clone();
        for param in params {
            let name = param["name"].as_str().unwrap_or_default();
            let position = param["position"].as_str().unwrap_or_default();

            if let Some(value) = call_request
                .arguments
                .as_ref()
                .and_then(|args| args.get(name))
            {
                match position {
                    "url" => {
                        query_params.insert(name.to_string(), value.clone());
                    }
                    "body" => {
                        body_params.insert(name.to_string(), value.clone());
                    }
                    "header" => {
                        header_params.insert(name.to_string(), value.clone());
                    }
                    _ => {}
                }
            }
        }
    }

    (query_params, body_params, header_params)
}
