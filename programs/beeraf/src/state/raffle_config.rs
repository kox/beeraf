use anchor_lang::prelude::*;

#[account]
pub struct RaffleConfig {
    pub authority: Pubkey,
    pub collection: Pubkey,
    pub slot: u64,
    pub tickets: u32,   
    pub ticket_price: u64,
    pub raffle_fee: u64,
    pub raffle_config_bump: u8,
}

impl RaffleConfig {
    pub const INIT_SPACE:usize = 8 + 32 + 32 + 8  + 8 + 4 + 8 +  8 + 1;  
}
