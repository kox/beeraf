use anchor_lang::prelude::*;
use solana_program::{sysvar::instructions::load_instruction_at_checked, ed25519_program, hash::hash};
use mpl_core::{ accounts::BaseCollectionV1, fetch_plugin, instructions::{UpdateCollectionPluginV1, UpdateCollectionPluginV1Cpi, UpdateCollectionPluginV1CpiBuilder}, types::{Attribute, Attributes, PluginType}, ID as MPL_CORE_ID };
use anchor_instruction_sysvar::Ed25519InstructionSignatures;

use crate::{error::BeeRafError, Config, RafEvent, RaffleConfig};

#[derive(Accounts)]
pub struct SolveRaffle<'info> {
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

    #[account(address = MPL_CORE_ID)]
    /// CHECK: This is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    /// CHECK: This is safe
    pub instruction_sysvar: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> SolveRaffle<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        // Get the Ed25519 signature instruction 
        let ix = load_instruction_at_checked(
            0, 
            &self.instruction_sysvar.to_account_info()
        )?;
        // Make sure the instruction is addressed to the ed25519 program
        require_keys_eq!(ix.program_id, ed25519_program::ID, BeeRafError::Ed25519Program);
        // Make sure there are no accounts present
        require_eq!(ix.accounts.len(), 0, BeeRafError::Ed25519Accounts);
        
        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1, BeeRafError::Ed25519DataLength);
        let signature = &signatures[0];

        // Make sure all the data is present to verify the signature
        require!(signature.is_verifiable, BeeRafError::Ed25519Header);
        
        // Ensure public keys match
        require_keys_eq!(signature.public_key.ok_or(BeeRafError::Ed25519Pubkey)?, self.maker.key(), BeeRafError::Ed25519Pubkey);

        // Ensure signatures match
        require!(&signature.signature.ok_or(BeeRafError::Ed25519Signature)?.eq(sig), BeeRafError::Ed25519Signature);

        // Ensure messages match
        require!(&signature.message.as_ref().ok_or(BeeRafError::Ed25519Signature)?.eq(&self.raffle_config.to_slice()), BeeRafError::Ed25519Signature);

        Ok(())
    }
    
    pub fn solve_raffle(&mut self, sig: &[u8]) -> Result<()> {
        let house = self.house.key();
        let raffle = self.raffle.key();

        let slot = Clock::get()?.slot;
        
        require!(slot > self.raffle_config.slot, BeeRafError::StillOpen);

        require!(
            self.raffle.num_minted > 0,
            BeeRafError::NoSoldAnyTicket
        );

        let hash = hash(sig).to_bytes();
        let mut hash_16: [u8;16] = [0;16];
        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);
        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);
        
        let roll = lower
            .wrapping_add(upper)
            .wrapping_rem(self.raffle.num_minted as u128) as u32 + 1;

        emit!(RafEvent {
            winner: roll,
        });

        require!(roll > 0 && roll < self.raffle.num_minted +1, BeeRafError::FailedRoll);

        // Check that the maximum number of tickets has not been reached yet
        let (_,mut collection_attribute_list, _) = fetch_plugin::<BaseCollectionV1, Attributes>(
            &self.raffle.to_account_info(),
            PluginType::Attributes,
        )?;

        let winner = Attribute {
            key: "Winner".to_string(),
            value: roll.to_string() // args.capacity.to_string(),
        };

        collection_attribute_list.attribute_list.push(winner);
        
        // Prepare seeds for the PDA `raffle_config`
        let raffle_config_seeds = &[
            b"raffle",
            house.as_ref(),
            raffle.as_ref(),
            &[self.raffle_config.raffle_config_bump],
        ];

        // Update the collection's attributes using CPI
        UpdateCollectionPluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.raffle.to_account_info())
            .authority(Some(&self.raffle_config.to_account_info()))
            .payer(&self.maker.to_account_info())
            .plugin(mpl_core::types::Plugin::Attributes(
                mpl_core::types::Attributes {
                    attribute_list: collection_attribute_list.attribute_list,
                },
            ))
            .system_program(&self.system_program)
            .invoke_signed(&[raffle_config_seeds])
            .unwrap();

        Ok(())
    }
}