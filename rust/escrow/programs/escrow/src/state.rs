use anchor_lang::prelude::*; //everytime call


#[account]
pub struct Escrow {
    pub mint_a: Pubkey,
    pub mint_b: Pubkey, 
    pub recieve: u64,
    pub bump: u8,
}

impl Space for Escrow {
    const INIT_SPACE: usize = 32 + 32 + 8 + 1 + 8;
}