use anchor_lang::error_code;

#[error_code]
pub enum ClawnchError {
    #[msg("Insufficient treasury balance")]
    InsufficientTreasuryBalance,

    #[msg("Insufficient treasury balance for rewards")]
    InsufficientTreasuryForRewards,

    #[msg("Staking vault overflow")]
    StakingOverflow,

    #[msg("Invalid fee configuration")]
    InvalidFeeConfig,
}
