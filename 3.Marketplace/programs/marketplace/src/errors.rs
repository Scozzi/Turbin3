use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Name is too long")]
    NameTooLong,
}
