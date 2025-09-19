use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(concession_id: String)]
pub struct InitializeQuota<'info> {
    #[account(
        init,
        payer = regulator,
        space = QuotaAccount::LEN,
        seeds = [b"quota", concession_id.as_bytes(), holder.key().as_ref()],
        bump
    )]
    pub quota_account: Account<'info, QuotaAccount>,
    
    /// CHECK: Holder of the quota - verified by regulator
    pub holder: AccountInfo<'info>,
    
    #[account(mut)]
    pub regulator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeQuota>,
    concession_id: String,
    allocated_quota: u64,
    validity_period: i64,
    quota_type: QuotaType,
) -> Result<()> {
    // Validation
    require!(allocated_quota > 0, QuotaError::InvalidQuotaAmount);
    require!(
        validity_period > Clock::get()?.unix_timestamp, 
        QuotaError::InvalidValidityPeriod
    );
    require!(
        concession_id.len() <= QuotaAccount::MAX_CONCESSION_ID_LEN, 
        QuotaError::ConcessionIdTooLong
    );

    let quota_account = &mut ctx.accounts.quota_account;
    let current_time = Clock::get()?.unix_timestamp;

    // Initialize quota account
    quota_account.concession_id = concession_id.clone();
    quota_account.holder = ctx.accounts.holder.key();
    quota_account.regulator = ctx.accounts.regulator.key();
    quota_account.allocated_quota = allocated_quota;
    quota_account.used_quota = 0;
    quota_account.available_quota = allocated_quota;
    quota_account.validity_period = validity_period;
    quota_account.status = QuotaStatus::Active;
    quota_account.quota_type = quota_type.clone();
    quota_account.mining_region = String::new(); // Can be updated later
    quota_account.environmental_clearance = String::new(); // Can be updated later
    quota_account.created_at = current_time;
    quota_account.updated_at = current_time;
    quota_account.bump = ctx.bumps.quota_account;

    // Emit event
    emit!(QuotaInitialized {
        concession_id,
        holder: quota_account.holder,
        allocated_quota,
        validity_period,
        quota_type,
        mining_region: quota_account.mining_region.clone(),
    });

    Ok(())
}