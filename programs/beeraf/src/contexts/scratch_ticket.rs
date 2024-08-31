use anchor_lang::prelude::*;
use mpl_core::{accounts::{BaseAssetV1, BaseCollectionV1}, fetch_plugin, types::{Attributes, PluginType, UpdateAuthority}, ID as MPL_CORE_ID};

use crate::{error::BeeRafError, Config, RaffleConfig, WinnerEvent};

#[event_cpi]
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
        /* constraint = ticket.owner == buyer.key(),
        constraint = ticket.update_authority == UpdateAuthority::Collection(raffle.key()),
 */    )]
    pub ticket: Signer<'info/* , BaseAssetV1 */>,
    
    #[account(address = MPL_CORE_ID)]
    /// CHECK: This is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> ScratchTicket<'info> {
    pub fn scratch_ticket(&mut self) -> Result<(u32 , u32)> {
        // Check that the maximum number of tickets has not been reached yet
        let (_, collection_attribute_list, _) = fetch_plugin::<BaseCollectionV1, Attributes>(
            &self.raffle.to_account_info(),
            PluginType::Attributes,
        )?;

        // Search for the Winner attribute
        let winner_attribute = collection_attribute_list
        .attribute_list
        .iter()
        .find(|attr| attr.key == "Winner")
        .ok_or(BeeRafError::MissingWinnerAttribute)?;

        // Unwrap the Winner attribute value
        let winner = winner_attribute
        .value
        .parse::<u32>()
        .map_err(|_| BeeRafError::NumericalOverflow)?;

        // Check that the maximum number of tickets has not been reached yet
        let (_, ticket_attribute_list, _) = fetch_plugin::<BaseAssetV1, Attributes>(
            &self.ticket.to_account_info(),
            PluginType::Attributes,
        )?;

        // Search for the Capacity attribute
        let ticket_number_attribute = ticket_attribute_list
        .attribute_list
        .iter()
        .find(|attr| attr.key == "Ticket Number")
        .ok_or(BeeRafError::MissingWinnerAttribute)?;

        // Unwrap the Winner attribute value
        let ticket_number = ticket_number_attribute
        .value
        .parse::<u32>()
        .map_err(|_| BeeRafError::NumericalOverflow)?;

        // emit_cpi!(WinnerEvent { winner, ticket: ticket_number }  );

        /* require!(ticket_number == winner, BeeRafError::NoWinner);
        */
        msg!("ticket number: {}", ticket_number);
        msg!("winner: {}", winner);

        // you are the winner

        Ok((winner, ticket_number))
    }
    
}