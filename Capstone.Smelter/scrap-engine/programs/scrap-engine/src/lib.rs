use anchor_lang::prelude::*;

declare_id!("5wCGNL7Mo2FNpATH8PPfq37L4rJ25qbSfbhCXFhTEsAH");

#[program]
pub mod scrap_engine {
    use super::*;

    pub fn initialize_collection(ctx: Context<InitializeCollection>) -> Result<()> {
        Ok(())
    }

    pub fn mint_cnft(ctx: Context<MintCnft>) -> Result<()> {
        Ok(())
    }

    pub fn burn_cnft(ctx: Context<BurnCnft>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCollection {}

#[derive(Accounts)]
pub struct MintCnft {}
