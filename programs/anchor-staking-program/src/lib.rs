use anchor_lang::prelude::*;

declare_id!("FvBdLVZv2FTBdCcNgsJCYHpQ9PCDyKVumRSQaZuSGJHi");

#[program]
pub mod anchor_staking_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
