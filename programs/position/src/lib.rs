pub mod errors;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("EvvVHCu51jPjxJv7nHipFcWkCL3PxL22rbGg4B7BYayu");

#[program]
pub mod position {
    use super::*;

    pub fn adjust_position(ctx: Context<AdjustPosition>) -> Result<()> {
        return instructions::adjust_position::handler(ctx);
    }
}
