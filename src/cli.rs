use clap::{ArgEnum, Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long, env = "DATABASE_URL")]
    pub db_url: String,
    #[clap(env = "SKYENG_LOGIN")]
    pub login: Option<String>,
    #[clap(env = "SKYENG_PASSWORD")]
    pub password: Option<String>,
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Sync,
    SyncWordset(IdOrName),
    Export(Export),
}

#[derive(Debug, Args)]
pub struct Export {
    pub destination: String,
    #[clap(arg_enum, value_parser, default_value = "xlsx")]
    pub format: Format,
    #[clap(short, long, action)]
    pub all: bool,
}

#[derive(ArgEnum, Clone, Copy, Debug)]
pub enum Format {
    Xlsx,
}

#[derive(Debug, Args)]
pub struct IdOrName {
    #[clap(short, long)]
    pub id: Option<i32>,
    #[clap(short, long)]
    pub name: Option<String>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
