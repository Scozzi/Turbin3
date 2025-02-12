use anchor_lang::prelude::*;

pub mod error;

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

declare_id!("2RwTZzWToZemV8dzeCDZ7QYwHNniPGadY2ej3penF4vB");

#[program]
pub mod escrow {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Program ID: {:?}", escrow::ID);
    //     Ok(())
    // }

    pub fn make(ctx: Context<Make>, id: u64, amount: u64) -> Result<()> {
        ctx.accounts.make(id, amount, ctx.bumps) // &escrow::ID,
    }

    pub fn take(ctx: Context<Take>, amount: u64) -> Result<()> {
        ctx.accounts.take(amount)
    }
}
