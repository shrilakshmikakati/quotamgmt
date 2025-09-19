use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct ReactivateQuota<'info> {
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

pub fn handler(ctx: Context<ReactivateQuota>) -> Result<()> {
    let quota_account = &mut ctx.accounts.quota_account;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Can only reactivate suspended quotas
    require!(
        quota_account.status == QuotaStatus::Suspended,
        QuotaError::QuotaNotActive
    );

    // Check if quota is still valid (not expired)
    require!(
        current_time <= quota_account.validity_period,
        QuotaError::QuotaExpired
    );

    // Check if quota is not exhausted
    require!(
        quota_account.available_quota > 0,
        QuotaError::QuotaExhausted
    );

    let old_status = quota_account.status.clone();
    quota_account.status = QuotaStatus::Active;
    quota_account.updated_at = current_time;

    // Emit event
    emit!(QuotaStatusUpdated {
        concession_id: quota_account.concession_id.clone(),
        old_status,
        new_status: QuotaStatus::Active,
        updated_by: ctx.accounts.regulator.key(),
        reason: "Quota reactivated by regulator".to_string(),
        timestamp: current_time,
    });

    Ok(())
}