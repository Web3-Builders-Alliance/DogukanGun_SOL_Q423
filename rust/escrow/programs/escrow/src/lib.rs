use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer}, associated_token::AssociatedToken};

pub mod state;
use state::*;

pub mod context;
use context::*;

declare_id!("4g7G1GBzRbQNzrY3odCtferEdrUNJoDVSwYcgvrwe3ZL");

#[program]
pub mod escrow {
    use anchor_spl::token::{close_account, CloseAccount, transfer, Transfer};
    use super::*;

    pub fn make(ctx: Context<Make>,deposit:u64,recieve:u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        ctx.accounts.save(recieve,&ctx.bumps);
    }

    pub fn take(ctx: Context<Take>,amount:u64) -> Result<()> {
        ctx.accounts.take(amount);
    }

    pub fn return(ctx: Context<Close>,refund:u64) -> Result<()> {
        ctx.accounts.refund(refund);
    }

}