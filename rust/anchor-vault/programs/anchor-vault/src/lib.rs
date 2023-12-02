use anchor_lang::prelude::*;

declare_id!("7bCFZ6BvXKWpPthmdMmQkK3q2NRiUJT2YNSEF7vbRiJn");

#[program]
pub mod anchor_vault {
    
    use anchor_lang::system_program::{Transfer, transfer};

    use super::*;

    pub fn deposit(ctx: Context<Vault>,lamports: u64) -> Result<()> {
        let accounts = Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            accounts,
        );

        transfer(cpi_ctx, lamports)
    }

    pub fn close(ctx: Context<Vault>,lamports: u64) -> Result<()> {
        let accounts = Transfer {
            from: ctx.accounts.vault.to_account_info().clone(),
            to: ctx.accounts.signer.to_account_info().clone(),
        };

        msg!("Transferring tokens to signer account");

        let signer_seeds: [&[&[u8]]; 1] = [&[b"vault", &ctx.accounts.signer.to_account_info().key.as_ref(),&[ctx.bumps.vault]]]; 

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            accounts,
            &signer_seeds,
        );

        transfer(cpi_ctx, lamports)
    }
}

#[derive(Accounts)]
pub struct Vault<'info> {

    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        seeds = [b"vault",signer.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,
    system_program: Program<'info, System>
}
