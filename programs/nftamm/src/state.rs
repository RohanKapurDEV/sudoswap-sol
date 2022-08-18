use anchor_lang::prelude::*;

#[account]
pub struct Pair {
    pub owner: Pubkey,
    pub collection_mint: Pubkey,
    pub quote_token_mint: Pubkey,
    pub quote_token_vault: Pubkey,
    pub pair_type: u8,     // 0 for TokenPair, 1 for NFTPair, 2 for TradePair
    pub bonding_curve: u8, // 0 for linear, 1 for exponential
    pub delta: u64,
    pub fee: u16,
    pub fee_vault: Pubkey,
    pub spot_price: u64,
    pub honor_royalties: bool,
    pub trade_count: u64,
    pub is_active: bool, // Set to true after first deposit has been made to the pair
    pub nfts_held: u32,
}

impl Pair {
    // Pubkeys + u64s + u32s + u8s + bools
    pub const SIZE: usize = (32 * 5) + (8 * 4) + (4 * 1) + (1 * 2) + (1 * 2);
}
