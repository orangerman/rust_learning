mod app;
mod config;
mod database;
mod entity;
mod logger;
mod server;

use anyhow::Ok;
use axum::{Router, debug_handler, extract::State, response::IntoResponse, routing};
use sea_orm::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 日志初始化
    logger::init();

    // 数据库初始化
    let db = database::init().await?;

    //路由配置
    let router: Router = Router::new()
        .route("/", routing::get(index))
        .route("/users", routing::get(find_user))
        .with_state(db);

    let port: u16 = config::get().server().port();
    // 监听配置
    let bind_add: String = format!("127.0.0.1:{port}");
    let listener: TcpListener = TcpListener::bind(&bind_add).await.unwrap();

    tracing::info!("Listener bind {bind_add}");

    axum::serve(listener, router).await.unwrap();
    Ok(())
}
