use anchor_lang::prelude::*;
use mpl_core::{
    instructions::CreateCollectionV2CpiBuilder,
    ID as MPL_CORE_ID,
};
use crate::{Config, RaffleConfig};

#[derive(Accounts)]
pub struct CreateRaffle<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

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

    #[account(mut)]
    pub raffle: Signer<'info>,

    // We will need to create the raffle config using a seed to the maker can create more than 1 raflle
    #[account(
        init,
        payer = maker,
        seeds = [
            b"raffle",
            house.key().as_ref(), 
            raffle.key().as_ref(),
        ],
        space = RaffleConfig::INIT_SPACE,
        bump,
    )]
    pub raffle_config: Account<'info, RaffleConfig>,

    #[account(address = MPL_CORE_ID)]
    /// CHECK: This is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateRaffle<'info> {
    pub fn create_raffle(&mut self, args: CreateRaffleArgs, bumps: &CreateRaffleBumps) -> Result<()> {
        let slot = Clock::get()?.epoch + 1_512_000;

        // Create the Collection that will hold the tickets
        CreateCollectionV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.raffle.to_account_info())
            .update_authority(Some(&self.maker.to_account_info()))
            .payer(&self.maker.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .invoke()?;


        self.raffle_config.set_inner(RaffleConfig {
            authority: self.maker.key(),
            collection: self.raffle.key(),
            slot,
            raffle_fee: args.raffle_fee,
            tickets: 0u32,
            ticket_price: args.ticket_price,
            raffle_config_bump: bumps.raffle_config,
        });
        
        Ok(())
    }
    
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateRaffleArgs {
    pub name: String,
    pub uri: String,
    pub ticket_price: u64,
    pub raffle_fee: u64,
}