/**
 * Enhanced Demo Page - Clawnch Protocol
 * Shows SDK in action with simuated API calls
 */

import { ClawnchSDK } from './sdk/index.js';

// Initialize SDK
const clawnch = new ClawnchSDK({
  apiKey: 'demo-api-key', // Demo mode
});

// Launch Token Function
async function launchToken() {
  try {
    // Disable button while "processing"
    const launchBtn = document.getElementById('launch-btn');
    launchBtn.disabled = true;
    launchBtn.textContent = 'Launching...';
    launchBtn.className = 'opacity-50 cursor-not-allowed';

    // Simulate API call
    const result = await clawnch.launchToken({
      name: 'DemoToken',
      symbol: 'DEMO',
      decimals: 9,
      initialSupply: 1000000000,
      creatorFeePercent: 20,
    });

    // Update UI after delay (simulate network request)
    setTimeout(() => {
      launchBtn.disabled = false;
      launchBtn.textContent = '🎉 Token Launched!';
      launchBtn.className = 'opacity-100 cursor-allowed';
      
      // Show results
      showTokenResult(result);
    }, 1500);

  } catch (error) {
    console.error('Launch error:', error);
    const launchBtn = document.getElementById('launch-btn');
    launchBtn.disabled = false;
    launchBtn.textContent = 'Failed';
    launchBtn.className = 'opacity-100 cursor-not-allowed text-red-500';
  }
}

// Stake Tokens Function
async function stakeTokens() {
  try {
    const stakeBtn = document.getElementById('stake-btn');
    const amountInput = document.getElementById('stake-amount');
    const amount = amountInput.value ? parseInt(amountInput.value) : 0;

    stakeBtn.disabled = true;
    stakeBtn.textContent = 'Staking...';
    stakeBtn.className = 'opacity-50 cursor-not-allowed';

    // Simulate API call
    const result = await clawnch.stake({
      amount: amount,
    });

    setTimeout(() => {
      stakeBtn.disabled = false;
      stakeBtn.textContent = '✅ Staked!';
      stakeBtn.className = 'opacity-100 cursor-allowed';
      
      showStakeResult(result);
    }, 1000);

  } catch (error) {
    console.error('Stake error:', error);
    const stakeBtn = document.getElementById('stake-btn');
    stakeBtn.disabled = false;
    stakeBtn.textContent = 'Failed';
    stakeBtn.className = 'opacity-100 cursor-not-allowed text-red-500';
  }
}

// Execute Buyback Function
async function executeBuyback() {
  try {
    const buybackBtn = document.getElementById('buyback-btn');
    const amountInput = document.getElementById('buyback-amount');
    const amount = amountInput.value ? parseInt(amountInput.value) : 0;

    buybackBtn.disabled = true;
    buybackBtn.textContent = 'Executing Buyback...';
    buybackBtn.className = 'opacity-50 cursor-not-allowed';

    // Simulate API call
    const result = await clawnch.executeBuyback({
      amount: amount,
    });

    setTimeout(() => {
      buybackBtn.disabled = false;
      buybackBtn.textContent = '💰 Buyback Executed!';
      buybackBtn.className = 'opacity-100 cursor-allowed';
      
      showBuybackResult(result);
    }, 1000);

  } catch (error) {
    console.error('Buyback error:', error);
    const buybackBtn = document.getElementById('buyback-btn');
    buybackBtn.disabled = false;
    buybackBtn.textContent = 'Failed';
    buybackBtn.className = 'opacity-100 cursor-not-allowed text-red-500';
  }
}

// Get Fee Distribution
async function getFeeDistribution() {
  try {
    const feeDist = await clawnch.getFeeDistribution();
    showFeeDistribution(feeDist);
  } catch (error) {
    console.error('Fee distribution error:', error);
    document.getElementById('fee-error').textContent = 'Failed to fetch fees';
  }
}

// Show Token Result
function showTokenResult(result) {
  const output = document.getElementById('launch-output');
  output.innerHTML = `
    <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 shadow-2xl">
      <h3 class="text-2xl font-bold text-purple-400">🎉 Token Launched Successfully!</h3>
      
      <div class="bg-white/20 backdrop-blur-sm rounded-xl p-6 shadow-xl">
        <h4 class="text-lg font-bold text-gray-700 mb-4">Token Details</h4>
        
        <div class="grid md:grid-cols-2 gap-6">
          <div class="bg-white/10 rounded-lg p-6 shadow">
            <div class="text-sm text-gray-500 mb-2">Mint Address</div>
            <div class="text-lg font-mono">${result.mint}</div>
          </div>
          
          <div class="bg-white/10 rounded-lg p-6 shadow">
            <div class="text-sm text-gray-500 mb-2">Symbol</div>
            <div class="text-lg font-mono">${result.symbol}</div>
          </div>
          
          <div class="bg-white/10 rounded-lg p-6 shadow">
            <div class="text-sm text-gray-500 mb-2">Initial Supply</div>
            <div class="text-lg font-mono">${result.initialSupply}</div>
          </div>
          
          <div class="bg-white/10 rounded-lg p-6 shadow">
            <div class="text-sm text-gray-500 mb-2">Creator Fee</div>
            <div class="text-lg font-mono">${result.creatorFeePercent}%</div>
          </div>
          
          <div class="bg-white/10 rounded-lg p-6 shadow">
            <div class="text-sm text-gray-500 mb-2">Fee Distribution</div>
            <div class="text-base">${JSON.stringify(result.feeDistribution, null, 2)}</div>
          </div>
        </div>
      </div>
  `;
}

// Show Stake Result
function showStakeResult(result) {
  const output = document.getElementById('stake-output');
  output.innerHTML = `
    <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 shadow-2xl">
      <h3 class="text-2xl font-bold text-green-400">✅ Staking Successful!</h3>
      
      <div class="bg-white/20 backdrop-blur-sm rounded-xl p-6 shadow-xl">
        <h4 class="text-lg font-bold text-gray-700 mb-4">Staking Details</h4>
        
        <div class="grid md:grid-cols-2 gap-6">
          <div class="bg-white/10 rounded-lg p-6 shadow">
            <div class="text-sm text-gray-500 mb-2">Staking Vault</div>
            <div class="text-lg font-mono">${result.stakingVault}</div>
          </div>
          
          <div class="bg-white/10 rounded-lg p-6 shadow">
            <div class="text-sm text-gray-500 mb-2">Amount Staked</div>
            <div class="text-lg font-mono">${result.stakedAmount}</div>
          </div>
          
          <div class="bg-white/10 rounded-lg p-6 shadow">
            <div class="text-sm text-gray-500 mb-2">Lock Duration</div>
            <div class="text-lg font-mono">${result.lockDuration ? result.lockDuration + 's locked' : 'Unlocked'}</div>
          </div>
          
          <div class="bg-white/10 rounded-lg p-6 shadow">
            <div class="text-sm text-gray-500 mb-2">Current APY</div>
            <div class="text-lg font-mono">${result.rewardsPerYear}% annual</div>
          </div>
        </div>
      </div>
  `;
}

// Show Buyback Result
function showBuybackResult(result) {
  const output = document.getElementById('buyback-output');
  output.innerHTML = `
    <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 shadow-2xl">
      <h3 class="text-2xl font-bold text-blue-400">💰 Buyback Executed Successfully!</h3>
      
      <div class="bg-white/20 backdrop-blur-sm rounded-xl p-6 shadow-xl">
        <h4 class="text-lg font-bold text-gray-700 mb-4">Buyback Details</h4>
        
        <div class="grid md:grid-cols-2 gap-6">
          <div class="bg-white/10 rounded-lg p-6 shadow">
            <div class="text-sm text-gray-500 mb-2">Treasury</div>
            <div class="text-lg font-mono">${result.treasury}</div>
          </div>
          
          <div class="bg-white/10 rounded-lg p-6 shadow">
            <div class="text-sm text-gray-500 mb-2">Buyback Amount</div>
            <div class="text-lg font-mono">${result.amount}</div>
          </div>
          
          <div class="bg-white/10 rounded-lg p-6 shadow">
            <div class="text-sm text-gray-500 mb-2">Price Impact</div>
            <div class="text-lg font-mono">${result.estimatedPriceImpact}</div>
          </div>
        </div>
      </div>
  `;
}

// Show Fee Distribution
function showFeeDistribution(feeDist) {
  const output = document.getElementById('fee-output');
  output.innerHTML = `
    <div class="bg-white/10 backdrop-blur-sm rounded-2xl p-8 shadow-2xl">
      <h3 class="text-2xl font-bold text-purple-400">💰 Fee Distribution</h3>
      
      <div class="bg-white/20 backdrop-blur-sm rounded-xl p-6 shadow-xl">
        <div class="text-center mb-6">
          <div class="inline-block align-middle bg-purple-500 rounded-lg p-4 shadow-2xl">
            <div class="text-sm text-white mb-1">10%</div>
          </div>
          <div class="inline-block align-middle bg-white rounded-lg p-4 shadow-2xl">
            <div class="text-sm text-gray-700">Clawnch Protocol</div>
          </div>
          
          <div class="text-lg text-purple-400 font-bold">10%</div>
          </div>
          
          <div class="inline-block align-middle bg-purple-500 rounded-lg p-4 shadow-2xl">
            <div class="text-sm text-white mb-1">20%</div>
          </div>
          <div class="inline-block align-middle bg-white rounded-lg p-4 shadow-2xl">
            <div class="text-sm text-gray-700">20%</div>
          </div>
          <div class="text-lg text-orange-400 font-bold">20%</div>
          </div>
          
          <div class="inline-block align-middle bg-blue-500 rounded-lg p-4 shadow-2xl">
            <div class="text-sm text-white mb-1">35%</div>
          </div>
          <div class="text-lg text-blue-400 font-bold">35%</div>
          </div>
          
          <div class="inline-block align-middle bg-green-500 rounded-lg p-4 shadow-2xl">
            <div class="text-sm text-white mb-1">35%</div>
          </div>
          <div class="text-lg text-green-400 font-bold">35%</div>
          </div>
          
          <div class="mt-6 text-center">
            <p class="text-sm text-gray-500">Stakers</p>
            <p class="text-sm text-gray-700">Buyback Fund</p>
            <p class="text-sm text-gray-500">Holder Rewards</p>
          </div>
          
          <div class="flex justify-between items-center">
            <div>
              <div class="text-sm text-gray-500">Clawnch Protocol</div>
              <div class="text-xs text-gray-400">Revenue</div>
            </div>
            <div>
              <div class="text-sm text-gray-500">Creator</div>
              <div class="text-xs text-gray-400">Incentive</div>
            </div>
          </div>
        </div>
      </div>
      
      <div class="mt-8 text-center">
        <p class="text-sm text-purple-400 font-bold">10/20/35/35 Split</p>
        <p class="text-xs text-gray-500">Fee Distribution Breakdown per 100m tokens traded</p>
      </div>
  `;
}

// Initialize on load
window.onload = () => {
  // Show initial state
  showTokenResult({
    mint: 'DemoTokenMockMintAddress',
    symbol: 'DEMO',
    initialSupply: 0,
    creatorFeePercent: 0,
    feeDistribution: null,
  });
  
  // Update status text
  const statusText = document.getElementById('sdk-status');
  if (statusText) {
    statusText.textContent = '🟢 Clawnch SDK Connected (Demo Mode)';
    statusText.className = 'text-green-400 font-medium';
  }
};
