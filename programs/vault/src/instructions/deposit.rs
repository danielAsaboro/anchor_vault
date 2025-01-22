use anchor_lang::{ prelude::*, system_program, system_program::Transfer };

use crate::states::vault_state::VaultState;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let transfer = Transfer {
        from: ctx.accounts.owner.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info(), transfer);

    system_program::transfer(cpi_context, amount)?;
    Ok(())
}
