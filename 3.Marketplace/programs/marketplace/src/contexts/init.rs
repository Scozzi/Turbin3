use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::errors::MarketplaceError;
use crate::state::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = Marketplace::INIT_SPACE,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump,
    )]
    marketplace: Box<Account<'info, Marketplace>>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = marketplace,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
    )]
    rewards_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    treasury: SystemAccount<'info>,

    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        require!(name.len() <= 32, MarketplaceError::NameTooLong);

        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            name,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_bump: bumps.rewards_mint,
        });

        Ok(())
    }
}
