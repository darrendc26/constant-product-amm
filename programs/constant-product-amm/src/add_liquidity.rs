use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, transfer, Transfer, mint_to, MintTo};
use anchor_spl::associated_token::AssociatedToken;
use crate::pool::Pool;
use crate::utilities::add_liquidity_helper;

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
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
    pub fee_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = lp_token_mint,
        associated_token::authority = user
    )]
    pub user_lp: Account<'info, TokenAccount>,
    pub lp_token_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn add_liquidity_handler(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    let (amount_a, amount_b, lp_tokens) = add_liquidity_helper(pool, amount_a, amount_b);

    let seeds = &[
        b"pool".as_ref(),
        pool.token_a.as_ref(),
        pool.token_b.as_ref(),
        &[pool.bump]
    ];
    let signer = &[&seeds[..]];

    // Transfer user's token A to vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.token_a.to_account_info(),
        to: ctx.accounts.user_token_a.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer(cpi_ctx, amount_a)?;

    // Transfer user's token B to vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_b.to_account_info(),
        to: ctx.accounts.token_b_vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer(cpi_ctx, amount_b)?;


    // Mint user's LP tokens
    let cpi_accounts = MintTo {
        mint: ctx.accounts.lp_token_mint.to_account_info(),
        to: ctx.accounts.user_lp.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    mint_to(cpi_ctx, lp_tokens)?;

    Ok(())
}