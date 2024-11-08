use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    // the mint being offered
    pub mint_a: Pubkey,
    // the mint asked in return
    pub mint_b: Pubkey,
    // the amount to get in return
    pub receive: u64,
    pub bump: u8,
}

// impl Space for Escrow {
//     const INIT_SPACE: usize = 8 + 8 + 32 + 32 + 32 + 8 + 1;
// }