use anyhow::Result;
use clap::Parser;
use client::{
    config::Config,
    options::{Commands, Options},
};

#[tokio::main]
pub async fn main() -> Result<()> {
    let options = Options::parse();
    let settings = Config::from_path(&options.config)?;

    match options.commands {
        Commands::Pool { pool_id } => {
            println!("{:#?}", &pool_id);
        }
    }

    Ok(())
}
