use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Mint, Token, Transfer};

declare_id!("");

pub mod staking {
    use super::*;

    pub fn initialize_staking(ctx: Context<InitializeStaking>, staking_bump: u8) -> Result<()> {
        let staking = &mut ctx.accounts.staking;
        staking.bump = staking_bump;
        staking.total_staked = 0;
        staking.reward_rate = 100;
        staking.last_update_time = Clock::get()?.unix_timestamp;

        Ok(())
    }

