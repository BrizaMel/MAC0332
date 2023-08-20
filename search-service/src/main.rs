pub mod cli;
pub mod controller;

use crate::cli::{Cli, Command};
use crate::controller::http::run_http_server;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::RunHttpServer => run_http_server().await?,
    }

    Ok(())
}
