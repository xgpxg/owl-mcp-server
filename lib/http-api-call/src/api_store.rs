use dashmap::DashMap;
use rocket::serde::{Deserialize, Serialize};
use std::fs;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Api {
    pub name: String,
    pub description: String,
    pub url: String,
    pub summary: String,
    pub method: String,
    pub schema: Option<serde_json::Map<String, serde_json::Value>>,
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
pub fn add_api(api: Api) -> anyhow::Result<()> {
    let mut map = API_STORE.get().unwrap().clone();
    map.insert(api.name.clone(), api);
    let api_list: Vec<Api> = map.into_iter().map(|x| x.1).collect();
    let api_file = serde_json::to_string_pretty(&api_list)?;
    fs::write(API_FILE, api_file)?;
    Ok(())
}

pub fn get_api(name: &str) -> Option<Api> {
    API_STORE.get().unwrap().get(name).map(|x| x.clone())
}

pub fn get_all_api() -> Vec<Api> {
    log::info!("get all api");
    API_STORE.get().unwrap().iter().map(|x| x.clone()).collect()
}

pub fn remove_api(name: &str) -> anyhow::Result<()> {
    let mut map = API_STORE.get().unwrap().clone();
    map.remove(name);
    let api_list: Vec<Api> = map.into_iter().map(|x| x.1).collect();
    let api_file = serde_json::to_string_pretty(&api_list)?;
    fs::write(API_FILE, api_file)?;
    Ok(())
}

pub fn count() -> usize {
    API_STORE.get().unwrap().len()
}

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
