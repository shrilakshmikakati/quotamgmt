#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

// Ensure this matches your program's ID from Anchor.toml
declare_id!("9v8P6i1esz4orrqTx9X5EhV28hifkT4Q3HEoGLPeKmPn");

#[program]
pub mod quotamanagement {
    use super::*;

    pub fn initialize_quota(
        ctx: Context<InitializeQuota>,
        concession_id: String,
        allocated_quota: u64,
        validity_period: i64,
        quota_type: QuotaType,
    ) -> Result<()> {
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

        quota_account.concession_id = concession_id.clone();
        quota_account.holder = ctx.accounts.holder.key();
        quota_account.regulator = ctx.accounts.regulator.key();
        quota_account.allocated_quota = allocated_quota;
        quota_account.used_quota = 0;
        quota_account.available_quota = allocated_quota;
        quota_account.validity_period = validity_period;
        quota_account.quota_type = quota_type;
        quota_account.status = QuotaStatus::Active;
        quota_account.created_at = current_time;
        quota_account.updated_at = current_time;
        quota_account.bump = ctx.bumps.quota_account;

        emit!(QuotaInitialized {
            concession_id,
            holder: quota_account.holder,
            allocated_quota,
            validity_period,
        });

        Ok(())
    }

    pub fn use_quota(
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
}

// Accounts
#[derive(Accounts)]
#[instruction(concession_id: String)]
pub struct InitializeQuota<'info> {
    #[account(
        init,
        payer = regulator,
        space = 8 + QuotaAccount::INIT_SPACE,
        seeds = [b"quota", concession_id.as_bytes(), holder.key().as_ref()],
        bump
    )]
    pub quota_account: Account<'info, QuotaAccount>,
    
    /// CHECK: Holder of the quota
    pub holder: AccountInfo<'info>,
    
    #[account(mut)]
    pub regulator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

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
        space = 8 + UsageRecord::INIT_SPACE,
        seeds = [b"usage", shipment_id.as_bytes(), holder.key().as_ref()],
        bump
    )]
    pub usage_record: Account<'info, UsageRecord>,
    
    #[account(mut)]
    pub holder: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// State
#[account]
pub struct QuotaAccount {
    pub concession_id: String,
    pub holder: Pubkey,
    pub regulator: Pubkey,
    pub allocated_quota: u64,
    pub used_quota: u64,
    pub available_quota: u64,
    pub validity_period: i64,
    pub quota_type: QuotaType,
    pub status: QuotaStatus,
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

impl QuotaAccount {
    pub const MAX_CONCESSION_ID_LEN: usize = 32;
    pub const INIT_SPACE: usize = 
        4 + Self::MAX_CONCESSION_ID_LEN + // concession_id (String)
        32 + // holder (Pubkey)
        32 + // regulator (Pubkey)
        8 + // allocated_quota (u64)
        8 + // used_quota (u64)
        8 + // available_quota (u64)
        8 + // validity_period (i64)
        1 + 1 + // quota_type (Enum)
        1 + 1 + // status (Enum)
        8 + // created_at (i64)
        8 + // updated_at (i64)
        1; // bump (u8)
}

#[account]
pub struct UsageRecord {
    pub concession_id: String,
    pub shipment_id: String,
    pub amount: u64,
    pub timestamp: i64,
    pub holder: Pubkey,
    pub quality_params: QualityParameters,
    pub source_location: String,
    pub destination_location: String,
    pub transport_details: String,
    pub bump: u8,
}

impl UsageRecord {
    pub const MAX_SHIPMENT_ID_LEN: usize = 32;
    pub const INIT_SPACE: usize =
        4 + Self::MAX_SHIPMENT_ID_LEN + // shipment_id (String)
        4 + QuotaAccount::MAX_CONCESSION_ID_LEN + // concession_id (String)
        8 + // amount (u64)
        8 + // timestamp (i64)
        32 + // holder (Pubkey)
        QualityParameters::INIT_SPACE + // quality_params
        4 + 32 + // source_location (String)
        4 + 32 + // destination_location (String)
        4 + 64 + // transport_details (String)
        1; // bump (u8)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct QualityParameters {
    pub gross_calorific_value: u16,
    pub moisture_content: u16,
    pub ash_content: u16,
    pub sulphur_content: u16,
    pub volatile_matter: u16,
    pub fixed_carbon: u16,
    pub coal_grade: CoalGrade,
    pub size_classification: String,
}

impl QualityParameters {
    pub const MAX_SIZE_CLASSIFICATION_LEN: usize = 16;
    pub const INIT_SPACE: usize = 
        2 + // gross_calorific_value
        2 + // moisture_content
        2 + // ash_content
        2 + // sulphur_content
        2 + // volatile_matter
        2 + // fixed_carbon
        1 + 1 + // coal_grade
        4 + Self::MAX_SIZE_CLASSIFICATION_LEN; // size_classification (String)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum CoalGrade {
    GradeA,
    GradeB,
    GradeC,
    GradeD,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum QuotaStatus {
    Active,
    Suspended,
    Expired,
    Exhausted,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum QuotaType {
    Annual,
    Monthly,
}

// Events
#[event]
pub struct QuotaInitialized {
    pub concession_id: String,
    pub holder: Pubkey,
    pub allocated_quota: u64,
    pub validity_period: i64,
}

#[event]
pub struct QuotaUsed {
    pub concession_id: String,
    pub shipment_id: String,
    pub amount: u64,
    pub remaining_quota: u64,
    pub quality_params: QualityParameters,
    pub timestamp: i64,
}

// Errors
#[error_code]
pub enum QuotaError {
    #[msg("Invalid quota amount")]
    InvalidQuotaAmount,
    #[msg("Invalid validity period")]
    InvalidValidityPeriod,
    #[msg("Concession ID too long")]
    ConcessionIdTooLong,
    #[msg("Quota is not active")]
    QuotaNotActive,
    #[msg("Quota has expired")]
    QuotaExpired,
    #[msg("Invalid usage amount")]
    InvalidUsageAmount,
    #[msg("Insufficient quota available")]
    InsufficientQuota,
    #[msg("Shipment ID too long")]
    ShipmentIdTooLong,
    #[msg("Invalid GCV Value")]
    InvalidGCVValue,
    #[msg("Invalid moisture content")]
    InvalidMoistureContent,
    #[msg("Invalid ash content")]
    InvalidAshContent,
    #[msg("Invalid sulphur content")]
    InvalidSulphurContent,
    #[msg("Size classification too long")]
    SizeClassificationTooLong,
}

// Helper function
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