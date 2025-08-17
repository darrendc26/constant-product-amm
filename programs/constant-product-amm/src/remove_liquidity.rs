use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, transfer, Transfer, Burn, burn};
use anchor_spl::associated_token::AssociatedToken;
use crate::pool::Pool;
use crate::utilities::remove_liquidity_helper;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"pool".as_ref(),
            pool.token_a.as_ref(),
            pool.token_b.as_ref()
        ],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,
    pub token_a: Account<'info, TokenAccount>,
    pub token_b: Account<'info, TokenAccount>,
    pub token_a_vault: Account<'info, TokenAccount>,
    pub token_b_vault: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_a_mint,
        associated_token::authority = user
    )]
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_b_mint,
        associated_token::authority = user
    )]
    pub user_token_b: Account<'info, TokenAccount>,
    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_lp: Account<'info, TokenAccount>,
    pub lp_token_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn remove_liquidity_handler(ctx: Context<RemoveLiquidity>, lp_tokens: u64) -> Result<()> {
    let (amount_a, amount_b) = remove_liquidity_helper(&mut ctx.accounts.pool, lp_tokens);

      let pool = &mut ctx.accounts.pool;
    let seeds = &[
        b"pool".as_ref(),
        pool.token_a.as_ref(),
        pool.token_b.as_ref(),
        &[pool.bump]
    ];
    let signer = &[&seeds[..]];

    // Burn user's LP tokens
    let cpi_accounts = Burn {
    mint: ctx.accounts.lp_token_mint.to_account_info(),
    from: ctx.accounts.user_lp.to_account_info(),
    authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    burn(cpi_ctx, lp_tokens)?;

    // Transfer token A to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.token_a_vault.to_account_info(),
        to: ctx.accounts.user_token_a.to_account_info(),
        authority: pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    transfer(cpi_ctx, amount_a)?;

    // Transfer token B to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.token_b_vault.to_account_info(),
        to: ctx.accounts.user_token_b.to_account_info(),
        authority: pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    transfer(cpi_ctx, amount_b)?;

    // After successful burn and transfers:
    pool.total_a_token = pool.total_a_token.checked_sub(amount_a).ok_or(ErrorCode::MathOverflow)?;
    pool.total_b_token = pool.total_b_token.checked_sub(amount_b).ok_or(ErrorCode::MathOverflow)?;
    pool.total_lp = pool.total_lp.checked_sub(lp_tokens).ok_or(ErrorCode::MathOverflow)?;
    pool.k = pool.total_a_token.checked_mul(pool.total_b_token).ok_or(ErrorCode::MathOverflow)?;

    Ok(())
}
