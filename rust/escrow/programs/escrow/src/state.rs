use anchor_lang::prelude::*; //everytime call

#[account]
pub struct Escrow {
    pub mint_a: PublicKey,
    pub mint_b: PublicKey, 
    pub recieve: u64,
    pub bump: u8,
}

impl Space for Escrow {
    const LEN: usize = 32 + 32 + 8 + 1 + 8;
}