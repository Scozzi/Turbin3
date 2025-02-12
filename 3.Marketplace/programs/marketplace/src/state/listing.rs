use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub nft_mint: Pubkey,
    pub nft_owner: Pubkey,
    pub price: u64,
    pub bump: u8,
}

impl Space for Listing {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}
