use crate::pool::Pool;

pub fn add_liquidity_helper(pool: &mut Pool, amount_a: u64, amount_b: u64) -> (u64, u64, u64) {
    if pool.k == 0 {
        // Initial liquidity sets pool reserves and LP mint amount using geometric mean
        let liquidity = ((amount_a as u128).checked_mul(amount_b as u128).unwrap() as f64).sqrt() as u64;
        return (amount_a, amount_b, liquidity);
    }

    let amount_b_optimal = amount_a.checked_mul(pool.total_b_token).unwrap() / pool.total_a_token;
    if amount_b_optimal <= amount_b {
        let liquidity = (amount_a as u128)
            .checked_mul(pool.total_lp as u128)
            .unwrap()
            .checked_div(pool.total_a_token as u128)
            .unwrap() as u64;
        return (amount_a, amount_b_optimal, liquidity);
    } else {
        let amount_a_optimal = amount_b.checked_mul(pool.total_a_token).unwrap() / pool.total_b_token;
        let liquidity = (amount_b as u128)
            .checked_mul(pool.total_lp as u128)
            .unwrap()
            .checked_div(pool.total_b_token as u128)
            .unwrap() as u64;
        return (amount_a_optimal, amount_b, liquidity);
    }
}
