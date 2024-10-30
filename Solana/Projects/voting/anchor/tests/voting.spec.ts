import * as anchor from '@coral-xyz/anchor';
import { BN, Program } from '@coral-xyz/anchor';
import { Keypair, PublicKey } from '@solana/web3.js';
import { startAnchor } from 'solana-bankrun';
import { BankrunProvider } from 'anchor-bankrun';
import { Voting } from '../target/types/voting';
const IDL = require('../target/idl/voting.json');

const votingAddress = new PublicKey('AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ');

describe('voting', () => {
  it('Initialize Poll', async () => {
    const context = await startAnchor('', [{ name: 'voting', programId: votingAddress }], []);
    const provider = new BankrunProvider(context);

    const votingProgram = new Program<Voting>(IDL, provider);

    const pollId = new anchor.BN(1);
    const description = 'Test Poll';
    const name = 'Test Name';
    const pollStart = new anchor.BN(0);
    const pollEnd = new anchor.BN(1829084142);

    const [pollAddress] = PublicKey.findProgramAddressSync([Buffer.from('poll'), new anchor.BN(1).toArrayLike(Buffer, 'le', 8)], votingAddress);

    await votingProgram.methods.initializePoll(pollId, name, description, pollStart, pollEnd).rpc();

    const pollData = await votingProgram.account.poll.fetch(pollAddress);

    console.log(pollData);

    expect(pollData.name).toEqual(name);
    expect(pollData.description).toEqual(description);
    expect(pollData.pollStart.toNumber()).toEqual(pollStart.toNumber());
    expect(pollData.pollEnd.toNumber()).toEqual(pollEnd.toNumber());
    expect(pollData.candidateAmount.toNumber()).toEqual(0);
    expect(pollData.pollId.toNumber()).toEqual(pollId.toNumber());
  });
});
