pub mod errors;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use whirlpool_cpi::{program::Whirlpool, state::Position};

declare_id!("EvvVHCu51jPjxJv7nHipFcWkCL3PxL22rbGg4B7BYayu");

#[program]
pub mod position {

    use super::*;

    pub fn initialize(ctx: Context<Reposition>) -> Result<()> {
        let cpi_program = ctx.accounts.whirlpool_program.to_account_info();

        let cpi_accounts = whirlpool_cpi::cpi::accounts::ClosePosition {
            position_authority: ctx.accounts.position_authority.to_account_info(),
            receiver: ctx.accounts.receiver.to_account_info(),
            position: ctx.accounts.position.to_account_info(),
            position_mint: ctx.accounts.position_mint.to_account_info(),
            position_token_account: ctx.accounts.position_token_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        msg!("Greetings from: {:?}", ctx.program_id);

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        whirlpool_cpi::cpi::close_position(cpi_ctx)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Reposition<'info> {
    pub whirlpool_program: Program<'info, Whirlpool>,

    pub position_authority: Signer<'info>,

    /// CHECK: safe (the account to receive the remaining balance of the closed account)
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut)]
    pub position: Account<'info, Position>,

    #[account(mut, address = position.position_mint)]
    pub position_mint: Account<'info, Mint>,

    #[account(mut,
        constraint = position_token_account.amount == 1,
        constraint = position_token_account.mint == position.position_mint)]
    pub position_token_account: Box<Account<'info, TokenAccount>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}
