pub mod errors;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Dk1TMwYtjVEEZsovJrtun34hae6Vw1mqhFMjgZ8zkmQ7");

#[program]
pub mod position {
    use super::*;

    pub fn adjust_position(ctx: Context<AdjustPosition>) -> Result<()> {
        return instructions::adjust_position::handler(ctx);
    }
}
