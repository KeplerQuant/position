use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{self, Token2022},
    token_interface::{Mint, TokenAccount},
};
use whirlpool_cpi::{program::Whirlpool as WhirlpoolProgram, state::Position};

#[derive(Accounts)]
pub struct AdjustPosition<'info> {
    pub whirlpool_program: Program<'info, WhirlpoolProgram>,

    pub position_authority: Signer<'info>,

    /// CHECK: safe, for receiving rent only
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut)]
    pub position: Account<'info, Position>,

    #[account(mut, address = position.position_mint, owner = token_2022_program.key())]
    pub position_mint: InterfaceAccount<'info, Mint>,

    #[account(mut,
        constraint = position_token_account.amount == 1,
        constraint = position_token_account.mint == position.position_mint
    )]
    pub position_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(address = token_2022::ID)]
    pub token_2022_program: Program<'info, Token2022>,
}

pub fn handler(ctx: Context<AdjustPosition>) -> Result<()> {
    let cpi_program = ctx.accounts.whirlpool_program.to_account_info();

    let cpi_accounts = whirlpool_cpi::cpi::accounts::ClosePositionWithTokenExtensions {
        position_authority: ctx.accounts.position_authority.to_account_info(),
        receiver: ctx.accounts.receiver.to_account_info(),
        position: ctx.accounts.position.to_account_info(),
        position_mint: ctx.accounts.position_mint.to_account_info(),
        position_token_account: ctx.accounts.position_token_account.to_account_info(),
        token_2022_program: ctx.accounts.token_2022_program.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);

    msg!("CPI: whirlpool close_position_with_token_extensions instruction");
    whirlpool_cpi::cpi::close_position_with_token_extensions(cpi_ctx)?;

    Ok(())
}
