use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(amount: u64, transfer_reason: String)]
pub struct TransferQuota<'info> {
    #[account(
        mut,
        has_one = holder,
        seeds = [b"quota", from_quota.concession_id.as_bytes(), from_quota.holder.as_ref()],
        bump = from_quota.bump
    )]
    pub from_quota: Account<'info, QuotaAccount>,
    
    #[account(
        mut,
        seeds = [b"quota", to_quota.concession_id.as_bytes(), to_quota.holder.as_ref()],
        bump = to_quota.bump
    )]
    pub to_quota: Account<'info, QuotaAccount>,

    #[account(
        init,
        payer = holder,
        space = TransferRecord::LEN,
        seeds = [
            b"transfer", 
            from_quota.concession_id.as_bytes(),
            to_quota.concession_id.as_bytes(),
            &Clock::get().unwrap().unix_timestamp.to_le_bytes()
        ],
        bump
    )]
    pub transfer_record: Account<'info, TransferRecord>,
    
    #[account(mut)]
    pub holder: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<TransferQuota>,
    amount: u64,
    transfer_reason: String,
) -> Result<()> {
    let from_quota = &mut ctx.accounts.from_quota;
    let to_quota = &mut ctx.accounts.to_quota;
    let current_time = Clock::get()?.unix_timestamp;

    // Validation
    require!(from_quota.status == QuotaStatus::Active, QuotaError::QuotaNotActive);
    require!(to_quota.status == QuotaStatus::Active, QuotaError::QuotaNotActive);
    require!(from_quota.available_quota >= amount, QuotaError::InsufficientQuota);
    require!(amount > 0, QuotaError::InvalidTransferAmount);
    require!(
        transfer_reason.len() <= TransferRecord::MAX_TRANSFER_REASON_LEN,
        QuotaError::ReasonTooLong
    );
    require!(
        from_quota.concession_id != to_quota.concession_id,
        QuotaError::SelfTransferNotAllowed
    );

    // Check validity periods
    require!(current_time <= from_quota.validity_period, QuotaError::QuotaExpired);
    require!(current_time <= to_quota.validity_period, QuotaError::QuotaExpired);

    // Update quotas
    from_quota.available_quota -= amount;
    from_quota.allocated_quota -= amount;
    from_quota.updated_at = current_time;

    to_quota.available_quota += amount;
    to_quota.allocated_quota += amount;
    to_quota.updated_at = current_time;

    // Update status if quota is exhausted
    if from_quota.available_quota == 0 {
        from_quota.status = QuotaStatus::Exhausted;
    }

    // Create transfer record
    let transfer_record = &mut ctx.accounts.transfer_record;
    transfer_record.from_concession = from_quota.concession_id.clone();
    transfer_record.to_concession = to_quota.concession_id.clone();
    transfer_record.amount = amount;
    transfer_record.timestamp = current_time;
    transfer_record.authorized_by = ctx.accounts.holder.key();
    transfer_record.transfer_reason = transfer_reason;
    transfer_record.transfer_type = TransferType::Planned; // Default type
    transfer_record.bump = ctx.bumps.transfer_record;

    // Emit event
    emit!(QuotaTransferred {
        from_concession: from_quota.concession_id.clone(),
        to_concession: to_quota.concession_id.clone(),
        amount,
        transfer_type: TransferType::Planned,
        authorized_by: ctx.accounts.holder.key(),
        timestamp: current_time,
    });

    Ok(())
}