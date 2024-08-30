use anchor_lang::prelude::*;

#[error_code]
pub enum BeeRafError {
    #[msg("Missing Attribute")]
    MissingAttribute,

    #[msg("Numerical Overflow")]
    NumericalOverflow,

    #[msg("Maximum Ticket Reached")]
    MaximumTicketsReached,

    
}
