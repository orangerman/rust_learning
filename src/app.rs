use crate::config;
use crate::database;
use crate::logger;
use crate::server;

use axum::routing::Router;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    // 日志初始化
    logger::init();

    // 数据库初始化
    let db: DatabaseConnection = database::init().await?;

    let app_state: AppState = AppState::new(db);

    let server: server::Server = server::Server::new(config::get().server());

    server.start(app_state, router).await?;
    Ok(())
}
