use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Math Overflow")]
    MathOverflow,
    #[msg("Insufficient Amount")]
    InsufficientAmount,
    #[msg("Invalid Swap Amount")]
    InvalidSwapAmount,
}