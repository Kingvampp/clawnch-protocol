# Clawnch SDK

AI agent-owned memecoin launchpad with real tokenomics on Solana.

## Installation

\`\`\n
npm install @kingvampp/clawnch-sdk
\`\`\`

## Quick Start

\`\`\`typescript
import { Clawnch } from '@kingvampp/clawnch-sdk';

// Launch a token
const clawnch = new Clawnch({ apiKey: process.env.CLAWNCH_API_KEY });

const result = await clawnch.launchToken({
  name: 'MyToken',
  symbol: 'TOKEN',
  decimals: 9,
  initialSupply: 1_000_000_000,
  creatorFeePercent: 20,
});

console.log('Token mint:', result.mint);
\`\`\`

## API Methods

### Token Operations
- \`launchToken({ name, symbol, decimals, initialSupply, creatorFeePercent })\`
- \`stake({ amount })\`
- \`unstake({})\`

### Portfolio Operations
- \`getPortfolio({ wallet })\`

### Fee Distribution
- Every trade generates 2% fees distributed as:
  - 10% → Clawnch Protocol (revenue)
  - 20% → Creator (incentive)
  - 35% → Buyback (price support)
  - 35% → Stakers (holder rewards)

## Integration Examples

### With SAID Protocol (Identity)
\`\`\`typescript
import { Clawnch } from '@kingvampp/clawnch-sdk';
import { SAID } from '@kaiclawd/said-sdk';

// Verify agent before allowing token launch
const identity = await SAID.verify({ wallet: '...' });
const launch = await clawnch.launchToken({ ..., identityProof: identity.proof });

\`\`\`

### With SuperRouter (Swap Execution)
\`\`\`typescript
import { Clawnch } from '@kingvampp/clawnch-sdk';

// Execute buyback using treasury funds
const buybackQuote = await SuperRouter.getQuote({ inputMint: '...', outputMint: '...', amount: 1000 });
const buybackTx = await SuperRouter.swap(buybackQuote);

\`\`\`

## Architecture

\`\`\`
Clawnch Protocol
├── API Server (Express.js)
│   ├── Token Management (mints, transfer fees)
│   ├── Staking System (PDA-based vaults)
│   ├── Treasury (buyback pool + rewards)
│   └── Fee Distribution Engine (10/20/35/35)
└── Smart Contracts (Anchor)
    ├── Fee Vault PDA
    ├── Staking Vault PDA
    ├── Token Mint PDAs
    └── Treasury PDA
└── Integrations
    ├── SAID Protocol (identity verification)
    ├── SuperRouter (swap execution)
    ├── AgentVault (portfolio tracking)
    └── SolanaYield (yield data)

\`\`\`

## License

MIT License - See LICENSE file for details.
