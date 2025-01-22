use anchor_lang::{ prelude::*, system_program, system_program::Transfer };

use crate::states::vault_state::VaultState;

#[derive(Accounts)]
pub struct TakeDeposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub vault: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

// pub fn take_deposit(ctx: Context<TakeDeposit>, amount: u64) -> Result<()> {
//     let owner_key = ctx.accounts.owner.key();
//     let seeds = &[b"vault", owner_key.as_ref(), &[ctx.bumps.vault]];

//     let cpi_context = CpiContext::new_with_signer(
//         ctx.accounts.system_program.to_account_info(),
//         Transfer {
//             from: ctx.accounts.vault.to_account_info(),
//             to: ctx.accounts.owner.to_account_info(),
//         },
//         &[seeds]
//     );

//     system_program::transfer(cpi_context, amount)?;
//     Ok(())
// }
pub fn take_deposit(ctx: Context<TakeDeposit>, amount: u64) -> Result<()> {
    let owner_key = ctx.accounts.owner.key();
    let vault_bump = ctx.bumps.vault;

    let seed_data = [b"vault", owner_key.as_ref(), &[vault_bump]];
    let seeds = &[&seed_data[..]];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
        },
        seeds
    );

    system_program::transfer(cpi_context, amount)?;
    Ok(())
}
