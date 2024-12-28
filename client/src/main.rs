use std::rc::Rc;

use anchor_client::{Client, Cluster};
use anyhow::Result;
use clap::Parser;
use client::{
    config::Config,
    options::{Commands, Options},
};

use solana_sdk::signature::read_keypair_file;

pub fn main() -> Result<()> {
    let options = Options::parse();
    let settings = Config::from_path(&options.config)?.merge_with_options(&options);

    let payer = read_keypair_file(&settings.payer_path).unwrap();

    let url = Cluster::Custom(settings.rpc_url, settings.ws_url);
    let anchor_client = Client::new(url, Rc::new(payer));
    let program = anchor_client.program(settings.raydium_v3_program)?;

    match options.commands {
        Commands::Pool { pool_id } => {
            let pool_account: raydium_amm_v3::states::PoolState = program.account(pool_id)?;
            println!("{:#?}", pool_account);
        }
    }

    Ok(())
}
