import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Day02 } from '../target/types/day_02';

describe('day_02', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day02 as Program<Day02>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.methods.initialize(new anchor.BN(777), new anchor.BN(888), 'Hello!').rpc();
    console.log('Your transaction signature', tx);
  });

  it('Array', async () => {
    const tx = await program.methods.array([new anchor.BN(777), new anchor.BN(888), new anchor.BN(999)]).rpc();
    console.log('Your transaction signature', tx);
  });

  it('Subtract not safe', async () => {
    const tx = await program.methods.subtractNotsafe(new anchor.BN(1), new anchor.BN(2)).rpc();
    console.log('Your transaction signature', tx);
  });

  it('Subtract safe', async () => {
    const tx = await program.methods.subtractSafe(new anchor.BN(1), new anchor.BN(2)).rpc();
    console.log('Your transaction signature', tx);
  });
});
