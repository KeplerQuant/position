use std::rc::Rc;

use anchor_client::{Client, Cluster};
use anyhow::Result;
use clap::Parser;
use client::{
    instructions,
    options::{Commands, Options},
    position::get_positions_by_owner,
};

use raydium_amm_v3::states::{PersonalPositionState, PoolState};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::read_keypair_file, signer::Signer, transaction::Transaction};

pub fn main() -> Result<()> {
    let options = Options::parse();

    let rpc_client = RpcClient::new(options.rpc_url.clone());
    let payer = read_keypair_file(&options.payer_path).unwrap();

    let url = Cluster::Custom(options.rpc_url, options.ws_url);
    let anchor_client = Client::new(url, Rc::new(payer));
    let raydium_program = anchor_client.program(raydium_amm_v3::ID)?;
    let position_program = anchor_client.program(position::ID)?;

    match options.commands {
        Commands::GetPool { pool_id } => {
            let pool_account: PoolState = raydium_program.account(pool_id)?;
            println!("{:#?}", pool_account);
        }
        Commands::GetPosition { position_id } => {
            let position_account: PersonalPositionState = raydium_program.account(position_id)?;
            println!("{:#?}", position_account);
        }
        Commands::GetPositionsByOwner { user_wallet } => {
            let user_positions =
                get_positions_by_owner(&rpc_client, &user_wallet, &raydium_program.id())?;
            println!("{:#?}", user_positions);
        }
        Commands::ClosePosition { position_mint } => {
            let instructions =
                instructions::close_position_instruction(&position_program, position_mint)?;

            let payer = read_keypair_file(&options.payer_path).unwrap();
            let signers = vec![&payer];
            let recent_hash = rpc_client.get_latest_blockhash()?;
            let txn = Transaction::new_signed_with_payer(
                &instructions,
                Some(&payer.pubkey()),
                &signers,
                recent_hash,
            );

            let ret = rpc_client.simulate_transaction(&txn)?;
            println!("{:#?}", ret);
        }
    }

    Ok(())
}
