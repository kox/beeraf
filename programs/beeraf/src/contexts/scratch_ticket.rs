use anchor_lang::prelude::*;
use mpl_core::{accounts::{BaseAssetV1, BaseCollectionV1}, types::UpdateAuthority, ID as MPL_CORE_ID};

use crate::{Config, RaffleConfig};

#[derive(Accounts)]
pub struct ScratchTicket<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: We don't make anything on this account
    pub house: UncheckedAccount<'info>,

    #[account(
        seeds = [b"treasury", house.key().as_ref()],
        bump = config.treasury_bump
    )]
    treasury: SystemAccount<'info>,

    #[account(
        seeds = [b"config", treasury.key().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [
            b"raffle",
            house.key().as_ref(), 
            raffle.key().as_ref(),
        ],
        bump = raffle_config.raffle_config_bump
    )]
    pub raffle_config: Account<'info, RaffleConfig>,

    #[account(
        mut,
        constraint = raffle.update_authority == raffle_config.key(),
    )]
    pub raffle: Account<'info, BaseCollectionV1>,
    
    #[account(
        mut,
        constraint = ticket.owner == buyer.key(),
        constraint = ticket.update_authority == UpdateAuthority::Collection(raffle.key()),
    )]
    pub ticket: Account<'info, BaseAssetV1>,
    
    #[account(address = MPL_CORE_ID)]
    /// CHECK: This is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> ScratchTicket<'info> {
    pub fn scratch_ticket(&mut self) -> Result<()> {

        
        Ok(())
    }
    
}