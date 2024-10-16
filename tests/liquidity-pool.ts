import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { assert } from 'chai';

describe('Liquidity Pool Tests', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.LiquidityPool as Program<LiquidityPool>;

  let poolAccount: anchor.web3.Keypair;
  let tokenAAccount: anchor.web3.Keypair;
  let tokenBAccount: anchor.web3.Keypair;
  let lpTokenMint: anchor.web3.Keypair;

  before(async () => {
    poolAccount = anchor.web3.Keypair.generate();
    tokenAAccount = anchor.web3.Keypair.generate();
    tokenBAccount = anchor.web3.Keypair.generate();
    lpTokenMint = anchor.web3.Keypair.generate();

    await program.rpc.initializePool({
      accounts: {
        pool: poolAccount.publicKey,
        authority: provider.wallet.publicKey
      },
      signers: [poolAccount],
    });
  });

  it('Add Liquidity to the Pool', async () => {
    const amountA = new anchor.BN(1000);
    const amountB = new anchor.BN(500);

    await program.rpc.addLiquidity(amountA, amountB, {
      accounts: {
        pool: poolAccount.publicKey,
        tokenA: tokenAAccount.publicKey,
        tokenB: tokenBAccount.publicKey,
        lpMint: lpTokenMint.publicKey,
        user: provider.wallet.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      },
    });

    const poolData = await program.account.pool.fetch(poolAccount.publicKey);
    assert.equal(poolData.tokenAReserve.toNumber(), 1000);
    assert.equal(poolData.tokenBReserve.toNumber(), 500);
  });
});
