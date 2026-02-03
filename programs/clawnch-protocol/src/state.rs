// Clawnch Protocol State
// Defines all PDAs and account structures

use anchor_lang::prelude::*;

/// Fee vault PDA
/// Seeds: ["fee_vault", program_id]
#[account]
#[derive(Debug)]
pub struct FeeVault {
    pub authority: Pubkey,
    pub balance: u64,
}

/// Staking vault PDA
/// Seeds: ["staking", mint, program_id]
#[account]
#[derive(Debug)]
pub struct StakingVault {
    pub staked_amount: u64,
    pub last_update: i64,
}

/// Token mint account
/// Seeds: ["token", token_name, program_id]
#[account]
#[derive(Debug)]
pub struct TokenMint {
    pub mint: Pubkey,
    pub freeze_authority: Pubkey,
    pub mint_authority: Pubkey,
    pub supply: u64,
    pub decimals: u8,
}

/// Token treasury
/// Seeds: ["treasury", mint, program_id]
#[account]
#[derive(Debug)]
pub struct TokenTreasury {
    pub authority: Pubkey,
    pub balance: u64,
}

/// Fee configuration
/// Seeds: ["config", program_id]
#[account]
#[derive(Debug)]
pub struct FeeConfig {
    pub protocol_fee_bps: u16,
    pub creator_fee_bps: u16,
    pub buyback_fee_bps: u16,
    pub staking_fee_bps: u16,
}

/// Token metadata
#[account]
#[derive(Debug)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
}

/// Seeds constants
pub mod seeds {
    pub const FEE_VAULT: &[u8] = b"fee_vault";
    pub const STAKING_VAULT: &[u8] = b"staking";
    pub const TOKEN: &[u8] = b"token";
    pub const TOKEN_NAME: &[u8] = b"token_name";
    pub const TREASURY: &[u8] = b"treasury";
    pub const CONFIG: &[u8] = b"config";
    pub const PROGRAM_ID: &[u8] = b"ClawnchProtocol";
}

/// Default fee distribution (10/20/35/35)
pub mod fees {
    pub const PROTOCOL_BPS: u16 = 1000; // 10%
    pub const CREATOR_BPS: u16 = 2000; // 20%
    pub const BUYBACK_BPS: u16 = 3500; // 35%
    pub const STAKING_BPS: u16 = 3500; // 35%
    pub const TOTAL_BPS: u16 = 10000; // 100%
}
