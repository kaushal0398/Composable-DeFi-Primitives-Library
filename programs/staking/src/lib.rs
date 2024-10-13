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

    pub fn stake_with_time_lock(ctx: Context<Stake>, amount: u64, lock_duration: u64) -> Result<()> {
        let staking = &mut ctx.accounts.staking;
        let user_stake = &mut ctx.accounts.user_stake;

        token::transfer(ctx.accounts.transfer_to_staking_ctx(), amount)?;

        user_stake.amount += amount;
        user_stake.lock_end_time = Clock::get()?.unix_timestamp + lock_duration as i64;
        staking.total_staked += amount;

        update_rewards(staking, user_stake)?;

        Ok(())
    }

    pub fn unstake_with_time_lock(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        let user_stake = &mut ctx.accounts.user_stake;

        let current_time = Clock::get()?.unix_timestamp;
        if current_time < user_stake.lock_end_time {
            return Err(ErrorCode::LockPeriodNotEnded.into());
        }
