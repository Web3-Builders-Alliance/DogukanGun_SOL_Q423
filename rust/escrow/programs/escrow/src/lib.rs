use anchor_lang::prelude::*;

pub mod state;

pub mod context;
use context::*;

declare_id!("4g7G1GBzRbQNzrY3odCtferEdrUNJoDVSwYcgvrwe3ZL");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>,deposit:u64,recieve:u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        let _ = ctx.accounts.save(recieve,&ctx.bumps);
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        let _ = ctx.accounts.take();
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        let _ = ctx.accounts.refund();
        Ok(())
    }

}