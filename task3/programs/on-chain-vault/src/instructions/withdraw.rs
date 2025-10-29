//-------------------------------------------------------------------------------
///
/// TASK: Implement the withdraw functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority
/// - Emit a withdraw event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_authority.key().as_ref()],
        bump,
        constraint = !vault.locked @ VaultError::VaultLocked
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // Check if vault has sufficient balance
    let vault_balance = ctx.accounts.vault.to_account_info().lamports();
    
    if amount > vault_balance {
        return Err(VaultError::InsufficientBalance.into());
    }

    // Use Anchor's native transfer method for PDA accounts
    let vault_info = ctx.accounts.vault.to_account_info();
    let authority_info = ctx.accounts.vault_authority.to_account_info();
    
    // Calculate new balances
    let new_vault_balance = vault_balance.checked_sub(amount).ok_or(VaultError::Overflow)?;
    let new_authority_balance = authority_info.lamports().checked_add(amount).ok_or(VaultError::Overflow)?;
    
    // Set new balances - this works because we're the program that owns the PDA
    **vault_info.try_borrow_mut_lamports()? = new_vault_balance;
    **authority_info.try_borrow_mut_lamports()? = new_authority_balance;

    // Emit withdraw event
    emit!(WithdrawEvent {
        amount,
        vault_authority: ctx.accounts.vault_authority.key(),
        vault: ctx.accounts.vault.key(),
    });

    Ok(())
}