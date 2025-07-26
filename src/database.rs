use sea_orm::{ConnectOptions, DatabaseConnection};

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let options = ConnectOptions::new(format!(""));

    todo!()
}
