use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::pool::Pool;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + Pool::INIT_SPACE,
        seeds = [
            b"pool".as_ref(),
            authority.key().as_ref(),
            token_a.key().as_ref(),
            token_b.key().as_ref()
        ],
        bump
    )]
    pub pool: Account<'info, Pool>,
    pub token_a: Account<'info, TokenAccount>,
    pub token_b: Account<'info, TokenAccount>,
    pub token_a_vault: Account<'info, TokenAccount>,
    pub token_b_vault: Account<'info, TokenAccount>,
    pub fee_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn initialize_pool_handler(ctx: Context<InitializePool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.authority = *ctx.accounts.authority.key;
    pool.token_a = ctx.accounts.token_a.key();
    pool.token_b = ctx.accounts.token_b.key();
    pool.token_a_vault = ctx.accounts.token_a_vault.key();
    pool.token_b_vault = ctx.accounts.token_b_vault.key();
    pool.fee_vault = ctx.accounts.fee_vault.key();
    pool.total_lp = 0;
    pool.fee = 0;
    pool.k = 0;
    pool.bump = ctx.bumps.pool;
    Ok(())
}
