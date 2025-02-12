use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::VaultState;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", owner.key().as_ref()],
        bump = state.state_bump,
        close = owner
    )]
    pub state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.owner.to_account_info(),
        };
        let vault_state_key = self.state.key();

        let seeds = &[b"vault", vault_state_key.as_ref(), &[self.state.vault_bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        let amount = self.vault.lamports();

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
