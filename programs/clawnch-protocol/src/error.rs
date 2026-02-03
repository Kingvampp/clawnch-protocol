use anchor_lang::prelude::*;

#[error_code]
pub enum ClawnchError {
    #[msg("Invalid fee configuration - must total 100%")]
    InvalidFeeConfig,

    #[msg("Staking vault overflow")]
    StakingOverflow,

    #[msg("Insufficient treasury balance")]
    InsufficientTreasuryBalance,

    #[msg("Insufficient treasury balance for rewards")]
    InsufficientTreasuryForRewards,

    #[msg("Invalid amount - must be greater than 0")]
    InvalidAmount,

    #[msg("Nothing staked in vault")]
    NothingStaked,

    #[msg("Unauthorized - not the vault owner")]
    Unauthorized,
}
