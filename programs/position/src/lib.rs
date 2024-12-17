pub mod errors;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("EvvVHCu51jPjxJv7nHipFcWkCL3PxL22rbGg4B7BYayu");

#[program]
pub mod position {
    use super::*;

    pub fn adjust_position(
        ctx: Context<AdjustPosition>,
        tick_lower_index: i32,
        tick_upper_index: i32,
    ) -> Result<()> {
        return instructions::adjust_position::handler(ctx, tick_lower_index, tick_upper_index);
    }
}
