#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

pub mod utilities;
pub mod pool;
pub mod initialize_pool;
pub mod add_liquidity;
pub mod swap;
pub mod errors;

use initialize_pool::*;

declare_id!("HoTFuDYDyVkeug3eyZ6Xx4gsjxkG3qa6NZLXqzGtYHge");

#[program]
pub mod constant_product_amm {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        initialize_pool_handler(ctx)
    }
}
