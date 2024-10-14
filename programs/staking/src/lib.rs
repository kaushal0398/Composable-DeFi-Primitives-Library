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

        token::transfer(ctx.accounts.transfer_from_staking_ctx(), amount)?;

        user_stake.amount -= amount;
        staking.total_staked -= amount;
        update_rewards(staking, user_stake)?;

        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let staking = &mut ctx.accounts.staking;
        let user_stake = &mut ctx.accounts.user_stake;

        update_rewards(staking, user_stake)?;

        Ok(())
    }
}

fn update_rewards(staking: &mut Staking, user_stake: &mut UserStake) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let time_elapsed = current_time - staking.last_update_time;

    if staking.total_staked > 0 {
        let new_rewards = user_stake.amount as u128 * time_elapsed as u128 * staking.reward_rate as u128 / staking.total_staked as u128;
        user_stake.pending_rewards += new_rewards as u64;
    }

    staking.last_update_time = current_time;
    Ok(())
}

pub struct Staking {
    pub bump: u8,
    pub total_staked: u64,
    pub reward_rate: u64,
    pub last_update_time: u64,
}

pub struct UserStake {
    pub amount: u64,
    pub pending_rewards: u64,
    pub lock_end_time: i64,
}

pub enum ErrorCode {
    #[msg("Staking lock period has not ended.")]
    LockPeriodNotEnded,
}
