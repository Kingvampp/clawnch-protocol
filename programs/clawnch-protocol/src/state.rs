// Clawnch Protocol State
// Defines all PDAs and account structures

use anchor_lang::prelude::*;

/// Fee configuration PDA
/// Seeds: ["config"]
#[account]
#[derive(Debug)]
pub struct FeeConfig {
    pub authority: Pubkey,
    pub protocol_fee_bps: u16, // 10% = 1000 bps
    pub creator_fee_bps: u16,  // 20% = 2000 bps
    pub buyback_fee_bps: u16,  // 35% = 3500 bps
    pub staking_fee_bps: u16,  // 35% = 3500 bps
    pub bump: u8,
}

impl FeeConfig {
    pub const SPACE: usize = 32 + 2 + 2 + 2 + 2 + 1; // 41 bytes
}

/// Staking vault PDA (per user)
/// Seeds: ["staking", user_pubkey]
#[account]
#[derive(Debug)]
pub struct StakingVault {
    pub user: Pubkey,
    pub staked_amount: u64,
    pub last_update: i64,
    pub bump: u8,
}

impl StakingVault {
    pub const SPACE: usize = 32 + 8 + 8 + 1; // 49 bytes
}

/// Fee vault PDA (collects protocol fees)
/// Seeds: ["fee_vault"]
#[account]
#[derive(Debug)]
pub struct FeeVault {
    pub authority: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

impl FeeVault {
    pub const SPACE: usize = 32 + 8 + 1; // 41 bytes
}

/// Token treasury PDA
/// Seeds: ["treasury", mint]
#[account]
#[derive(Debug)]
pub struct TokenTreasury {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

impl TokenTreasury {
    pub const SPACE: usize = 32 + 32 + 8 + 1; // 73 bytes
}

/// Default fee distribution (10/20/35/35)
pub mod fees {
    pub const PROTOCOL_BPS: u16 = 1000; // 10%
    pub const CREATOR_BPS: u16 = 2000;  // 20%
    pub const BUYBACK_BPS: u16 = 3500;  // 35%
    pub const STAKING_BPS: u16 = 3500;  // 35%
    pub const TOTAL_BPS: u16 = 10000;   // 100%
}
