use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer, Mint, Token};

declare_id!("");

pub mod liquidity_pool {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, pool_bump: u8) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.bump = pool_bump;
        pool.token_a_reserve = 0;
        pool.token_b_reserve = 0;
        pool.total_lp_supply = 0;

        Ok(())
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        token::transfer(ctx.accounts.transfer_a_ctx(), amount_a)?;
        pool.token_a_reserve += amount_a;

        token::transfer(ctx.accounts.transfer_b_ctx(), amount_b)?;
        pool.token_b_reserve += amount_b;

        let lp_amount = calculate_lp_tokens(pool, amount_a, amount_b);
        pool.total_lp_supply += lp_amount;

        token::mint_to(ctx.accounts.lp_mint_to_ctx(), lp_amount)?;

        Ok(())
    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, lp_amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        token::burn(ctx.accounts.lp_burn_ctx(), lp_amount)?;

        let amount_a = (lp_amount as u128 * pool.token_a_reserve as u128 / pool.total_lp_supply as u128) as u64;
        let amount_b = (lp_amount as u128 * pool.token_b_reserve as u128 / pool.total_lp_supply as u128) as u64;

        pool.token_a_reserve -= amount_a;
        pool.token_b_reserve -= amount_b;
        pool.total_lp_supply -= lp_amount;

        token::transfer(ctx.accounts.transfer_to_user_a_ctx(), amount_a)?;
        token::transfer(ctx.accounts.transfer_to_user_b_ctx(), amount_b)?;

        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, amount_in: u64, is_token_a_to_b: bool) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let (reserve_in, reserve_out) = if is_token_a_to_b {
            (pool.token_a_reserve, pool.token_b_reserve)
        } else {
            (pool.token_b_reserve, pool.token_a_reserve)
        };

        let amount_out = calculate_swap_amount(reserve_in, reserve_out, amount_in);
        if is_token_a_to_b {
            pool.token_a_reserve += amount_in;
            pool.token_b_reserve -= amount_out;
        } else {
            pool.token_b_reserve += amount_in;
            pool.token_a_reserve -= amount_out;
        }

        token::transfer(ctx.accounts.transfer_swap_out_ctx(), amount_out)?;

        Ok(())
    }
}

fn calculate_lp_tokens(pool: &Pool, amount_a: u64, amount_b: u64) -> u64 {
    if pool.total_lp_supply == 0 {
        amount_a + amount_b
    } else {
        (amount_a * pool.total_lp_supply / pool.token_a_reserve).min(amount_b * pool.total_lp_supply / pool.token_b_reserve)
    }
}

fn calculate_swap_amount(reserve_in: u64, reserve_out: u64, amount_in: u64) -> u64 {
    let amount_in_with_fee = amount_in * 997 / 1000;
    let numerator = amount_in_with_fee * reserve_out;
    let denominator = reserve_in + amount_in_with_fee;
    numerator / denominator
}

#[account]
pub struct Pool {
    pub bump: u8,
    pub token_a_reserve: u64,
    pub token_b_reserve: u64,
    pub total_lp_supply: u64,
}
