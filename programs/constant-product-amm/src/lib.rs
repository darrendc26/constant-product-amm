#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

pub mod utilities;
pub mod pool;
pub mod initialize_pool;
pub mod add_liquidity;
pub mod swap;
pub mod remove_liquidity;
pub mod errors;

use initialize_pool::*;
use add_liquidity::*;
use swap::*;

declare_id!("HoTFuDYDyVkeug3eyZ6Xx4gsjxkG3qa6NZLXqzGtYHge");

#[program]
pub mod constant_product_amm {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        initialize_pool_handler(ctx)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        add_liquidity_handler(ctx, amount_a, amount_b)
    }

    pub fn swap(ctx: Context<Swap>, amount_in: u64, a_to_b : bool) -> Result<()> {
        swap_handler(ctx, amount_in, a_to_b)
    }
}
