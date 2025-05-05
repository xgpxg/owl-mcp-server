use crate::api_store;
use crate::api_store::Api;
use crate::res::Res;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{Request, Response, post, response};
use serde::{Deserialize, Serialize};

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
struct ListApiReq {
    pub page: PageReq,
    pub filter: Option<String>,
}

/// 添加API
#[post("/add", data = "<req>")]
pub async fn add_api(req: Json<Api>) -> Res<()> {
    match api_store::add_api(req.into_inner()) {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

#[post("/remove", data = "<req>")]
pub async fn remove_api(req: Json<RemoveApiReq>) -> Res<()> {
    match api_store::remove_api(&req.name) {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

/// 修改API
#[post("/update", data = "<req>")]
pub async fn update_api(req: Json<Api>) -> Res<()> {
    match api_store::add_api(req.into_inner()) {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

#[post("/list", data = "<req>")]
pub async fn list_api(req: Json<ListApiReq>) -> Res<Vec<Api>> {
    let req = req.into_inner();
    match api_store::filter_api_page(
        &req.filter.unwrap_or_default(),
        req.page.page_num as usize,
        req.page.page_size as usize,
    ) {
        Ok(apis) => Res::success(apis),
        Err(e) => Res::error(&e.to_string()),
    }
}
