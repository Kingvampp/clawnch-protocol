import express, { Request, Response } from 'express';
import { Connection, Keypair, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { ClawnchToken, TokenConfig } from './token';

const app = express();
const PORT = process.env.PORT || 3000;

// Use devnet for hackathon, mainnet for production
const connection = new Connection(clusterApiUrl('devnet'));

/**
 * POST /launch-token
 * Create a new memecoin with Clawnch tokenomics
 */
app.post('/api/launch-token', async (req: Request, res: Response) => {
  try {
    const {
      name,
      symbol,
      decimals = 9,
      initialSupply = 1_000_000_000,
      feeBasisPoints = 200, // 2%
      creatorWallet,
    } = req.body;

    // Validate input
    if (!name || !symbol || !creatorWallet) {
      return res.status(400).json({
        error: 'Missing required fields: name, symbol, creatorWallet',
      });
    }

    // Clawnch protocol addresses (these would be deployed)
    const protocolWallet = new PublicKey('ClawnchProtocolWalletPlaceholder');
    const stakingVault = new PublicKey('ClawnchStakingVaultPlaceholder');

    const config: TokenConfig = {
      name,
      symbol,
      decimals,
      initialSupply,
      feeBasisPoints,
      creatorWallet: new PublicKey(creatorWallet),
      protocolWallet,
      stakingVault,
    };

    const clawnch = new ClawnchToken(connection, Keypair.generate());

    const mint = await clawnch.createToken(config);

    res.json({
      success: true,
      mint: mint.toString(),
      name,
      symbol,
      decimals,
      initialSupply,
      feePercent: feeBasisPoints / 100,
      feeDistribution: {
        protocol: '10%',
        creator: '20%',
        buyback: '35%',
        staking: '35%',
      },
    });
  } catch (error: any) {
    console.error('Error launching token:', error);
    res.status(500).json({ error: error.message });
  }
});

/**
 * GET /health
 * Health check endpoint
 */
app.get('/health', (_req: Request, res: Response) => {
  res.json({ status: 'ok', protocol: 'Clawnch', version: '1.0.0' });
});

/**
 * GET /stats
 * Protocol statistics
 */
app.get('/api/stats', async (_req: Request, res: Response) => {
  // TODO: Query on-chain for real stats
  res.json({
    totalTokensLaunched: 0,
    totalFeesCollected: '0',
    totalBuybacks: '0',
    totalStaked: '0',
  });
});

app.listen(PORT, () => {
  console.log(`🚀 Clawnch Protocol API running on port ${PORT}`);
  console.log(`Network: ${clusterApiUrl('devnet')}`);
});
