use anyhow::{bail, Result};

mod cli;
use crate::cli::Format;
use skyeng_words::sync::IdOrName;
use skyeng_words::{client::Client, db, export, sync};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    dotenv::dotenv()?;
    let cli = cli::parse();

    migrate(cli.db_url.as_str()).await?;
    db::init_pool(cli.db_url.as_str()).await;

    let get_client = || async {
        assert!(
            cli.login.is_some() && cli.password.is_some(),
            "login and password must be presented"
        );
        init_client(cli.login.unwrap(), cli.password.unwrap()).await
    };
    match cli.command {
        cli::Command::Sync => {
            let client = get_client().await?;
            sync::sync(&client).await?;
        }
        cli::Command::SyncWordset(id_or_name) => {
            let client = get_client().await?;
            match (id_or_name.id, id_or_name.name) {
                (Some(_), Some(_)) => {
                    bail!("id and name can't be presented simultaneously")
                }
                (None, None) => {
                    bail!("neither id nor name presented")
                }
                (Some(id), None) => {
                    sync::sync_wordset(&client, IdOrName::Id(id)).await?;
                }
                (None, Some(name)) => {
                    sync::sync_wordset(&client, IdOrName::Name(name)).await?;
                }
            }
        }
        cli::Command::Export(export_opts) => match export_opts.format {
            Format::Xlsx => {
                export_to_xlsx(export_opts.all).await?;
            }
        },
    }

    Ok(())
}

async fn init_client(login: String, password: String) -> Result<Client> {
    let client = Client::new(login, password)?;

    log::debug!("start login");
    client.login().await?;
    log::debug!("logged in");
    Ok(client)
}

async fn migrate(url: &str) -> Result<()> {
    use migration::{Migrator, MigratorTrait};
    let connection = sea_orm::Database::connect(url).await?;
    Ok(Migrator::up(&connection, None).await?)
}

async fn export_to_xlsx(all: bool) -> Result<()> {
    let words = match all {
        true => db::get_all_words().await?,
        false => db::get_unexported_words().await?,
    };

    if words.len() == 0 {
        bail!("found no words for export")
    }
    export::export_to_xlsx("temp.xlsx", &words)?;
    db::mark_as_exported(words.iter().map(|w| w.id).collect()).await?;

    Ok(())
}
