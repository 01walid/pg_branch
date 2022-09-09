mod cli;
mod config;

use crate::cli::Cli;
use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    Cli::run().await?;
    Ok(())
}
