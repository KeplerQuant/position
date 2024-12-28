use anyhow::Result;
use clap::Parser;
use client::{
    config::Config,
    options::{Commands, Options},
};
use solana_client::rpc_client::RpcClient;

#[tokio::main]
pub async fn main() -> Result<()> {
    let options = Options::parse();
    let settings = Config::from_path(&options.config)?.merge_with_options(&options);

    let _rpc_client = RpcClient::new(settings.rpc_url);

    match options.commands {
        Commands::Pool { pool_id } => {
            println!("{:#?}", &pool_id);
        }
    }

    Ok(())
}
