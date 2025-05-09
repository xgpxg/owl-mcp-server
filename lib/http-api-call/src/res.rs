use rmcp::Error;
use rmcp::handler::server::tool::IntoCallToolResult;
use rmcp::model::{CallToolResult, Content};
use serde::{Deserialize, Serialize};
use serde_json::json;

///通用Json响应返回
#[derive(Debug, Serialize, Deserialize)]
pub struct Res<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

/// 响应成功
const SUCCESS_CODE: i32 = 0;
/// 系统错误
const ERROR_CODE: i32 = 1;

impl<T> Res<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Res {
            code: SUCCESS_CODE,
            msg: "".to_string(),
            data: Some(data),
        }
    }

    pub fn error(msg: &str) -> Self {
        Res {
            code: ERROR_CODE,
            msg: msg.to_string(),
            data: None,
        }
    }

    #[allow(unused)]
    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    #[allow(unused)]
    pub fn to_json_string(&self) -> String {
        json!(&self).to_string()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageRes<T> {
    pub page_num: u64,
    pub page_size: u64,
    pub total: u64,
    pub list: Vec<T>,
}


impl<T: Serialize> IntoCallToolResult for Res<T> {
    fn into_call_tool_result(self) -> Result<CallToolResult, Error> {
        Ok(CallToolResult::success(vec![Content::json(self)?]))
    }
}
