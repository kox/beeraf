use anchor_lang::prelude::*;

#[account]
pub struct RaffleConfig {
    pub authority: Pubkey,
    pub collection: Pubkey,
    pub slot: u64,
    pub ticket_price: u64,
    pub raffle_fee: u64,
    pub raffle_config_bump: u8,
}

impl RaffleConfig {
    pub const INIT_SPACE:usize = 8 + 32 + 32 + 8  + 8 + 4 + 8 +  8 + 1;  

    pub fn to_slice(&self) -> Vec<u8> {
        let mut info = self.authority.to_bytes().to_vec();

        info.extend_from_slice(&self.collection.to_bytes());
        info.extend_from_slice(&self.slot.to_le_bytes());
        info.extend_from_slice(&self.ticket_price.to_le_bytes());
        info.extend_from_slice(&self.raffle_fee.to_le_bytes());
        info.extend_from_slice(&[self.raffle_config_bump]);
        
        info
    }
}

#[event]
pub struct RafEvent {
    pub winner: u32,
}