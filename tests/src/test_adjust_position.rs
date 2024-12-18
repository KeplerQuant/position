use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};
use anchor_spl::token_2022::spl_token_2022;
use orca_whirlpools_client::get_position_address;
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
    let (position_pda, _) = get_position_address(&position_mint).unwrap();
    let mint_info = program.rpc().get_account(&position_mint).unwrap();
    let position_token_account = get_associated_token_address_with_program_id(
        &program.payer(),
        &position_mint,
        &mint_info.owner,
    );

    dbg!(&position_token_account);

    let tx = program
        .request()
        .accounts(position::accounts::AdjustPosition {
            whirlpool_program: whirlpool_cpi::ID,
            position_authority: program.payer(),
            receiver: program.payer(),
            position: position_pda,
            position_mint,
            position_token_account,
            token_2022_program: spl_token_2022::ID,
        })
        .args(position::instruction::AdjustPosition {})
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}
