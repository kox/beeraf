use anchor_lang::{prelude::*, solana_program::address_lookup_table::instruction};

use crate::{raffle_config, Config, RaffleConfig};

#[derive(Accounts)]
#[instruction(seed: u64)]
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

    // We will need to create the raffle config using a seed to the maker can create more than 1 raflle
    #[account(
        init_if_needed,
        payer = maker,
        seeds = [b"raffle_config", house.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = RaffleConfig::INIT_SPACE,
        bump,
    )]
    pub raffle_config: Account<'info, RaffleConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateRaffle<'info> {
    pub fn create_raffle(&mut self, seed: u64, ticket_price: u64, raffle_fee: u64, bumps: &CreateRaffleBumps) -> Result<()> {
        let slot = Clock::get()?.epoch + 1_512_000;

        self.raffle_config.set_inner(RaffleConfig {
            authority: self.maker.key(),
            slot,
            seed,
            raffle_fee,
            tickets: 0u32,
            ticket_price,
            raffle_config_bump: bumps.raffle_config,
        });
        
        Ok(())
    }
    
}