use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program::{transfer, Transfer}};

use crate::Config;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub house: Signer<'info>,

    #[account(
        seeds = [b"treasury", house.key().as_ref()],
        bump
    )]
    treasury: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = house,
        seeds = [b"config", treasury.key().as_ref()],
        space = Config::INIT_SPACE,
        bump
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, fee: u64, bumps: &InitializeBumps) -> Result<()> {
        /* let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.house.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, 10 * LAMPORTS_PER_SOL)?; */
        
        self.config.set_inner(Config {
            authority: self.house.key(),
            fee,
            config_bump: bumps.config,
            treasury_bump: bumps.treasury,
        });

        Ok(())
    }
}