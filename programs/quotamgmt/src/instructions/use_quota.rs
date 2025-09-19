use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(amount: u64, shipment_id: String)]
pub struct UseQuota<'info> {
    #[account(
        mut,
        seeds = [b"quota", quota_account.concession_id.as_bytes(), quota_account.holder.as_ref()],
        bump = quota_account.bump,
        has_one = holder
    )]
    pub quota_account: Account<'info, QuotaAccount>,

    #[account(
        init,
        payer = holder,
        space = UsageRecord::LEN,
        seeds = [b"usage", shipment_id.as_bytes(), holder.key().as_ref()],
        bump
    )]
    pub usage_record: Account<'info, UsageRecord>,
    
    #[account(mut)]
    pub holder: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<UseQuota>,
    amount: u64,
    shipment_id: String,
    quality_params: QualityParameters,
) -> Result<()> {
    let quota_account = &mut ctx.accounts.quota_account;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validation
    require!(quota_account.status == QuotaStatus::Active, QuotaError::QuotaNotActive);
    require!(current_time <= quota_account.validity_period, QuotaError::QuotaExpired);
    require!(amount > 0, QuotaError::InvalidUsageAmount);
    require!(quota_account.available_quota >= amount, QuotaError::InsufficientQuota);
    require!(
        shipment_id.len() <= UsageRecord::MAX_SHIPMENT_ID_LEN, 
        QuotaError::ShipmentIdTooLong
    );
    
    // Validate quality parameters
    validate_quality_parameters(&quality_params)?;

    // Update quota account
    quota_account.used_quota += amount;
    quota_account.available_quota -= amount;
    quota_account.updated_at = current_time;

    // Check if quota is now exhausted
    if quota_account.available_quota == 0 {
        quota_account.status = QuotaStatus::Exhausted;
    }

    // Record usage
    let usage_record = &mut ctx.accounts.usage_record;
    usage_record.concession_id = quota_account.concession_id.clone();
    usage_record.shipment_id = shipment_id.clone();
    usage_record.amount = amount;
    usage_record.timestamp = current_time;
    usage_record.holder = quota_account.holder;
    usage_record.quality_params = quality_params.clone();
    usage_record.source_location = String::new(); // To be updated by mobile app
    usage_record.destination_location = String::new(); // To be updated by mobile app
    usage_record.transport_details = String::new(); // To be updated by mobile app
    usage_record.bump = ctx.bumps.usage_record;

    // Emit event
    emit!(QuotaUsed {
        concession_id: quota_account.concession_id.clone(),
        shipment_id,
        amount,
        remaining_quota: quota_account.available_quota,
        quality_params,
        timestamp: current_time,
    });

    Ok(())
}

fn validate_quality_parameters(params: &QualityParameters) -> Result<()> {
    // Validate GCV (2000-8000 kcal/kg is typical range for coal)
    require!(
        params.gross_calorific_value >= 2000 && params.gross_calorific_value <= 8000,
        QuotaError::InvalidGCVValue
    );
    
    // Validate moisture content (0-50%)
    require!(
        params.moisture_content <= 5000, // 50.00%
        QuotaError::InvalidMoistureContent
    );
    
    // Validate ash content (0-50%)
    require!(
        params.ash_content <= 5000, // 50.00%
        QuotaError::InvalidAshContent
    );
    
    // Validate sulphur content (0-10%)
    require!(
        params.sulphur_content <= 1000, // 10.00%
        QuotaError::InvalidSulphurContent
    );
    
    // Validate size classification length
    require!(
        params.size_classification.len() <= QualityParameters::MAX_SIZE_CLASSIFICATION_LEN,
        QuotaError::SizeClassificationTooLong
    );

    Ok(())
}