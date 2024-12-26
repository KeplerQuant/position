pub mod errors;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("GMCkSz2KK2oS7aiBMsLmHNXCoN97YriUXBy2Q1EFFVad");

#[program]
pub mod position {
    use super::*;

    pub fn adjust_position(ctx: Context<AdjustPosition>) -> Result<()> {
        return instructions::adjust_position::handler(ctx);
    }
}
