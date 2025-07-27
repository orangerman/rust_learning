mod config;
mod database;
mod entity;
mod logger;

use anyhow::Ok;
use axum::{Router, debug_handler, extract::State, response::IntoResponse, routing};
use entity::prelude::*;
use entity::*;
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

#[debug_handler]
async fn index() -> &'static str {
    "hello rust!"
}

#[debug_handler]
async fn find_user(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    let users = User::find()
        .filter(user::Column::Name.eq("123123"))
        .all(&db)
        .await
        .unwrap();
    axum::Json(users)
}
