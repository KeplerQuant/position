pub mod errors;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("GMCkSz2KK2oS7aiBMsLmHNXCoN97YriUXBy2Q1EFFVad");

#[program]
pub mod position {
    use super::*;

    pub fn close_position<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ClosePosition<'info>>,
    ) -> Result<()> {
        return instructions::close_position::handler(ctx);
    }
}
