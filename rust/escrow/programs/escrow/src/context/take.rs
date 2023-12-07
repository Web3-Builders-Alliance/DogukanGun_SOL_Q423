use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer}, associated_token::AssociatedToken};
use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker
    )]
    pub taker_ata_a: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker
    )]
    pub taker_ata_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker
    )]
    pub maker_ata_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        close=maker,
        seeds=[b"escrow".as_ref(), maker.key().as_ref()], 
        bump=escrow.bump, 
        token::mint=mint_a,
        token::authority=escrow,
    )]
    pub escrow: Account<'info, Escrow>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

}

pub impl<'info> Take<'info> {

    pub fn take(&mut self,amount: u64) -> Result<()> {
        let transfer_accounts = Transfer {
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.clone(), transfer_accounts);

        transfer(cpi_ctx, self.escrow.recieve);

        let transfer_accounts = Transfer {
            from: self.escrow.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let signer_seeds: [&[&[u8]];1] = [
            &[
                b"escrow", 
                self.maker.to_account_info().key.as_ref(), 
                [..],
                &[self.escrow.escrow_bump]
            ]
        ];
        /*let seeds = &[
            "escrow".as_bytes(),
            self.maker.to_account_info().key.as_ref(), 
            &[self.escrow.escrow_bump]
        ];
        let signer_seeds = &[&seeds[..]];*/
        let cpi_ctx = CpiContext::new_with_signer(self.token_program.clone(), transfer_account, signer_seeds);

        transfer(cpi_ctx, amount);
        Ok(())
    }

}