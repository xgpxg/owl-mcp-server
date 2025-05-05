use dashmap::DashMap;
use rocket::serde::{Deserialize, Serialize};
use std::fs;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Api {
    /// 唯一的名称（作为工具名称）
    pub name: String,
    /// 功能描述
    pub description: String,
    /// 请求地址
    pub url: String,
    /// 简介（即中文名称）
    pub summary: String,
    /// 请求方法
    pub method: String,
    /// 请求参数
    pub request_params: Option<serde_json::Value>,
    /// 请求参数对应的json schema（作为工具的input_schema）
    pub schema: Option<serde_json::Map<String, serde_json::Value>>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 状态: 0未启用 1已启用
    pub status: Option<i8>,
}

const API_FILE: &str = "data/api.json";

static API_STORE: OnceLock<DashMap<String, Api>> = OnceLock::new();

pub(crate) fn init() -> anyhow::Result<()> {
    let path = std::path::Path::new(API_FILE);
    if !path.exists() {
        //SAFE
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(API_FILE, "[]")?;
        println!("api file created");
    }
    let api_file = fs::read_to_string(API_FILE)?;
    log::info!("api file:{}", api_file);
    let api_list: Vec<Api> = serde_json::from_str(&api_file)?;
    log::info!("api list:{:?}", api_list);
    let map = api_list
        .into_iter()
        .map(|x| (x.name.clone(), x))
        .collect::<DashMap<String, Api>>();
    API_STORE.get_or_init(|| map);
    Ok(())
}

/// 保存到磁盘
fn save_to_disk() -> anyhow::Result<()> {
    let mut map = API_STORE.get().unwrap();
    let api_list = map.iter().map(|x| x.clone()).collect::<Vec<_>>();
    let api_file = serde_json::to_string_pretty(&api_list)?;
    fs::write(API_FILE, api_file)?;
    Ok(())
}

/// 添加API
pub fn add_api(api: Api) -> anyhow::Result<()> {
    {
        let mut map = API_STORE.get().unwrap();
        map.insert(api.name.clone(), api);
    }
    save_to_disk()?;
    Ok(())
}

/// 获取API
pub fn get_api(name: &str) -> Option<Api> {
    API_STORE.get().unwrap().get(name).map(|x| x.clone())
}

/// 获取所有API
pub fn get_all_api() -> Vec<Api> {
    log::info!("get all api");
    API_STORE.get().unwrap().iter().map(|x| x.clone()).collect()
}

/// 删除API
pub fn remove_api(name: &str) -> anyhow::Result<()> {
    {
        let mut map = API_STORE.get().unwrap();
        map.remove(name);
    }
    save_to_disk()?;
    Ok(())
}

/// 获取API数量
pub fn count() -> usize {
    API_STORE.get().unwrap().len()
}

/// 筛选API
pub fn filter_api_page(filter: &str, page: usize, page_size: usize) -> anyhow::Result<Vec<Api>> {
    let list = API_STORE
        .get()
        .unwrap()
        .iter()
        .filter(|x| {
            x.name.contains(filter) || x.description.contains(filter) || x.summary.contains(filter)
        })
        .skip((page - 1) * page_size)
        .take(page_size)
        .map(|x| x.clone())
        .collect();
    Ok(list)
}
