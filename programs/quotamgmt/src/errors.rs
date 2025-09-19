use anchor_lang::prelude::*;

#[error_code]
pub enum QuotaError {
    #[msg("Invalid quota amount - must be greater than 0")]
    InvalidQuotaAmount,
    
    #[msg("Invalid validity period - must be in the future")]
    InvalidValidityPeriod,
    
    #[msg("Concession ID too long - maximum 32 characters")]
    ConcessionIdTooLong,
    
    #[msg("Quota is not active - cannot perform operation")]
    QuotaNotActive,
    
    #[msg("Quota has expired")]
    QuotaExpired,
    
    #[msg("Invalid usage amount - must be greater than 0")]
    InvalidUsageAmount,
    
    #[msg("Insufficient quota available")]
    InsufficientQuota,
    
    #[msg("Shipment ID too long - maximum 32 characters")]
    ShipmentIdTooLong,
    
    #[msg("Invalid transfer amount - must be greater than 0")]
    InvalidTransferAmount,
    
    #[msg("Reason too long - maximum 200 characters")]
    ReasonTooLong,
    
    #[msg("Mining region too long - maximum 64 characters")]
    MiningRegionTooLong,
    
    #[msg("Environmental clearance reference too long - maximum 64 characters")]
    EnvironmentalClearanceTooLong,
    
    #[msg("Location name too long - maximum 100 characters")]
    LocationTooLong,
    
    #[msg("Transport details too long - maximum 200 characters")]
    TransportDetailsTooLong,
    
    #[msg("Size classification too long - maximum 20 characters")]
    SizeClassificationTooLong,
    
    #[msg("Unauthorized - only regulator can perform this action")]
    UnauthorizedRegulator,
    
    #[msg("Unauthorized - only quota holder can perform this action")]
    UnauthorizedHolder,
    
    #[msg("Cannot transfer to the same concession")]
    SelfTransferNotAllowed,
    
    #[msg("Invalid quality parameters")]
    InvalidQualityParameters,
    
    #[msg("Quota already exhausted")]
    QuotaExhausted,
    
    #[msg("Cannot modify expired quota")]
    CannotModifyExpiredQuota,
    
    #[msg("Invalid coal grade")]
    InvalidCoalGrade,
    
    #[msg("Transfer amount exceeds available quota")]
    TransferAmountExceedsAvailable,
    
    #[msg("Quota utilization threshold exceeded")]
    UtilizationThresholdExceeded,
    
    #[msg("Duplicate shipment ID")]
    DuplicateShipmentId,
    
    #[msg("Invalid GCV value - must be between 2000-8000 kcal/kg")]
    InvalidGCVValue,
    
    #[msg("Invalid moisture content - must be between 0-50%")]
    InvalidMoistureContent,
    
    #[msg("Invalid ash content - must be between 0-50%")]
    InvalidAshContent,
    
    #[msg("Invalid sulphur content - must be between 0-10%")]
    InvalidSulphurContent,
}