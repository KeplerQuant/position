pub mod errors;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("5MCHiVtTNpdfkqTT6FwK1pnUCfvaLqK73Qwqb9xHpDvG");

#[program]
pub mod position {
    use super::*;

    pub fn adjust_position(ctx: Context<AdjustPosition>) -> Result<()> {
        return instructions::adjust_position::handler(ctx);
    }
}
