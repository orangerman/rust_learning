mod app;
mod config;
mod database;
mod entity;
mod logger;
mod server;

use crate::app::AppState;
use anyhow::Ok;
use axum::{Router, debug_handler, extract::State, response::IntoResponse, routing};
use entity::prelude::*;
use entity::*;
use sea_orm::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //路由配置
    let router: Router<AppState> = Router::new()
        .route("/", routing::get(index))
        .route("/users", routing::get(find_user));

    app::run(router).await?;
    Ok(())
}

#[debug_handler]
async fn index() -> &'static str {
    "hello rust!"
}

#[debug_handler]
async fn find_user(State(AppState { db }): State<AppState>) -> impl IntoResponse {
    let users = User::find()
        .filter(user::Column::Name.eq("123123"))
        .all(&db)
        .await
        .unwrap();
    axum::Json(users)
}
