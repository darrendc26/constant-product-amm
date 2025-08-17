use crate::pool::Pool;

pub fn remove_liquidity_helper(pool: &mut Pool, lp_tokens: u64) -> (u64, u64) {
    let amount_a = pool.total_a_token.checked_mul(lp_tokens).unwrap() / pool.total_lp;
    let amount_b = pool.total_b_token.checked_mul(lp_tokens).unwrap() / pool.total_lp;
    return (amount_a, amount_b);
}