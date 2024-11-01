import * as anchor from '@project-serum/anchor';
import { Program } from 
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

  it('Stake LP Tokens with Time Lock', async () => {
    const stakeAmount = new anchor.BN(1000);
    const lockDuration = new anchor.BN(86400); // 1 day

    await program.rpc.stakeWithTimeLock(stakeAmount, lockDuration, {
      accounts: {
        staking: stakingAccount.publicKey,
        userStake: userStakeAccount.publicKey,
        lpToken: lpTokenAccount.publicKey,
        user: provider.wallet.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      },
      signers: [userStakeAccount],
    });

    const stakingData = await program.account.staking.fetch(stakingAccount.publicKey);
    const userStakeData = await program.account.userStake.fetch(userStakeAccount.publicKey);

    assert.equal(stakingData.totalStaked.toNumber(), 1000);
    assert.equal(userStakeData.amount.toNumber(), 1000);
  });
});
