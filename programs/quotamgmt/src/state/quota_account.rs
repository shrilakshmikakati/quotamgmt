use anchor_lang::prelude::*;

/// Main quota account storing concession quota information
#[account]
pub struct QuotaAccount {
    /// Unique identifier for the mining concession (max 32 chars)
    pub concession_id: String,
    /// Public key of the quota holder
    pub holder: Pubkey,
    /// Public key of the regulator who issued the quota
    pub regulator: Pubkey,
    /// Total allocated quota in metric tons
    pub allocated_quota: u64,
    /// Amount of quota already used
    pub used_quota: u64,
    /// Available quota remaining
    pub available_quota: u64,
    /// Unix timestamp when quota expires
    pub validity_period: i64,
    /// Current status of the quota
    pub status: QuotaStatus,
    /// Type of quota (annual, monthly, special)
    pub quota_type: QuotaType,
    /// Geographic region or mine location
    pub mining_region: String,
    /// Environmental clearance reference
    pub environmental_clearance: String,
    /// Timestamp when quota was created
    pub created_at: i64,
    /// Timestamp when quota was last updated
    pub updated_at: i64,
    /// PDA bump seed
    pub bump: u8,
}

impl QuotaAccount {
    pub const MAX_CONCESSION_ID_LEN: usize = 32;
    pub const MAX_MINING_REGION_LEN: usize = 64;
    pub const MAX_ENV_CLEARANCE_LEN: usize = 64;
    
    pub const LEN: usize = 8 + // discriminator
        4 + Self::MAX_CONCESSION_ID_LEN + // concession_id
        32 + // holder
        32 + // regulator
        8 + // allocated_quota
        8 + // used_quota
        8 + // available_quota
        8 + // validity_period
        1 + 1 + // status (enum + padding)
        1 + 1 + // quota_type (enum + padding)
        4 + Self::MAX_MINING_REGION_LEN + // mining_region
        4 + Self::MAX_ENV_CLEARANCE_LEN + // environmental_clearance
        8 + // created_at
        8 + // updated_at
        1; // bump

    /// Check if quota is valid and active
    pub fn is_valid(&self) -> bool {
        self.status == QuotaStatus::Active && 
        Clock::get().unwrap().unix_timestamp <= self.validity_period
    }

    /// Check if enough quota is available
    pub fn has_sufficient_quota(&self, amount: u64) -> bool {
        self.available_quota >= amount
    }

    /// Calculate utilization percentage
    pub fn utilization_percentage(&self) -> u8 {
        if self.allocated_quota == 0 {
            return 0;
        }
        ((self.used_quota as f64 / self.allocated_quota as f64) * 100.0) as u8
    }
}

/// Record of quota usage for each shipment
#[account]
pub struct UsageRecord {
    /// Concession ID this usage belongs to
    pub concession_id: String,
    /// Unique shipment identifier
    pub shipment_id: String,
    /// Amount of quota used (metric tons)
    pub amount: u64,
    /// Timestamp when quota was used
    pub timestamp: i64,
    /// Public key of the quota holder
    pub holder: Pubkey,
    /// Quality parameters of the coal
    pub quality_params: QualityParameters,
    /// Source location (mine/loading point)
    pub source_location: String,
    /// Destination location
    pub destination_location: String,
    /// Transport company/vehicle details
    pub transport_details: String,
    /// PDA bump seed
    pub bump: u8,
}

impl UsageRecord {
    pub const MAX_CONCESSION_ID_LEN: usize = 32;
    pub const MAX_SHIPMENT_ID_LEN: usize = 32;
    pub const MAX_LOCATION_LEN: usize = 100;
    pub const MAX_TRANSPORT_DETAILS_LEN: usize = 200;
    
    pub const LEN: usize = 8 + // discriminator
        4 + Self::MAX_CONCESSION_ID_LEN + // concession_id
        4 + Self::MAX_SHIPMENT_ID_LEN + // shipment_id
        8 + // amount
        8 + // timestamp
        32 + // holder
        QualityParameters::LEN + // quality_params
        4 + Self::MAX_LOCATION_LEN + // source_location
        4 + Self::MAX_LOCATION_LEN + // destination_location
        4 + Self::MAX_TRANSPORT_DETAILS_LEN + // transport_details
        1; // bump
}

/// Quality parameters for coal shipments
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct QualityParameters {
    /// Gross Calorific Value in kcal/kg
    pub gross_calorific_value: u32,
    /// Moisture content percentage (scaled by 100, e.g., 1050 = 10.50%)
    pub moisture_content: u16,
    /// Ash content percentage (scaled by 100)
    pub ash_content: u16,
    /// Sulphur content percentage (scaled by 100)
    pub sulphur_content: u16,
    /// Volatile matter percentage (scaled by 100)
    pub volatile_matter: u16,
    /// Fixed carbon percentage (scaled by 100)
    pub fixed_carbon: u16,
    /// Coal grade classification
    pub coal_grade: CoalGrade,
    /// Size classification
    pub size_classification: String,
}

impl QualityParameters {
    pub const MAX_SIZE_CLASSIFICATION_LEN: usize = 20;
    
    pub const LEN: usize = 
        4 + // gross_calorific_value
        2 + // moisture_content
        2 + // ash_content
        2 + // sulphur_content
        2 + // volatile_matter
        2 + // fixed_carbon
        1 + 1 + // coal_grade (enum + padding)
        4 + Self::MAX_SIZE_CLASSIFICATION_LEN; // size_classification
}

/// Transfer record for quota transfers between concessions
#[account]
pub struct TransferRecord {
    /// Source concession ID
    pub from_concession: String,
    /// Destination concession ID
    pub to_concession: String,
    /// Amount transferred
    pub amount: u64,
    /// Transfer timestamp
    pub timestamp: i64,
    /// Authorized by (regulator or holder)
    pub authorized_by: Pubkey,
    /// Transfer reason/description
    pub transfer_reason: String,
    /// Transfer type
    pub transfer_type: TransferType,
    /// PDA bump seed
    pub bump: u8,
}

impl TransferRecord {
    pub const MAX_CONCESSION_ID_LEN: usize = 32;
    pub const MAX_TRANSFER_REASON_LEN: usize = 200;
    
    pub const LEN: usize = 8 + // discriminator
        4 + Self::MAX_CONCESSION_ID_LEN + // from_concession
        4 + Self::MAX_CONCESSION_ID_LEN + // to_concession
        8 + // amount
        8 + // timestamp
        32 + // authorized_by
        4 + Self::MAX_TRANSFER_REASON_LEN + // transfer_reason
        1 + 1 + // transfer_type (enum + padding)
        1; // bump
}

// Enums

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum QuotaStatus {
    /// Quota is active and can be used
    Active,
    /// Quota is temporarily suspended
    Suspended,
    /// Quota has expired
    Expired,
    /// Quota has been revoked
    Revoked,
    /// Quota is fully utilized
    Exhausted,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum QuotaType {
    /// Annual quota allocation
    Annual,
    /// Monthly quota allocation
    Monthly,
    /// Special/Emergency quota
    Special,
    /// Temporary additional quota
    Supplementary,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum CoalGrade {
    /// Grade A (high quality)
    GradeA,
    /// Grade B (medium quality)
    GradeB,
    /// Grade C (standard quality)
    GradeC,
    /// Grade D (low quality)
    GradeD,
    /// Grade E (very low quality)
    GradeE,
    /// Non-coking coal
    NonCoking,
    /// Semi-coking coal
    SemiCoking,
    /// Prime coking coal
    PrimeCoking,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum TransferType {
    /// Emergency transfer due to operational needs
    Emergency,
    /// Planned transfer between related entities
    Planned,
    /// Regulatory mandated transfer
    Regulatory,
    /// Commercial sale/transfer
    Commercial,
}

// Events

#[event]
pub struct QuotaInitialized {
    pub concession_id: String,
    pub holder: Pubkey,
    pub allocated_quota: u64,
    pub validity_period: i64,
    pub quota_type: QuotaType,
    pub mining_region: String,
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

#[event]
pub struct QuotaTransferred {
    pub from_concession: String,
    pub to_concession: String,
    pub amount: u64,
    pub transfer_type: TransferType,
    pub authorized_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct QuotaStatusUpdated {
    pub concession_id: String,
    pub old_status: QuotaStatus,
    pub new_status: QuotaStatus,
    pub updated_by: Pubkey,
    pub reason: String,
    pub timestamp: i64,
}

#[event]
pub struct QuotaUpdated {
    pub concession_id: String,
    pub old_allocated_quota: u64,
    pub new_allocated_quota: u64,
    pub old_validity_period: i64,
    pub new_validity_period: i64,
    pub updated_by: Pubkey,
    pub timestamp: i64,
}