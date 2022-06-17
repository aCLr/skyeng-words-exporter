use anyhow::Result;

use std::env;

use skyeng_words::client::Client;
mod sync;

#[tokio::main]
async fn main() -> Result<()> {
    migrate().await?;

    let client = Client::new(
        env::var("LOGIN").expect("login expected"),
        env::var("PASSWORD").expect("password expected"),
    )?;

    client.login().await?;

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
