use crate::app::AppState;
use crate::config::*;
use crate::database;
use crate::entity::*;
use anyhow::Ok;
use axum::ServiceExt;
use axum::{Router, debug_handler, extract::State, response::IntoResponse, routing};
use sea_orm::DatabaseConnection;
use sea_orm::prelude::*;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub struct Server {
    pub config: &'static ServerConifg,
}

impl Server {
    pub fn new(config: &'static ServerConifg) -> Self {
        Self { config }
    }

    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let router: Router = self.build_router(state, router);
        let port: u16 = self.config.port();
        // 监听配置
        let bind_addr: String = format!("127.0.0.1:{port}");
        tracing::info!("bind addr {bind_addr} ....");
        let listener: TcpListener = TcpListener::bind(&bind_addr).await.unwrap();
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        Ok(())
    }

    fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        Router::new().merge(router).with_state(state)
    }
}
