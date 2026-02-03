# Clawnch Protocol - Smart Contracts

Solana smart contracts written in Anchor framework.

## Program Structure

```
src/
├── lib.rs          # Main program logic
├── state.rs        # PDAs and account structures
└── error.rs        # Custom error types
```

## Instructions

### `initialize_config`
Initialize the protocol with fee distribution percentages.
- Default: 10% protocol, 20% creator, 35% buyback, 35% staking

### `create_token`
Create a new SPL Token-2022 with Clawnch tokenomics built in.
Features:
- Transfer fee extension (2% on all transfers)
- Automatic fee distribution
- Creator tracking

### `distribute_fees`
Collect and distribute 2% fee from each trade:
- 10% → Fee vault (protocol revenue)
- 20% → Creator (incentive)
- 35% → Treasury (buyback pool)
- 35% → Staking vault (reward pool)

### `execute_buyback`
Execute buyback from treasury funds.
Authority can spend treasury tokens to buy back tokens from the market.

### `stake`
Stake tokens to earn rewards from the 35% fee share.

### `unstake`
Unstake tokens and claim rewards based on staking duration.

## PDAs (Program Derived Addresses)

| PDA | Seeds | Purpose |
|-----|--------|---------|
| FeeVault | `["fee_vault", program_id]` | Stores protocol fees |
| StakingVault | `["staking", mint, program_id]` | Staking pool + rewards |
| TokenMint | `["token", token_name, program_id]` | Token mint with extensions |
| TokenTreasury | `["treasury", mint, program_id]` | Buyback + reward funds |
| FeeConfig | `["config", program_id]` | Fee distribution config |

## To Build

```bash
# Install dependencies (if needed)
cargo install solana-program

# Build the program
anchor build

# Test on devnet
anchor test

# Deploy
anchor deploy
```

## Program ID

```
ClawnchProtocol
```

## Next Steps

1. Set up development environment (Rust + Anchor)
2. Write tests for each instruction
3. Deploy to devnet and verify
4. Integrate with API server
