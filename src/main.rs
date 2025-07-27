mod config;
mod database;
mod logger;

use anyhow::Ok;
use axum::{Router, debug_handler, routing};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 日志初始化
    logger::init();

    // 数据库初始化
    database::init().await?;

    //路由配置
    let router: Router = Router::new().route("/", routing::get(index));

    let port: u16 = config::get().server().port();
    // 监听配置
    let bind_add: String = format!("0.0.0.0:{port}");
    let listener: TcpListener = TcpListener::bind(&bind_add).await.unwrap();

    tracing::info!("Listener bind {bind_add}");

    axum::serve(listener, router).await.unwrap();
    Ok(())
}

#[debug_handler]
async fn index() -> &'static str {
    "hello rust!"
}
