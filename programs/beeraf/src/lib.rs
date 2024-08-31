pub mod constants;
pub mod contexts;
pub mod error;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use contexts::*;
pub use state::*;

declare_id!("9kqdw16Bf66qL53XSzG21TZjDEWPfawuyBTML1vVPqTs");

#[program]
pub mod beeraf {
    use super::*;

    // It will create the treasoury account where the fees will go.
    // Setup the fee amount it will get
    // authority person who can take out funds
    pub fn initialize(ctx: Context<Initialize>, fee: u64) -> Result<()> {
        ctx.accounts.initialize(fee, &ctx.bumps)
    }

    // Create_raffle will create a PDA where contains the amount required to buy a ticket
    // mint autority with the tickets
    // NFT details to mint the NFT onchain
    // Counter with the number of tickets sold
    pub fn create_raffle(
        ctx: Context<CreateRaffle>,
        args: CreateRaffleArgs
    ) -> Result<()> {
        ctx.accounts.create_raffle(args, &ctx.bumps)
    }

    // It will pay the amount referenced in the PDA
    // it will mint a ticket
    pub fn buy_ticket(
        ctx: Context<BuyTicket>,
        args: BuyTicketArgs
    ) -> Result<()> {
        ctx.accounts.buy_ticket(args)
    }

    // It will generate a valid number considering the amount of tickets
    // It will get stored in the PDA and change the status to RESOLVED
    pub fn solve_raffle(
        ctx: Context<SolveRaffle>,
        sig: Vec<u8>
    ) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.solve_raffle(&sig)
    }

    // IT will check if the ticket is the winner
    // it will send the money to the user
    // it will burn the token to recover the rent
    pub fn scratch_ticket(
        ctx: Context<ScratchTicket>
    ) -> Result<()> {
        ctx.accounts.scratch_ticket()
    }

    // it will close the raffle and return the rent to the authority.
    // pub close_raffle
}
