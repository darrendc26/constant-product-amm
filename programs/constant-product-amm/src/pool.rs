use anchor_lang::prelude::*;

    #[account]
    #[derive(InitSpace)]
    pub struct Pool {
    pub authority: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub fee_vault: Pubkey,
    pub total_lp: u64,
    pub fee: u64,
    pub k: u64,
    pub bump: u8,
}