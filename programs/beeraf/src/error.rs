use anchor_lang::prelude::*;

#[error_code]
pub enum BeeRafError {
    #[msg("Missing Attribute")]
    MissingAttribute,

    #[msg("Numerical Overflow")]
    NumericalOverflow,

    #[msg("Maximum Ticket Reached")]
    MaximumTicketsReached,

    #[msg("Raffle is still open")]
    StillOpen,

    #[msg("It didn't solve any ticket")]
    NoSoldAnyTicket,

    #[msg("Failed roll generation. Try again")]
    FailedRoll,

    #[msg("Ed25519 Header Error")]
    Ed25519Header,

    #[msg("Ed25519 Pubkey Error")]
    Ed25519Pubkey,

    #[msg("Ed25519 Message Error")]
    Ed25519Message,

    #[msg("Ed25519 Signature Error")]
    Ed25519Signature,

    #[msg("Ed25119 Program Error")]
    Ed25519Program,

    #[msg("Ed25119 Accounts Error")]
    Ed25519Accounts,

    #[msg("Ed25119 Data Length Error")]
    Ed25519DataLength
}
