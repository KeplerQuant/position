use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
};
use whirlpool_cpi::{
    program::Whirlpool as WhirlpoolProgram,
    state::{Position, Whirlpool},
};

#[derive(Accounts)]
pub struct AdjustPosition<'info> {
    pub whirlpool_program: Program<'info, WhirlpoolProgram>,

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

    pub whirlpool: Box<Account<'info, Whirlpool>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(
    ctx: Context<AdjustPosition>,
    tick_lower_index: i32,
    tick_upper_index: i32,
) -> Result<()> {
    let cpi_program = ctx.accounts.whirlpool_program.to_account_info();

    let cpi_accounts = whirlpool_cpi::cpi::accounts::ClosePosition {
        position_authority: ctx.accounts.position_authority.to_account_info(),
        receiver: ctx.accounts.receiver.to_account_info(),
        position: ctx.accounts.position.to_account_info(),
        position_mint: ctx.accounts.position_mint.to_account_info(),
        position_token_account: ctx.accounts.position_token_account.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);

    msg!("CPI: whirlpool close_position instruction");
    whirlpool_cpi::cpi::close_position(cpi_ctx)?;

    let cpi_accounts = whirlpool_cpi::cpi::accounts::OpenPosition {
        funder: ctx.accounts.position_authority.to_account_info(),
        owner: ctx.accounts.position_authority.to_account_info(),
        position: ctx.accounts.position.to_account_info(),
        position_mint: ctx.accounts.position_mint.to_account_info(),
        position_token_account: ctx.accounts.position_token_account.to_account_info(),
        whirlpool: ctx.accounts.whirlpool.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    msg!("CPI: whirlpool open_position instruction");
    whirlpool_cpi::cpi::open_position(
        cpi_ctx,
        whirlpool_cpi::state::OpenPositionBumps { position_bump: 0 }, // passed bump is no longer used
        tick_lower_index,
        tick_upper_index,
    )?;

    Ok(())
}
