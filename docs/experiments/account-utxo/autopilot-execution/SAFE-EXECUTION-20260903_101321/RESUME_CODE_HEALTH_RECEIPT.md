# Resume Code-Health Transcript

**Status:** UNBOUND EXECUTOR-REPORTED OBSERVATION
**Run:** SAFE-EXECUTION-20260903_101321 resume
**Planning commit:** e4e808047a739c4bdc6deda3de86b2121a19dc52
**Execution branch:** automation/nondecisional-execution-20260903_101321

This file preserves an inline transcript attributed to the resume executor.
It is not a durable receipt: the run directory was untracked, and no receipt
identity, start/end timestamps, complete environment identity, or complete
toolchain identity was recorded. The transcript therefore cannot substantiate
a current or package-wide pass. At most it is an executor-reported code-health
observation; it is not Account-vs-UTXO state-model evidence.

## Missing binding fields

| Field | Status |
|---|---|
| receipt identity | `UNRESOLVED` |
| start timestamp with timezone | `UNRESOLVED` |
| end timestamp with timezone | `UNRESOLVED` |
| operating-system and execution-environment identity | `UNRESOLVED` |
| Rust compiler, Cargo, formatter, and Clippy identities | `UNRESOLVED` |
| repository/worktree content identity at execution | `UNRESOLVED` |
| durable external or content-addressed binding | `ABSENT` |

## cargo fmt

Command:

cargo fmt --all -- --check

Exit code:

0

stdout SHA-256:

E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855

stderr SHA-256:

E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855

### stdout

~~~text

~~~

### stderr

~~~text

~~~

## cargo test

Command:

cargo test --workspace --locked

Exit code:

0

stdout SHA-256:

5C3C9323C26300562D5E6D14970792DF8CB859767566D1500F228DF5EA21306B

stderr SHA-256:

1508C62455EB26742D0997A0159EB8DD10FF5F3331CD89237122C602C4129E45

### stdout

~~~text

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 45 tests
test bool::tests::decodes_canonical_values_and_consumes_one_byte ... ok
test bytes::tests::decodes_literal_vectors_and_leaves_trailing_bytes_untouched ... ok
test bool::tests::rejects_empty_input_without_advancing_cursor ... ok
test bool::tests::encodes_canonical_values ... ok
test bytes::tests::encodes_literal_vectors_byte_for_byte_at_exact_width ... ok
test bool::tests::rejects_every_non_canonical_byte_without_advancing_cursor ... ok
test bytes::tests::zero_length_decoding_succeeds_without_consuming_input ... ok
test bytes::tests::rejects_every_truncated_positive_width_without_advancing_cursor ... ok
test fixed_width::tests::decodes_u128_fixed_vectors_and_advances_input ... ok
test fixed_width::tests::decodes_u16_fixed_vectors_and_advances_input ... ok
test fixed_width::tests::decodes_u32_fixed_vectors_and_advances_input ... ok
test fixed_width::tests::decodes_u64_fixed_vectors_and_advances_input ... ok
test fixed_width::tests::decodes_u8_fixed_vectors_and_advances_input ... ok
test fixed_width::tests::encodes_u128_fixed_vectors_at_exact_width ... ok
test fixed_width::tests::encodes_u16_fixed_vectors_at_exact_width ... ok
test fixed_width::tests::encodes_u32_fixed_vectors_at_exact_width ... ok
test fixed_width::tests::encodes_u64_fixed_vectors_at_exact_width ... ok
test fixed_width::tests::encodes_u8_fixed_vectors_at_exact_width ... ok
test fixed_width::tests::rejects_every_truncated_u128_input_without_advancing ... ok
test fixed_width::tests::rejects_every_truncated_u16_input_without_advancing ... ok
test fixed_width::tests::rejects_every_truncated_u32_input_without_advancing ... ok
test fixed_width::tests::rejects_every_truncated_u64_input_without_advancing ... ok
test fixed_width::tests::rejects_every_truncated_u8_input_without_advancing ... ok
test option::tests::decodes_nested_options_and_leaves_trailing_bytes_untouched ... ok
test option::tests::decodes_none_and_leaves_trailing_bytes_untouched ... ok
test option::tests::decodes_some_values_and_leaves_trailing_bytes_untouched ... ok
test option::tests::encodes_nested_options_with_literal_canonical_vectors ... ok
test option::tests::encodes_none_without_invoking_nested_encoder ... ok
test option::tests::encodes_some_values_with_literal_canonical_vectors ... ok
test option::tests::propagates_nested_encoder_error_after_exactly_one_call ... ok
test option::tests::rejects_empty_input_without_advancing_cursor ... ok
test option::tests::rejects_every_invalid_tag_without_advancing_cursor ... ok
test option::tests::rolls_back_outer_cursor_for_every_nested_decoder_error ... ok
test u256::tests::decodes_fixed_vectors_and_advances_input ... ok
test u256::tests::encodes_fixed_vectors_at_exact_width ... ok
test u256::tests::explicit_little_endian_conversions_preserve_fixed_vectors ... ok
test u256::tests::rejects_every_truncated_input_without_advancing ... ok
test uleb128::tests::decodes_fixed_conformance_vectors_and_advances_input ... ok
test uleb128::tests::encodes_fixed_conformance_vectors ... ok
test uleb128::tests::rejects_non_canonical_encodings_without_advancing_input ... ok
test uleb128::tests::rejects_overflow_without_advancing_input ... ok
test uleb128::tests::rejects_truncated_encoding_without_advancing_input ... ok
test unix_timestamp::tests::decodes_fixed_vectors_and_leaves_trailing_bytes_untouched ... ok
test unix_timestamp::tests::encodes_fixed_vectors_at_exact_width ... ok
test unix_timestamp::tests::rejects_every_truncated_input_without_advancing_cursor ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


~~~

### stderr

~~~text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running unittests src\lib.rs (target\debug\deps\dilithia_consensus-54e2e7c5b54bfa33.exe)
     Running unittests src\lib.rs (target\debug\deps\dilithia_core-f8ced5d877f3430d.exe)
     Running unittests src\lib.rs (target\debug\deps\dilithia_crypto-57c4c0301ec6612b.exe)
     Running unittests src\lib.rs (target\debug\deps\dilithia_guard-49f765168089137e.exe)
     Running unittests src\main.rs (target\debug\deps\dilithia_node-3fafe6d10491e2a0.exe)
     Running unittests src\lib.rs (target\debug\deps\dilithia_p2p-486f78e2ed22af1f.exe)
     Running unittests src\lib.rs (target\debug\deps\dilithia_serialization-4af07fd4eac2b1f2.exe)
   Doc-tests dilithia_consensus
   Doc-tests dilithia_core
   Doc-tests dilithia_crypto
   Doc-tests dilithia_guard
   Doc-tests dilithia_p2p
   Doc-tests dilithia_serialization

~~~

## cargo clippy

Command:

cargo clippy --workspace --all-targets -- -D warnings

Exit code:

0

stdout SHA-256:

E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855

stderr SHA-256:

F0788AA36D1C86B6CCDFB7F4042A5574102D82061C046422A2D42E008A1E4710

### stdout

~~~text

~~~

### stderr

~~~text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s

~~~

## Result boundary

Historical command claims from the earlier execution run remain
executor-reported unless independently bound by durable artifacts.

The inline transcript does not supersede any historical claim. Re-execution
under `PACKAGE_WIDE_CHECK_AND_RECEIPT_SCHEMA.md`, with all applicable identity,
timestamp, environment, toolchain, input, and output fields bound, remains
pending.

Comparison scoring: NOT STARTED
State-model ranking: NONE
State-model decision: NOT MADE
Account selected: NO
UTXO selected: NO
Main merge: NOT DONE
