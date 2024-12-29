use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use raydium_amm_v3::{
    program::AmmV3,
    states::{PersonalPositionState, POSITION_SEED},
};

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    pub clmm_program: Program<'info, AmmV3>,

    /// The position nft owner
    #[account(mut)]
    pub nft_owner: Signer<'info>,

    /// Mint address bound to the personal position.
    #[account(
      mut,
      address = personal_position.nft_mint,
      mint::token_program = token_program,
    )]
    pub position_nft_mint: Box<InterfaceAccount<'info, Mint>>,

    /// User token account where position NFT be minted to
    #[account(
        mut,
        token::mint = position_nft_mint,
        token::authority = nft_owner,
        constraint = position_nft_account.amount == 1,
        token::token_program = token_program,
    )]
    pub position_nft_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [POSITION_SEED.as_bytes(), position_nft_mint.key().as_ref()],
        bump,
        close = nft_owner
    )]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    /// System program to close the position state account
    pub system_program: Program<'info, System>,

    /// Token/Token2022 program to close token/mint account
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ClosePosition<'info>>,
) -> Result<()> {
    let program = ctx.accounts.clmm_program.to_account_info();
    let remaining_accounts = ctx.remaining_accounts.to_vec();

    let cpi_accounts = raydium_amm_v3::cpi::accounts::ClosePosition {
        nft_owner: ctx.accounts.nft_owner.to_account_info(),
        position_nft_mint: ctx.accounts.position_nft_mint.to_account_info(),
        position_nft_account: ctx.accounts.position_nft_account.to_account_info(),
        personal_position: ctx.accounts.personal_position.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };
    let cpi_context =
        CpiContext::new(program, cpi_accounts).with_remaining_accounts(remaining_accounts);

    raydium_amm_v3::cpi::close_position(cpi_context)?;

    Ok(())
}
