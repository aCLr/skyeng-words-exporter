use anyhow::Result;

use std::env;

use skyeng_words::client::Client;
mod sync;
use skyeng_words::db;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    migrate().await?;
    db::init_pool().await;

    let client = Client::new(
        env::var("LOGIN").expect("login expected"),
        env::var("PASSWORD").expect("password expected"),
    )?;

    log::info!("start login");
    client.login().await?;
    log::info!("logged in");

    sync::sync(&client).await?;

    Ok(())
}

async fn migrate() -> Result<()> {
    use dotenv::dotenv;
    use migration::{Migrator, MigratorTrait};
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("Environment variable 'DATABASE_URL' not set");

    let connection = sea_orm::Database::connect(&url).await?;
    Ok(Migrator::up(&connection, None).await?)
}
