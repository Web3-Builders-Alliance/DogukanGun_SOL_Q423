use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer}, associated_token::AssociatedToken};
use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker
    )]
    pub maker_ata_a: Account<'info, TokenAccount>,

    #[account(
        init, 
        space=Escrow::INIT_SPACE, 
        seeds=[b"escrow".as_ref(), maker.key().as_ref()], 
        bump,
        payer=maker,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init, 
        payer=maker,
        token::mint=mint_a,
        token::authority=escrow,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

}

 impl<'info> Make<'info> {
    pub fn save(&mut self,recieve:u64,bumps: &MakeBumps) -> Result<()> {
        /*self.escrow.set_inner(Escrow {
            seed,
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            recieve,
            bump: bumps.escrow,
            vault_bump: bumps.vault,
        });*/
        self.escrow.mint_a = self.mint_a.key();
        self.escrow.mint_b = self.mint_b.key();
        self.escrow.recieve = recieve;
        self.escrow.bump = bumps.escrow;
        Ok(())
    }
    pub fn deposit(&mut self,deposit:u64) -> Result<()> {
        let transfer_accounts = Transfer {
            from: self.maker_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        let _ = transfer(cpi_ctx, deposit);
        Ok(())
    }
}