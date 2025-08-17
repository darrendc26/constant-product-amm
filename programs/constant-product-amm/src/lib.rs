#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

pub mod pool;

declare_id!("HoTFuDYDyVkeug3eyZ6Xx4gsjxkG3qa6NZLXqzGtYHge");

#[program]
pub mod constant_product_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
