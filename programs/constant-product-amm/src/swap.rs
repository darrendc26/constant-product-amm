use anchor_lang::prelude::*;
use anchor_spl::token::{ Transfer, TokenAccount, transfer, Token};
use crate::pool::Pool;
use crate::utilities::swap_helper;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct Swap<'info> {
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
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn swap_handler(ctx: Context<Swap>, amount_in: u64, a_to_b : bool) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    let amount_out = swap_helper(pool, amount_in, a_to_b);
    if a_to_b {
        require!(amount_in > 0, ErrorCode::MathOverflow);
        require!(amount_out > 0, ErrorCode::MathOverflow);
        require!(amount_out <= pool.total_b_token, ErrorCode::InsufficientAmount);


        // Transfer token A to vault
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_a.to_account_info(),
            to: ctx.accounts.token_a_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount_in)?;

        // Transfer token B to user
            let seeds = &[
                b"pool".as_ref(),
                pool.token_a.as_ref(),
                pool.token_b.as_ref(),
                &[pool.bump]
            ];
            let signer = &[&seeds[..]];

            let cpi_accounts = Transfer {
                from: ctx.accounts.token_b_vault.to_account_info(),
                to: ctx.accounts.user_token_b.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            transfer(cpi_ctx, amount_out)?;

            pool.total_a_token = pool.total_a_token.checked_add(amount_in).unwrap();
            pool.total_b_token = pool.total_b_token.checked_sub(amount_out).unwrap();
        }
        else {
            require!(amount_out <= pool.total_a_token, ErrorCode::InsufficientAmount);
            require!(amount_in > 0, ErrorCode::MathOverflow);
            require!(amount_out > 0, ErrorCode::MathOverflow);
            // Transfer token B to vault
            let cpi_accounts = Transfer {
                from: ctx.accounts.user_token_b.to_account_info(),
                to: ctx.accounts.token_b_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            transfer(cpi_ctx, amount_in)?;

            // Transfer token A to user
            let seeds = &[
                b"pool".as_ref(),
                pool.token_a.as_ref(),
                pool.token_b.as_ref(),
                &[pool.bump]
            ];
            let signer = &[&seeds[..]];

            let cpi_accounts = Transfer {
                from: ctx.accounts.token_a_vault.to_account_info(),
                to: ctx.accounts.user_token_a.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            transfer(cpi_ctx, amount_out)?; 

            pool.total_a_token = pool.total_a_token.checked_sub(amount_in).unwrap();
            pool.total_b_token = pool.total_b_token.checked_add(amount_out).unwrap();
        }
            Ok(())
        }  
        