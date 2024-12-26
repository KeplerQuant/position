use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};
use spl_associated_token_account::get_associated_token_address_with_program_id;

#[test]
fn test_adjust_position() {
    let program_id = "Dk1TMwYtjVEEZsovJrtun34hae6Vw1mqhFMjgZ8zkmQ7";
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let position_mint = Pubkey::from_str("3sMCGsHSDtihSuuQ8KxX7XrKaGYfc38WE77SELcRNAPK").unwrap();
    let mint_info = program.rpc().get_account(&position_mint).unwrap();
    let position_token_account = get_associated_token_address_with_program_id(
        &program.payer(),
        &position_mint,
        &mint_info.owner,
    );

    dbg!(&position_token_account);

    let tx = program
        .request()
        .accounts(position::accounts::AdjustPosition {})
        .args(position::instruction::AdjustPosition {})
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}
