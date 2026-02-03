// Clawnch Protocol - Solana Smart Contracts
// Built with Anchor framework (anchor-lang)
// To build: anchor build
// To deploy: anchor deploy

use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("ClawnchProtocol");

#[program]
pub mod clawnch {
    use super::*;
    use crate::state::*;
    use crate::error::ClawnchError;

    /// Fee vault PDA
    /// Stores collected fees before distribution
    /// Seeds: ["fee_vault", program_id]
    #[account]
    pub struct FeeVault<'info> {
        #[constraint(mut)]
        pub authority: Signer<'info>,
        pub bump: u8,
    }

    /// Staking vault PDA
    /// Stores staked tokens and reward calculations
    /// Seeds: ["staking", mint, program_id]
    #[account]
    pub struct StakingVault<'info> {
        #[constraint(mut)]
        pub authority: Signer<'info>,
        pub staked_amount: u64,
        pub last_update: i64,
        pub bump: u8,
    }

    /// Token mint with Token-2022 extensions
    /// Seeds: ["token", token_name, program_id]
    #[account]
    pub struct TokenMint<'info> {
        #[constraint(mut)]
        pub mint: Signer<'info>,
        #[constraint(address = mint.key())]
        pub freeze_authority: Signer<'info>,
        pub mint_authority: Signer<'info>,
        pub supply: u64,
        pub decimals: u8,
        #[constraint(address = token::spl_token_2022::extensions::transfer_fee::ID)]
        pub transfer_fee_config: Account<'info, Mutable>,
        pub bump: u8,
    }

    /// Token treasury PDA
    /// Stores tokens for buybacks and staking rewards
    /// Seeds: ["treasury", mint, program_id]
    #[account]
    pub struct TokenTreasury<'info> {
        #[constraint(mut)]
        pub authority: Signer<'info>,
        pub balance: u64,
        pub bump: u8,
    }

    /// Fee distribution config
    /// Seeds: ["config", program_id]
    #[account]
    pub struct FeeConfig<'info> {
        #[constraint(mut)]
        pub authority: Signer<'info>,
        pub protocol_fee_bps: u16, // 10% = 1000 bps
        pub creator_fee_bps: u16, // 20% = 2000 bps
        pub buyback_fee_bps: u16, // 35% = 3500 bps
        pub staking_fee_bps: u16, // 35% = 3500 bps
        pub bump: u8,
    }

    /// Token metadata
    #[account]
    pub struct TokenMetadata<'info> {
        #[constraint(mut)]
        pub name: String,
        pub symbol: String,
        pub bump: u8,
    }

    // ==================== INSTRUCTIONS ====================

    /// Initialize the protocol with fee config
    #[instruction]
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        protocol_fee_bps: u16,
        creator_fee_bps: u16,
        buyback_fee_bps: u16,
        staking_fee_bps: u16,
    ) -> Result<()> {
        // Default fee distribution: 10/20/35/35
        let fee_config = FeeConfig {
            authority: ctx.accounts.fee_config.authority,
            protocol_fee_bps,
            creator_fee_bps,
            buyback_fee_bps,
            staking_fee_bps,
            bump: ctx.bumps.fee_config,
        };

        // Create fee config account
        // SystemProgram::create_account would go here

        emit!(ConfigInitialized {
            authority: *ctx.accounts.fee_config.authority.key,
            protocol_fee: protocol_fee_bps,
            creator_fee: creator_fee_bps,
            buyback_fee: buyback_fee_bps,
            staking_fee: staking_fee_bps,
        });

        Ok(())
    }

    /// Create a new token with Clawnch tokenomics
    #[instruction]
    pub fn create_token(
        ctx: Context<CreateToken>,
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: u64,
        creator: Pubkey,
    ) -> Result<()> {
        let seeds = [b"token", name.as_bytes(), ctx.accounts.token_mint.mint.key().as_ref()];
        let bump = ctx.bumps.token_mint.bump;

        // Initialize mint with Token-2022 extensions
        let mint = anchor_spl::spl_token_2022::instructions::initialize_mint2(
            ctx.accounts.token_mint.mint,
            &ctx.accounts.token_mint.freeze_authority,
            &ctx.accounts.token_mint.mint_authority,
            None,
            decimals,
            &ctx.accounts.token_mint.supply,
            &ctx.accounts.token_mint.transfer_fee_config,
            None, // Update authority
            None, // Conf close authority
            None,
        );

        // Create metadata
        let metadata = TokenMetadata {
            name: name.clone(),
            symbol: symbol.clone(),
            bump: ctx.bumps.token_metadata,
        };

        emit!(TokenCreated {
            mint: ctx.accounts.token_mint.mint.key(),
            name,
            symbol,
            decimals,
            initial_supply,
            creator: *ctx.accounts.creator.key,
        });

        Ok(())
    }

    /// Collect fees from a trade and distribute
    #[instruction]
    pub fn distribute_fees(
        ctx: Context<DistributeFees>,
        fee_amount: u64,
        creator: Pubkey,
    ) -> Result<()> {
        let protocol_fee = (fee_amount * ctx.accounts.fee_config.protocol_fee_bps as u64) / 10000;
        let creator_fee = (fee_amount * ctx.accounts.fee_config.creator_fee_bps as u64) / 10000;
        let buyback_fee = (fee_amount * ctx.accounts.fee_config.buyback_fee_bps as u64) / 10000;
        let staking_fee = (fee_amount * ctx.accounts.fee_config.staking_fee_bps as u64) / 10000;

        // Add protocol fee to fee vault
        **ctx.accounts.fee_vault.balance += protocol_fee;

        // Transfer creator fee to creator
        **anchor_spl::token::transfer(ctx.accounts.token_mint, ctx.accounts.fee_vault, creator, creator_fee);**
        // This would use the system program

        // Transfer buyback fee to treasury
        **anchor_spl::token::transfer(ctx.accounts.token_mint, ctx.accounts.fee_vault, ctx.accounts.treasury, buyback_fee);**

        // Transfer staking fee to staking vault (for reward pool)
        **anchor_spl::token::transfer(ctx.accounts.token_mint, ctx.accounts.fee_vault, ctx.accounts.staking_vault, staking_fee);**

        emit!(FeesDistributed {
            total_fee: fee_amount,
            protocol_fee,
            creator_fee,
            buyback_fee,
            staking_fee,
            fee_vault: *ctx.accounts.fee_vault.key,
        });

        Ok(())
    }

    /// Execute buyback using treasury tokens
    #[instruction]
    pub fn execute_buyback(
        ctx: Context<ExecuteBuyback>,
        amount: u64,
    ) -> Result<()> {
        let bump = ctx.bumps.treasury.bump;

        // Treasury must have enough tokens for buyback
        require!(
            ctx.accounts.treasury.balance >= amount,
            ClawnchError::InsufficientTreasuryBalance
        );

        // Transfer tokens from treasury to burn account or back to LP
        // This would be done via Jupiter integration
        **// anchor_spl::token::transfer(ctx.accounts.token_mint, ctx.accounts.treasury, ctx.accounts.recipient, amount);**

        emit!(BuybackExecuted {
            treasury: *ctx.accounts.treasury.key,
            amount,
            authority: *ctx.accounts.treasury.authority.key,
        });

        Ok(())
    }

    /// Stake tokens to earn rewards
    #[instruction]
    pub fn stake(
        ctx: Context<StakeTokens>,
        amount: u64,
    ) -> Result<()> {
        let bump = ctx.bumps.staking_vault.bump;

        // Transfer staked tokens from user to vault
        **// anchor_spl::token::transfer(ctx.accounts.token_mint, ctx.accounts.user, ctx.accounts.staking_vault, amount);**

        // Update vault state
        ctx.accounts.staking_vault.staked_amount += amount;
        ctx.accounts.staking_vault.last_update = Clock::get().unix_timestamp();

        emit!(TokensStaked {
            user: *ctx.accounts.user.key,
            amount,
            vault: *ctx.accounts.staking_vault.key,
        });

        Ok(())
    }

    /// Unstake tokens and claim rewards
    #[instruction]
    pub fn unstake(
        ctx: Context<UnstakeTokens>,
    ) -> Result<()> {
        let bump = ctx.bumps.staking_vault.bump;

        // Calculate rewards based on time staked
        let now = Clock::get().unix_timestamp();
        let duration = now - ctx.accounts.staking_vault.last_update;
        let staked_amount = ctx.accounts.staking_vault.staked_amount;

        // Simple reward calculation (1% annual, proportional to duration)
        let reward = (staked_amount * duration as u64) / (365 * 24 * 3600);

        require!(
            ctx.accounts.treasury.balance >= staked_amount + reward,
            ClawnchError::InsufficientTreasuryForRewards
        );

        // Transfer staked amount + reward to user
        **// anchor_spl::token::transfer(ctx.accounts.token_mint, ctx.accounts.treasury, ctx.accounts.user, staked_amount + reward);**

        // Reset vault
        ctx.accounts.staking_vault.staked_amount = 0;
        ctx.accounts.staking_vault.last_update = now;

        emit!(TokensUnstaked {
            user: *ctx.accounts.user.key,
            principal: staked_amount,
            reward,
            vault: *ctx.accounts.staking_vault.key,
        });

        Ok(())
    }

    // ==================== ERRORS ====================

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
}

// ==================== EVENTS ====================

#[event]
pub struct ConfigInitialized {
    pub authority: Pubkey,
    pub protocol_fee_bps: u16,
    pub creator_fee_bps: u16,
    pub buyback_fee_bps: u16,
    pub staking_fee_bps: u16,
}

#[event]
pub struct TokenCreated {
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_supply: u64,
    pub creator: Pubkey,
}

#[event]
pub struct FeesDistributed {
    pub total_fee: u64,
    pub protocol_fee: u64,
    pub creator_fee: u64,
    pub buyback_fee: u64,
    pub staking_fee: u64,
    pub fee_vault: Pubkey,
}

#[event]
pub struct BuybackExecuted {
    pub treasury: Pubkey,
    pub amount: u64,
    pub authority: Pubkey,
}

#[event]
pub struct TokensStaked {
    pub user: Pubkey,
    pub amount: u64,
    pub vault: Pubkey,
}

#[event]
pub struct TokensUnstaked {
    pub user: Pubkey,
    pub principal: u64,
    pub reward: u64,
    pub vault: Pubkey,
}
