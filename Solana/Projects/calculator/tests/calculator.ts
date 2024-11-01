import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Calculator } from '../target/types/calculator';
import { assert, expect } from 'chai';

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

  it('subtracts two numbers, handle underflow', async () => {
    try {
      const tx = await program.methods.subtract(new anchor.BN(2), new anchor.BN(10)).rpc();
    } catch (error) {
      expect(error).to.exist;
      expect(error.error.errorCode.code.toString()).to.equal('Underflow');
    }
  });

  it('multiplies two numbers', async () => {
    const tx = await program.methods.multiply(new anchor.BN(10), new anchor.BN(2)).rpc();
  });

  it('divides two numbers', async () => {
    const tx = await program.methods.divide(new anchor.BN(10), new anchor.BN(2)).rpc();
  });

  it('calculates the square root of a number', async () => {
    const tx = await program.methods.squareRoot(16).rpc();
  });

  it('calculates the log10 of a number', async () => {
    const tx = await program.methods.log10(100).rpc();
  });
});
