use anchor_lang::prelude::*;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("DuhUAUnAc4eBaaGyMFNPE7aERFSJnjGU53svGM9t9QvJ");

#[program]
pub mod vault {
    //
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::deposit(ctx, amount)
    }

    pub fn take_deposit(ctx: Context<TakeDeposit>, amount: u64) -> Result<()> {
        instructions::take::take_deposit(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
