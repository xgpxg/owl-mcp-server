use crate::http_service::{add_api, list_api, remove_api, update_api};
use rocket::data::{ByteUnit, Limits};
use rocket::fs::{FileServer, NamedFile};
use rocket::{Config, get, routes};
use std::net::{SocketAddr, TcpListener};

#[allow(unused)]
fn get_available_port() -> anyhow::Result<u16> {
    // 绑定到 0 端口，让系统分配一个未使用的端口
    let listener = TcpListener::bind("127.0.0.1:0")?;

    // 获取实际分配的端口
    let addr: SocketAddr = listener.local_addr()?;
    Ok(addr.port())
}
pub async fn start() -> anyhow::Result<()> {
    let port = std::env::var("HTTP_PORT")?.parse::<u16>()?;
    let mut builder = rocket::build().configure(Config {
        port,
        limits: Limits::default()
            .limit("json", ByteUnit::Mebibyte(5))
            .limit("data-form", ByteUnit::Mebibyte(100))
            .limit("file", ByteUnit::Mebibyte(100)),
        ..Config::debug_default()
    });

    builder = builder.mount("/api", routes![add_api, remove_api, update_api, list_api]);

    if cfg!(not(debug_assertions)) {
        // 前端服务，映射到文件夹，rank设置为100，优先级低于接口映射
        builder = builder.mount("/", FileServer::from("resources/web").rank(100));
        // VUE单页面在404时需要转发到index.html
        builder = builder.mount("/", routes![index]);
    }

    log::info!("api-call http server started");

    builder.launch().await?;
    Ok(())
}

#[get("/<_..>", rank = 200)]
async fn index() -> Option<NamedFile> {
    NamedFile::open("resources/web/index.html").await.ok()
}
