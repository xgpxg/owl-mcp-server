use crate::search::{PageResult, SearchResult, WebSearch};
use crate::{ExtractType, FetchType};
use clap::ValueEnum;
use rocket::data::{ByteUnit, Limits};
use rocket::response::{Responder, content};
use rocket::{Config, Request, get, routes};
use serde::Serialize;

pub(crate) async fn start_http_server(port: u16) -> anyhow::Result<()> {
    let mut builder = rocket::build().configure(Config {
        port,
        limits: Limits::default()
            .limit("string", ByteUnit::Mebibyte(5))
            .limit("json", ByteUnit::Mebibyte(5))
            .limit("data-form", ByteUnit::Mebibyte(100))
            .limit("file", ByteUnit::Mebibyte(100)),
        ..Config::default()
    });

    // 搜索
    builder = builder.mount("/api", routes![search, extract]);

    builder.launch().await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Res<T: Serialize> {
    data: Option<T>,
    code: i32,
    msg: Option<String>,
}

impl<'r, 'o: 'r, T: Serialize> Responder<'r, 'o> for Res<T> {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let s = match serde_json::to_string(&self) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to serialize response: {}", e);
                return Err(rocket::http::Status::InternalServerError);
            }
        };

        content::RawJson(s).respond_to(request)
    }
}
#[get("/search?<q>&<count>")]
pub(crate) async fn search(q: &str, count: Option<u32>) -> Res<Vec<SearchResult>> {
    match WebSearch::search(q, count).await {
        Ok(result) => Res {
            data: Some(result),
            code: 0,
            msg: None,
        },
        Err(e) => Res {
            data: None,
            code: 1,
            msg: Some(e.to_string()),
        },
    }
}

#[get("/extract?<url>&<fetch_type>&<extract_type>")]
pub(crate) async fn extract(
    url: &str,
    fetch_type: Option<&str>,
    extract_type: Option<&str>,
) -> Res<PageResult> {
    match WebSearch::extract(
        url,
        FetchType::from_str(fetch_type.unwrap_or("static"), true).expect("Invalid fetch type"),
        ExtractType::from_str(extract_type.unwrap_or("algorithm"), true)
            .expect("Invalid extract type"),
    )
    .await
    {
        Ok(result) => Res {
            data: Some(result),
            code: 0,
            msg: None,
        },
        Err(e) => Res {
            data: None,
            code: 1,
            msg: Some(e.to_string()),
        },
    }
}
