use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer, CloseAccount, close_account}, associated_token::AssociatedToken};
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

impl<'info> Take<'info> {

    pub fn take(&mut self) -> Result<()> {
        let transfer_accounts = Transfer {
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let cpi_ctx: CpiContext<'_, '_, '_, '_, Transfer<'_>> = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        let _ = transfer(cpi_ctx, self.escrow.recieve);

        /*let signer_seeds: &[&[&[u8]];1] = [
            &[
                b"escrow", 
                self.maker.to_account_info().key.as_ref(), 
                [..],
                &[self.escrow.escrow_bump]
            ]
        ];*/
        Ok(())

    }

    pub fn withdraw_and_close(&mut self) -> Result<()> {
        let seeds = &[
            "escrow".as_bytes(),
            self.maker.to_account_info().key.as_ref(), 
            &[self.escrow.bump]
        ];
        let signer_seeds = &[&seeds[..]];
        let transfer_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let cpi_ctx: CpiContext<'_, '_, '_, '_, Transfer<'_>> = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer(cpi_ctx, self.vault.amount)?;

        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.escrow.to_account_info()
        };
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            accounts,
            signer_seeds
        );

        let _ = close_account(ctx);
        Ok(())
    }

}