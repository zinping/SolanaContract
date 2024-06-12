use anchor_lang::prelude::*;

#[account]
pub struct TokenState {
    pub token_total_supply: u64,
    pub owner: Pubkey,
    pub reward_address: Pubkey,
}