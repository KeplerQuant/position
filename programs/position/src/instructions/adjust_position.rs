use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AdjustPosition {}

pub fn handler(_ctx: Context<AdjustPosition>) -> Result<()> {
    msg!("CPI: whirlpool close_position_with_token_extensions instruction");

    Ok(())
}
