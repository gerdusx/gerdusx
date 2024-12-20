use trident_client::fuzzing::*;
use solana_sdk::native_token::LAMPORTS_PER_SOL;

use unchecked_arithmetic_0::trident_fuzz_initialize_snapshot::InitializeAlias;
use unchecked_arithmetic_0::trident_fuzz_update_snapshot::UpdateAlias;


/// Link the relevant Account Context Alias from the program.
/// Aliases are generated by the `AccountsSnapshots` macro.
type InitializeSnapshot<'info> = InitializeAlias<'info>;
type UpdateSnapshot<'info> = UpdateAlias<'info>;
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    Initialize(Initialize),
    Update(Update),
}
#[derive(Arbitrary, Debug)]
pub struct Initialize {
    pub accounts: InitializeAccounts,
    pub _data: InitializeData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeAccounts {
    pub user: AccountId,
    pub counter: AccountId,
    pub _system_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct InitializeData {}
#[derive(Arbitrary, Debug)]
pub struct Update {
    pub accounts: UpdateAccounts,
    pub data: UpdateData,
}
#[derive(Arbitrary, Debug)]
pub struct UpdateAccounts {
    pub counter: AccountId,
    pub authority: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct UpdateData {
    pub input1: u8,
    pub input2: u8,
}
///IxOps implementation for `Initialize` with all required functions.
impl<'info> IxOps<'info> for Initialize {
    type IxData = unchecked_arithmetic_0::instruction::Initialize;
    type IxAccounts = FuzzAccounts;
    type IxSnapshot = InitializeSnapshot<'info>;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        unchecked_arithmetic_0::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = unchecked_arithmetic_0::instruction::Initialize {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        
        let user = fuzz_accounts.user.get_or_create_account(
            self.accounts.user, 
            client, 
            5 * LAMPORTS_PER_SOL
        );

        let counter = fuzz_accounts.counter.get_or_create_account(
            self.accounts.counter, 
            client, 
            5 + LAMPORTS_PER_SOL
        );


        let acc_meta = unchecked_arithmetic_0::accounts::Initialize {
            user: user.pubkey(),
            counter: counter.pubkey(),
            system_program: solana_sdk::system_program::id(),
        }
        .to_account_metas(None);
        Ok((vec![user.clone(), counter.clone()], acc_meta))
    }
}
///IxOps implementation for `Update` with all required functions.
impl<'info> IxOps<'info> for Update {
    type IxData = unchecked_arithmetic_0::instruction::Update;
    type IxAccounts = FuzzAccounts;
    type IxSnapshot = UpdateSnapshot<'info>;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        unchecked_arithmetic_0::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = unchecked_arithmetic_0::instruction::Update {
            input1: self.data.input1,
            input2: self.data.input2,
        };
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        
        let user = fuzz_accounts.user.get_or_create_account(
            self.accounts.authority, 
            client,
            15 * LAMPORTS_PER_SOL
        );

        let counter = fuzz_accounts.counter.get_or_create_account(
            self.accounts.counter,
            client,
            5 + LAMPORTS_PER_SOL
        );

        let acc_meta = unchecked_arithmetic_0::accounts::Update {
            counter: counter.pubkey(),
            authority: user.pubkey(),
        }
        .to_account_metas(None);
        Ok((vec![user.clone()], acc_meta))
    }

    // Invariant Checks
    fn check(
        &self,
        pre_ix: Self::IxSnapshot,
        post_ix: Self::IxSnapshot,
        _ix_data: Self::IxData,
    ) -> Result<(), FuzzingError> {
        if let Some(counter) = post_ix.counter {
            if counter.count == 5 {
                return Err(FuzzingError::Custom(5));
            }
        }
        Ok(())
    }
    
}
/// Use AccountsStorage<T> where T can be one of:
/// Keypair, PdaStore, TokenStore, MintStore, ProgramStore
#[derive(Default)]
pub struct FuzzAccounts {
    // authority: AccountsStorage<todo!()>,
    counter: AccountsStorage<Keypair>,
    // system_program: AccountsStorage<todo!()>,
    user: AccountsStorage<Keypair>,
}
