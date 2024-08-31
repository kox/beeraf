use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use mpl_core::{
    instructions::CreateCollectionV2CpiBuilder, types::{Attribute, Attributes, Plugin, PluginAuthority, PluginAuthorityPair}, ID as MPL_CORE_ID
};
use crate::{Config, RaffleConfig};

#[derive(Accounts)]
pub struct CreateRaffle<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    /// CHECK: We don't make anything on this account
    pub house: UncheckedAccount<'info>,

    #[account(
        mut,
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

    #[account(
        seeds = [b"vault", maker.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,

    #[account(address = MPL_CORE_ID)]
    /// CHECK: This is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateRaffle<'info> {
    pub fn create_raffle(&mut self, args: CreateRaffleArgs, bumps: &CreateRaffleBumps) -> Result<()> {
        let slot = Clock::get()?.slot + args.slot_interval; //+ 1_512_000;

        // Add an Attribute Plugin that will hold the event details
        let mut collection_plugin: Vec<PluginAuthorityPair> = vec![];

        let attribute_list: Vec<Attribute> = vec![
            Attribute {
                key: "Capacity".to_string(),
                value: 1000.to_string() // args.capacity.to_string(),
            },
        ];
        
        collection_plugin.push(PluginAuthorityPair {
            plugin: Plugin::Attributes(Attributes { attribute_list }),
            authority: Some(PluginAuthority::UpdateAuthority),
        });

        // Create the Collection that will hold the tickets
        CreateCollectionV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.raffle.to_account_info())
            .update_authority(Some(&self.raffle_config.to_account_info()))
            .payer(&self.maker.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .plugins(collection_plugin)
            .invoke()?;


        self.raffle_config.set_inner(RaffleConfig {
            authority: self.maker.key(),
            collection: self.raffle.key(),
            slot,
            raffle_fee: args.raffle_fee,
            ticket_price: args.ticket_price,
            raffle_config_bump: bumps.raffle_config,
            vault_bump: bumps.vault,
        });

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.maker.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, self.config.fee)?;
        
        Ok(())
    }
    
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateRaffleArgs {
    pub name: String,
    pub uri: String,
    pub ticket_price: u64,
    pub raffle_fee: u64,
    pub slot_interval: u64,
}
