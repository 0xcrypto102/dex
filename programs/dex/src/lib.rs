#![allow(clippy::result_large_err)]
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod events;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;
pub use events::*;

declare_id!("BNmapsBA72Egnxd6H25UEuWmbtBKuXLD5stHQqDo6eg6");

#[program]
pub mod dex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn create_amm(ctx: Context<CreateAmm>, id: Pubkey, fee: u16) -> Result<()> {
        instructions::create_amm(ctx, id, fee)
    }

    pub fn confirm_pool(ctx: Context<ConfirmPool>) -> Result<()> {
        instructions::confirm_pool(ctx)
    }

    pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
        instructions::create_pool(ctx)
    }

    pub fn deposit_liquidity(
        ctx: Context<DepositLiquidity>,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        instructions::deposit_liquidity(ctx, amount_a, amount_b)
    }

    pub fn withdraw_liquidity_tokena(ctx: Context<WithdrawLiquidityTokenA>, amount: u64) -> Result<()> {
        instructions::withdraw_liquidity_tokena(ctx, amount)
    }
    pub fn withdraw_liquidity_tokenb(ctx: Context<WithdrawLiquidityTokenB>, amount: u64) -> Result<()> {
        instructions::withdraw_liquidity_tokenb(ctx, amount)
    }

    pub fn swap_exact_tokens_for_tokens(
        ctx: Context<SwapExactTokensForTokens>,
        swap_a: bool,
        input_amount: u64,
        min_output_amount: u64,
    ) -> Result<()> {
        instructions::swap_exact_tokens_for_tokens(ctx, swap_a, input_amount, min_output_amount)
    }
}
