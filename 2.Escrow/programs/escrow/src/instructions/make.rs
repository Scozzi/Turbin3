use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}
};

use crate::state::Escrow;
// use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(id: u64, amount: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker, 
        seeds = [b"escrow", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump,
        space = 8 + Escrow::INIT_SPACE
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info>{

    pub fn make(&mut self, id: u64, amount: u64, bumps: MakeBumps) -> Result<()> { // pid: &Pubkey,
        self.init_escrow(id, amount, bumps.escrow)?;

        self.deposit_into_escrow(amount)?;

        Ok(())
    }

    fn init_escrow(&mut self, id:u64, recieve_amount: u64, bump: u8) -> Result<()> {
        self.escrow.set_inner(Escrow {
            id,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            recieve_amount,
            bump,
        });

        Ok(())
    }

    fn deposit_into_escrow(&mut self, deposit: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_account = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.mint_a.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);
        transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)
    }
}