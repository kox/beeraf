use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use mpl_core::{
    accounts::BaseCollectionV1, fetch_plugin, instructions::CreateV2CpiBuilder, types::{AppDataInitInfo, Attribute, Attributes, ExternalPluginAdapterInitInfo, ExternalPluginAdapterSchema, PermanentBurnDelegate, PermanentFreezeDelegate, PermanentTransferDelegate, Plugin, PluginAuthority, PluginAuthorityPair, PluginType}, ID as MPL_CORE_ID
};

use crate::{error::BeeRafError, BuyEvent, Config, RaffleConfig};

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: We don't make anything on this account
    pub house: UncheckedAccount<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

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

    #[account(
        mut,
        seeds = [b"vault", maker.key().as_ref()],
        bump = raffle_config.vault_bump
    )]
    vault: SystemAccount<'info>,

    #[account(address = MPL_CORE_ID)]
    /// CHECK: This is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> BuyTicket<'info> {
    pub fn buy_ticket(&mut self, args: BuyTicketArgs) -> Result<()> {
        let house = self.house.key();
        let raffle = self.raffle.key();

        let current_slot = Clock::get()?.slot;

        require!(current_slot <= self.raffle_config.slot, BeeRafError::TimeExpired);

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

        // Add an Attribute Plugin that will hold the ticket details
        let mut ticket_plugin: Vec<PluginAuthorityPair> = vec![];

        let attribute_list: Vec<Attribute> = vec![
            Attribute {
                key: "Ticket Number".to_string(),
                value: self.raffle
                    .num_minted
                    .checked_add(1)
                    .ok_or(BeeRafError::NumericalOverflow)?
                    .to_string(),
            },
        ];
        ticket_plugin.push(PluginAuthorityPair {
            plugin: Plugin::Attributes(Attributes { attribute_list }),
            authority: Some(PluginAuthority::UpdateAuthority),
        });
        ticket_plugin.push(PluginAuthorityPair {
            plugin: Plugin::PermanentFreezeDelegate(PermanentFreezeDelegate { frozen: false }),
            authority: Some(PluginAuthority::UpdateAuthority),
        });
        ticket_plugin.push(PluginAuthorityPair {
            plugin: Plugin::PermanentBurnDelegate(PermanentBurnDelegate {}),
            authority: Some(PluginAuthority::UpdateAuthority),
        });
        ticket_plugin.push(PluginAuthorityPair {
            plugin: Plugin::PermanentTransferDelegate(PermanentTransferDelegate {}),
            authority: Some(PluginAuthority::UpdateAuthority),
        });

        let ticket_external_plugin: Vec<ExternalPluginAdapterInitInfo> =
            vec![ExternalPluginAdapterInitInfo::AppData(AppDataInitInfo {
                init_plugin_authority: Some(PluginAuthority::UpdateAuthority),
                data_authority: PluginAuthority::Address {
                    address: self.raffle_config.key(),
                },
                schema: Some(ExternalPluginAdapterSchema::Binary),
            })];

        let signer_seeds = &[
            b"raffle".as_ref(),
            house.as_ref(), 
            raffle.as_ref(), 
            &[self.raffle_config.raffle_config_bump]
        ];

        // Create the Ticket
        CreateV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.ticket.to_account_info())
            .collection(Some(&self.raffle.to_account_info()))
            .payer(&self.buyer.to_account_info())
            .authority(Some(&self.raffle_config.to_account_info()))
            .owner(Some(&self.buyer.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .plugins(ticket_plugin)
            .external_plugin_adapters(ticket_external_plugin)
            .invoke_signed(&[signer_seeds])?;

        let maker_fee = (self.raffle_config.ticket_price * self.raffle_config.raffle_fee) / 10_000;
                let vault_earning = self.raffle_config.ticket_price - maker_fee;

        emit!(BuyEvent {
            maker_fee,
            vault_earning,
        });
        msg!("maker_fee: {}", maker_fee);
        msg!("vault_earning: {}", vault_earning);

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, maker_fee)?;

        let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_program = self.system_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, vault_earning)?;

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct BuyTicketArgs {
    pub name: String,
    pub uri: String,
}
