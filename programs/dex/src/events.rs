use anchor_lang::prelude::*;

#[event]
pub struct DepositLiquidityEvent {
    #[index]
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64
}
