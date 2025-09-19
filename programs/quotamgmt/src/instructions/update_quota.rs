use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct UpdateQuota<'info> {
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

pub fn handler(
    ctx: Context<UpdateQuota>,
    new_allocated_quota: Option<u64>,
    new_validity_period: Option<i64>,
    status: Option<QuotaStatus>,
    update_reason: String,
) -> Result<()> {
    let quota_account = &mut ctx.accounts.quota_account;
    let current_time = Clock::get()?.unix_timestamp;

    // Validation
    require!(
        update_reason.len() <= 200,
        QuotaError::ReasonTooLong
    );

    // Cannot modify expired quota unless we're changing the validity period
    if quota_account.status == QuotaStatus::Expired && new_validity_period.is_none() {
        return Err(QuotaError::CannotModifyExpiredQuota.into());
    }

    // Store old values for event
    let old_allocated_quota = quota_account.allocated_quota;
    let old_validity_period = quota_account.validity_period;
    let old_status = quota_account.status.clone();

    // Update allocated quota
    if let Some(new_quota) = new_allocated_quota {
        require!(new_quota > 0, QuotaError::InvalidQuotaAmount);
        
        let difference = if new_quota > quota_account.allocated_quota {
            new_quota - quota_account.allocated_quota
        } else {
            quota_account.allocated_quota - new_quota
        };

        // Update allocated and available quota
        quota_account.allocated_quota = new_quota;
        
        // Adjust available quota proportionally
        if new_quota >= quota_account.used_quota {
            quota_account.available_quota = new_quota - quota_account.used_quota;
        } else {
            // If new quota is less than used quota, set available to 0
            quota_account.available_quota = 0;
            quota_account.status = QuotaStatus::Exhausted;
        }
    }

    // Update validity period
    if let Some(new_period) = new_validity_period {
        require!(
            new_period > current_time,
            QuotaError::InvalidValidityPeriod
        );
        quota_account.validity_period = new_period;
        
        // If quota was expired and we're extending it, reactivate if conditions are met
        if quota_account.status == QuotaStatus::Expired && quota_account.available_quota > 0 {
            quota_account.status = QuotaStatus::Active;
        }
    }

    // Update status
    if let Some(new_status) = status {
        quota_account.status = new_status;
    }

    // Check if quota should be expired based on current time
    if current_time > quota_account.validity_period {
        quota_account.status = QuotaStatus::Expired;
    }

    // Update timestamp
    quota_account.updated_at = current_time;

    // Emit status update event if status changed
    if old_status != quota_account.status {
        emit!(QuotaStatusUpdated {
            concession_id: quota_account.concession_id.clone(),
            old_status,
            new_status: quota_account.status.clone(),
            updated_by: ctx.accounts.regulator.key(),
            reason: update_reason.clone(),
            timestamp: current_time,
        });
    }

    // Emit quota update event
    emit!(QuotaUpdated {
        concession_id: quota_account.concession_id.clone(),
        old_allocated_quota,
        new_allocated_quota: quota_account.allocated_quota,
        old_validity_period,
        new_validity_period: quota_account.validity_period,
        updated_by: ctx.accounts.regulator.key(),
        timestamp: current_time,
    });

    Ok(())
}