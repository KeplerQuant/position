use std::rc::Rc;

use anchor_client::{Client, Cluster};
use anyhow::Result;
use clap::Parser;
use client::options::{Commands, Options};

use solana_sdk::signature::read_keypair_file;

pub fn main() -> Result<()> {
    let options = Options::parse();

    let payer = read_keypair_file(&options.payer_path).unwrap();

    let url = Cluster::Custom(options.rpc_url, options.ws_url);
    let anchor_client = Client::new(url, Rc::new(payer));
    let program = anchor_client.program(options.raydium_v3_program)?;

    match options.commands {
        Commands::Pool { pool_id } => {
            let pool_account: raydium_amm_v3::states::PoolState = program.account(pool_id)?;
            println!("{:#?}", pool_account);
        }
    }

    Ok(())
}
