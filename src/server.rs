use crate::config::*;
use crate::database;
use anyhow::Ok;
use axum::Router;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use tokio::net::unix::SocketAddr;

pub struct server {
    pub config: &'static ServerConifg,
}

impl server {
    pub fn new(config: &'static ServerConifg) -> Self {
        Self { config }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let db = database::init().await?;

        let router = self.build_router(db);
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

    pub fn build_router(&self, db: DatabaseConnection) -> Router {
        // Router::new()
        //     .route("/", routing::get(index))
        //     .route("/users", routing::get(find_user))
        //     .with_state(db)
        todo!()
    }
}
