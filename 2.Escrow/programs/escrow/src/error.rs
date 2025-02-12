use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds in the escrow account")]
    InsufficientFunds,

    #[msg("Escrow already exists")]
    EscrowAlreadyExists,
}
