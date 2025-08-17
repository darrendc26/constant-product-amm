use crate::pool::Pool;

pub fn swap_helper(pool: &mut Pool, amount_in: u64, a_to_b : bool) ->  u64 {
    let fee_denominator = 10_000u64;
    let fee = pool.fee; 
    let amount_in_with_fee = amount_in
        .checked_mul(fee_denominator - fee)
        .unwrap()
        .checked_div(fee_denominator)
        .unwrap();


    let amount_out: u64 = if a_to_b {
        let new_a = amount_in_with_fee.checked_add(pool.total_a_token).unwrap();
        let new_b = pool.k.checked_div(new_a).unwrap();
        pool.total_b_token.checked_sub(new_b).unwrap()
    }
    else {
        let new_b = amount_in_with_fee.checked_add(pool.total_b_token).unwrap();
        let new_a = pool.k.checked_div(new_b).unwrap();
        pool.total_a_token.checked_sub(new_a).unwrap()
    };
return amount_out;
}