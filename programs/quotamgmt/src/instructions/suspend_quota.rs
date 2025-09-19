use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct SuspendQuota<'info> {
    #[account(
        mut,
        seeds = [b"quota", quota_account.concession_id.as_bytes(), quota_account.holder.as_ref()],
        bump = quota_account.bump,
        has_one = regulator
    )]
    pub quota_account: Account<'info, QuotaAccount>,
    
    #[account(mut)]
    pub regulator: Signer<'info>,
}

pub fn handler(ctx: Context<SuspendQuota>, reason: String) -> Result<()> {
    require!(reason.len() <= 200, QuotaError::ReasonTooLong);
    
    let quota_account = &mut ctx.accounts.quota_account;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Can only suspend active quotas
    require!(
        quota_account.status == QuotaStatus::Active,
        QuotaError::QuotaNotActive
    );

    let old_status = quota_account.status.clone();
    quota_account.status = QuotaStatus::Suspended;
    quota_account.updated_at = current_time;

    // Emit event
    emit!(QuotaStatusUpdated {
        concession_id: quota_account.concession_id.clone(),
        old_status,
        new_status: QuotaStatus::Suspended,
        updated_by: ctx.accounts.regulator.key(),
        reason,
        timestamp: current_time,
    });

    Ok(())
}