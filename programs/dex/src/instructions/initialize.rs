use anchor_lang::prelude::*;
use crate::GlobalState;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let accts = ctx.accounts;
    accts.global_state.owner = accts.owner.key();
    accts.global_state.pools = Vec::new();

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [b"global-state"],
        bump,
        space = 2000
    )]
    pub global_state: Account<'info, GlobalState>,

    pub system_program: Program<'info, System>
}