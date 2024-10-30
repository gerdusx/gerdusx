use trident_client::fuzzing::*;
mod fuzz_instructions;
use fuzz_instructions::FuzzInstruction;
use unchecked_arithmetic_0::entry as entry_unchecked_arithmetic_0;
use unchecked_arithmetic_0::ID as PROGRAM_ID_UNCHECKED_ARITHMETIC_0;
const PROGRAM_NAME_UNCHECKED_ARITHMETIC_0: &str = "unchecked_arithmetic_0";
struct MyFuzzData;
/// Define instruction sequences for invocation.
/// `pre_ixs` runs at the start, `ixs` in the middle, and `post_ixs` at the end.
/// For example, to call `InitializeFn` at the start of each fuzzing iteration:
/// ```
/// fn pre_ixs(u: &mut arbitrary::Unstructured) ->
/// arbitrary::Result<Vec<FuzzInstruction>> {
///     let init = FuzzInstruction::InitializeFn(InitializeFn::arbitrary(u)?);
///     Ok(vec![init])
/// }
/// ```
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/instructions-sequences/#instructions-sequences
impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}
/// `fn fuzz_iteration` runs during every fuzzing iteration.
/// Modification is not required.
fn fuzz_iteration<T: FuzzTestExecutor<U> + std::fmt::Display, U>(
    fuzz_data: FuzzData<T, U>,
    config: &Config,
) {
    let fuzzing_program_unchecked_arithmetic_0 = FuzzingProgram::new(
        PROGRAM_NAME_UNCHECKED_ARITHMETIC_0,
        &PROGRAM_ID_UNCHECKED_ARITHMETIC_0,
        processor!(convert_entry!(entry_unchecked_arithmetic_0)),
    );
    let mut client =
        ProgramTestClientBlocking::new(&[fuzzing_program_unchecked_arithmetic_0], config).unwrap();
    let _ = fuzz_data.run_with_runtime(&mut client, config);
}
fn main() {
    let config = Config::new();
    fuzz_trident ! (fuzz_ix : FuzzInstruction , | fuzz_data : MyFuzzData | { fuzz_iteration (fuzz_data , & config) ; });
}
