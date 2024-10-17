import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { assert } from 'chai';

describe('Staking Tests', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Staking as Program<Staking>;

  let stakingAccount: anchor.web3.Keypair;
  let lpTokenAccount: anchor.web3.Keypair;
  let rewardTokenAccount: anchor.web3.Keypair;
  let userStakeAccount: anchor.web3.Keypair;

  before(async () => {
    stakingAccount = anchor.web3.Keypair.generate();
    lpTokenAccount = anchor.web3.Keypair.generate();
    rewardTokenAccount = anchor.web3.Keypair.generate();
    userStakeAccount = anchor.web3.Keypair.generate();

    await program.rpc.initializeStaking({
      accounts: {
        staking: stakingAccount.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [stakingAccount],
    });
  });
});
