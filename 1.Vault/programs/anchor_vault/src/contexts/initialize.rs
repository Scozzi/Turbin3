use anchor_lang::prelude::*;

use crate::state::VaultState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [b"state", owner.key().as_ref()],
        space = 8 + VaultState::INIT_SPACE,
        bump
    )]
    pub state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        let vs = &mut self.state;
        vs.state_bump = bumps.state;
        vs.vault_bump = bumps.vault;

        Ok(())
    }
}
