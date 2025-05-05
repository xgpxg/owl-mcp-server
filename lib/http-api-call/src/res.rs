use rocket::Request;
use rocket::http::ContentType;
use rocket::response::{Responder, content};
use rocket::serde::{Deserialize, Serialize};
use serde_json::json;
use crate::result::AppError;

///通用Json响应返回
#[derive(Debug, Serialize, Deserialize)]
pub struct Res<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

///原始数据返回，返回文本或字节数组
pub enum RawData {
    Text(String),
    Bytes(Vec<u8>),
    Error(String),
}

///指定响应类型的返回数据
pub type ResData = (ContentType, RawData);

/// 响应成功
const SUCCESS_CODE: i32 = 0;
/// 系统错误
const ERROR_CODE: i32 = 1;

impl<T> Res<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        content::RawText("");
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

    pub fn from_error(error: AppError) -> Self {
        match error {
            // MessageError传递给调用方
            AppError::MessageError(e) => Res {
                code: ERROR_CODE,
                msg: e.to_string(),
                data: None,
            },
            // MessageCodeError传递给调用方
            AppError::MessageCodeError(code, e) => Res {
                code,
                msg: e.to_string(),
                data: None,
            },
            // 其他错误，打印错误日志，对外屏蔽错误细节
            e => {
                log::error!("{}", e);
                Res {
                    code: ERROR_CODE,
                    msg: "系统异常".to_string(),
                    data: None,
                }
            }
        }
    }

    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    pub fn to_json_string(&self) -> String {
        json!(&self).to_string()
    }
}

impl<'r, 'o: 'r, T: Serialize> Responder<'r, 'o> for Res<T> {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        json!(&self).respond_to(request)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for RawData {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        match self {
            RawData::Text(content) => content.respond_to(request),
            RawData::Bytes(content) => content.respond_to(request),
            RawData::Error(msg) => Res::<()>::error(msg.as_str()).respond_to(request),
        }
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

pub trait IntoPageRes<I, T>
where
    I: Send + Sync,
    T: Send + Sync,
{
    fn convert_to_page_res<F>(self, f: F) -> PageRes<T>
    where
        F: Fn(Vec<I>) -> Vec<T>;
}
