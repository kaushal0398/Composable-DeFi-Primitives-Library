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
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [poolAccount],
    });
  });