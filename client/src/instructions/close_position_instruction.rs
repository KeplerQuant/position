use std::rc::Rc;

use anchor_client::Client;
use anchor_lang::system_program;
use anyhow::Result;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Keypair};

use crate::position::get_position_pda;

pub fn close_position_instruction(
    client: &Client<Rc<Keypair>>,
    nft_mint: Pubkey,
) -> Result<Vec<Instruction>> {
    let program = client.program(raydium_amm_v3::ID)?;

    let nft_ata_token_account =
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &program.payer(),
            &nft_mint,
            &spl_token_2022::ID,
        );
    let personal_position_key = get_position_pda(&nft_mint, &program.id());

    let instructions = program
        .request()
        .accounts(raydium_amm_v3::accounts::ClosePosition {
            nft_owner: program.payer(),
            position_nft_mint: nft_mint,
            position_nft_account: nft_ata_token_account,
            personal_position: personal_position_key,
            system_program: system_program::ID,
            token_program: spl_token_2022::id(),
        })
        .args(position::instruction::ClosePosition {})
        .instructions()?;

    Ok(instructions)
}
