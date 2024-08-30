use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub fee: u64,
    pub config_bump: u8,
    pub treasury_bump: u8,    
}

impl Config {
    pub const INIT_SPACE:usize = 8 + 32 + 8  + 1 + 1;  
}
