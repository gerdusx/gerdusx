import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Calculator } from '../target/types/calculator';
import { assert } from 'chai';

describe('calculator', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Calculator as Program<Calculator>;

  it('adds two numbers', async () => {
    const tx = await program.methods.add(new anchor.BN(1), new anchor.BN(2)).rpc();
  });

  it('subtracts two numbers', async () => {
    const tx = await program.methods.subtract(new anchor.BN(10), new anchor.BN(2)).rpc();
  });
});
