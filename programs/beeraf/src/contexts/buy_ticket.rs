use anchor_lang::prelude::*;
use mpl_core::{
    accounts::BaseCollectionV1, fetch_plugin, instructions::CreateCollectionV2CpiBuilder, types::{Attributes, PluginType}, ID as MPL_CORE_ID
};

use crate::{error::BeeRafError, Config, RaffleConfig};

#[derive(Accounts)]
pub struct BuyTicket<'info> {
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
        mut,
        constraint = raffle.update_authority == raffle_config.key(),
    )]
    pub raffle: Account<'info, BaseCollectionV1>,

    #[account(
        seeds = [
            b"raffle",
            house.key().as_ref(), 
            raffle.key().as_ref(),
        ],
        bump = raffle_config.raffle_config_bump
    )]
    pub raffle_config: Account<'info, RaffleConfig>,

    #[account(mut)]
    pub ticket: Signer<'info>,

    #[account(address = MPL_CORE_ID)]
    /// CHECK: This is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> BuyTicket<'info> {
    pub fn buy_ticket(&mut self, args: BuyTicketArgs) -> Result<()> {
         // Check that the maximum number of tickets has not been reached yet
         let (_, collection_attribute_list, _) = fetch_plugin::<BaseCollectionV1, Attributes>(
            &self.raffle.to_account_info(),
            PluginType::Attributes,
        )?;

         // Search for the Capacity attribute
        let capacity_attribute = collection_attribute_list
        .attribute_list
        .iter()
        .find(|attr| attr.key == "Capacity")
        .ok_or(BeeRafError::MissingAttribute)?;

        // Unwrap the Capacity attribute value
        let capacity = capacity_attribute
        .value
        .parse::<u32>()
        .map_err(|_| BeeRafError::NumericalOverflow)?;

        require!(
            self.raffle.num_minted < capacity,
            BeeRafError::MaximumTicketsReached
        );
        
        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct BuyTicketArgs {
    pub name: String,
    pub uri: String,
}
