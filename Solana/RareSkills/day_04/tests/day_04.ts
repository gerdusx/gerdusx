import * as anchor from '@coral-xyz/anchor';
import { AnchorError, Program } from '@coral-xyz/anchor';
import { Day04 } from '../target/types/day_04';
import { assert } from 'chai';

describe('day_04', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day04 as Program<Day04>;

  // it('limit range', async () => {
  //   try {
  //     const tx = await program.methods.limitRange(new anchor.BN(5)).accounts({}).rpc();
  //     console.log('Your transaction signature', tx);
  //   } catch (_err) {
  //     assert.isTrue(_err instanceof anchor.AnchorError);
  //     const err: AnchorError = _err;
  //     const errMsg = 'a is too small';
  //     assert.equal(err.error.errorMessage, errMsg);
  //     console.log('Error code', err.error.errorCode);
  //   }

  //   try {
  //     const tx = await program.methods.limitRange(new anchor.BN(101)).accounts({}).rpc();
  //     console.log('Your transaction signature', tx);
  //   } catch (_err) {
  //     assert.isTrue(_err instanceof anchor.AnchorError);
  //     const err: AnchorError = _err;
  //     const errMsg = 'a is too big';
  //     assert.equal(err.error.errorMessage, errMsg);
  //     console.log('Error code', err.error.errorCode);
  //   }
  // });

  it('always errors', async () => {
    try {
      const tx = await program.methods.alwaysErrors().accounts({}).rpc();
    } catch (_err) {
      assert.isTrue(_err instanceof anchor.AnchorError);
      const err: AnchorError = _err;
      const errMsg = 'Always errors';
      assert.equal(err.error.errorMessage, errMsg);
      console.log('Error code', err.error.errorCode);
    }
  });
});
